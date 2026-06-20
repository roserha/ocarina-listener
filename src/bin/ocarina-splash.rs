use std::fs::OpenOptions;
use std::io::Write;

const LOGO_BYTES: &[u8] = include_bytes!("../../imgs/OcarinaOS.png");

fn rgb_to_rgb565(r: u8, g: u8, b: u8) -> u16 {
    let r = (r as u16 >> 3) & 0x1F;
    let g = (g as u16 >> 2) & 0x3F;
    let b = (b as u16 >> 3) & 0x1F;
    (r << 11) | (g << 5) | b
}

fn main() {
    let fb_width: u32 = 1024;
    let fb_height: u32 = 768;

    // hide cursor
    let _ = std::process::Command::new("sh")
        .arg("-c")
        .arg("echo 0 > /sys/class/graphics/fbcon/cursor_blink 2>/dev/null; printf '\\033[?25l' > /dev/tty1 2>/dev/null")
        .status();

    let black = rgb_to_rgb565(0, 0, 0);
    let black_bytes = black.to_le_bytes();

    let mut framebuf = vec![0u8; (fb_width * fb_height * 2) as usize];
    for pixel in framebuf.chunks_mut(2) {
        pixel[0] = black_bytes[0];
        pixel[1] = black_bytes[1];
    }

    // load logo
    let logo = image::load_from_memory(LOGO_BYTES).expect("failed to load logo");
    let logo = logo.into_rgba8();
    let (logo_w, logo_h) = logo.dimensions();

    // scale to 60% of screen width
    let scale = if logo_w > fb_width * 6 / 10 {
        (fb_width * 6 / 10) as f32 / logo_w as f32
    } else {
        1.0f32
    };

    let scaled_w = (logo_w as f32 * scale) as u32;
    let scaled_h = (logo_h as f32 * scale) as u32;

    let logo = image::imageops::resize(&logo, scaled_w, scaled_h, image::imageops::FilterType::Lanczos3);

    // center
    let x_off = (fb_width - scaled_w) / 2;
    let y_off = (fb_height - scaled_h) / 2;

    // blit with alpha blending
    for (x, y, pixel) in logo.enumerate_pixels() {
        let px = x_off + x;
        let py = y_off + y;
        if px < fb_width && py < fb_height {
            let alpha = pixel[3] as f32 / 255.0;
            let r = (pixel[0] as f32 * alpha * (1.0 - alpha)) as u8;
            let g = (pixel[1] as f32 * alpha) as u8;
            let b = (pixel[2] as f32 * alpha * (1.0 - alpha)) as u8;
            let color = rgb_to_rgb565(r, g, b);
            let bytes = color.to_le_bytes();
            let idx = ((py * fb_width + px) * 2) as usize;
            framebuf[idx] = bytes[0];
            framebuf[idx + 1] = bytes[1];
        }
    }

    let mut fb = OpenOptions::new()
        .write(true)
        .open("/dev/fb0")
        .expect("failed to open /dev/fb0");

    fb.write_all(&framebuf).expect("failed to write to framebuffer");

    // stay alive until killed
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}