use image::GenericImageView;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;

fn main() {
    // open framebuffer
    let mut fb = OpenOptions::new()
        .write(true)
        .open("/dev/fb0")
        .expect("failed to open /dev/fb0");

    // get screen size via ioctl
    let (fb_width, fb_height) = get_fb_size();
    
    // fill purple (R=51, G=0, B=102 for 0.2, 0.0, 0.4 in 8bit)
    let purple: u32 = 0xFF330066; // ARGB
    let mut framebuf = vec![0u8; (fb_width * fb_height * 4) as usize];
    for pixel in framebuf.chunks_mut(4) {
        pixel[0] = 0x66; // B
        pixel[1] = 0x00; // G
        pixel[2] = 0x33; // R
        pixel[3] = 0xFF; // A
    }

    // load logo
    const LOGO_BYTES: &[u8] = include_bytes!("../../imgs/OcarinaOS.png");

    // then load with:
    let logo = image::load_from_memory(LOGO_BYTES).expect("failed to load logo");
    let (logo_w, logo_h) = logo.dimensions();
    
    // scale if needed
    let scale = if logo_w > fb_width * 6 / 10 {
        (fb_width * 6 / 10) as f32 / logo_w as f32
    } else {
        1.0f32
    };
    
    let scaled_w = (logo_w as f32 * scale) as u32;
    let scaled_h = (logo_h as f32 * scale) as u32;
    let logo = logo.resize(scaled_w, scaled_h, image::imageops::FilterType::Lanczos3);
    
    // center it
    let x_off = (fb_width - scaled_w) / 2;
    let y_off = (fb_height - scaled_h) / 2;
    
    // blit logo onto framebuf
    for (x, y, pixel) in logo.pixels() {
        let px = x_off + x;
        let py = y_off + y;
        if px < fb_width && py < fb_height {
            let idx = ((py * fb_width + px) * 4) as usize;
            let alpha = pixel[3] as f32 / 255.0;
            framebuf[idx]     = (pixel[2] as f32 * alpha + 0x33 as f32 * (1.0 - alpha)) as u8; // B
            framebuf[idx + 1] = (pixel[1] as f32 * alpha) as u8; // G
            framebuf[idx + 2] = (pixel[0] as f32 * alpha + 0x66 as f32 * (1.0 - alpha)) as u8; // R  
            framebuf[idx + 3] = 0xFF;
        }
    }
    
    fb.write_all(&framebuf).expect("failed to write to framebuffer");
    
    // sleep until killed
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}

fn get_fb_size() -> (u32, u32) {
    // try to read from /sys first
    let width = std::fs::read_to_string("/sys/class/graphics/fb0/virtual_size")
        .ok()
        .and_then(|s| {
            let parts: Vec<&str> = s.trim().split(',').collect();
            parts[0].parse::<u32>().ok()
        })
        .unwrap_or(1920);
    
    let height = std::fs::read_to_string("/sys/class/graphics/fb0/virtual_size")
        .ok()
        .and_then(|s| {
            let parts: Vec<&str> = s.trim().split(',').collect();
            parts[1].parse::<u32>().ok()
        })
        .unwrap_or(1080);
    
    (width, height)
}