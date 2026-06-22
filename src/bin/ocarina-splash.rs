// This is the only code I didn't write myself, and relied on LLMs.
// I personally advocate for knowledge of the code I write, but I
// don't think that a splash screen that shows up for a few seconds
// before boot was essential. I might come back to this in the future,
// but as it currently stands, I didn't hold this as a vital part of the
// project, unlike the actual GUI interface or the OcarinaOS itself.

use std::fs::OpenOptions;
use std::io::{Seek, SeekFrom, Write};
use std::time::{Duration, Instant};

const LOGO_BYTES: &[u8] = include_bytes!("../../imgs/OcarinaOS.png");
const GLOW_BYTES: &[u8] = include_bytes!("../../imgs/OcarinaOS-Blur.png");

const FB_WIDTH: u32 = 1024;
const FB_HEIGHT: u32 = 768;
const FPS: u64 = 30;
const FRAME_MS: u64 = 1000 / FPS;
const MAX_SPARKLES: usize = 100;
const MIN_SPARKLES: usize = 50;

fn rgb_to_rgb565(r: u8, g: u8, b: u8) -> u16 {
    let r = (r as u16 >> 3) & 0x1F;
    let g = (g as u16 >> 2) & 0x3F;
    let b = (b as u16 >> 3) & 0x1F;
    (r << 11) | (g << 5) | b
}

fn blend(fg_r: u8, fg_g: u8, fg_b: u8, fg_a: f32, bg_r: u8, bg_g: u8, bg_b: u8) -> (u8, u8, u8) {
    let r = (fg_r as f32 * fg_a + bg_r as f32 * (1.0 - fg_a)) as u8;
    let g = (fg_g as f32 * fg_a + bg_g as f32 * (1.0 - fg_a)) as u8;
    let b = (fg_b as f32 * fg_a + bg_b as f32 * (1.0 - fg_a)) as u8;
    (r, g, b)
}

struct Sparkle {
    x: f32,
    y: f32,
    speed: f32,
    life: f32,
    active: bool,
}

impl Sparkle {
    fn new() -> Self {
        Sparkle { x: 0.0, y: 0.0, speed: 0.0, life: 0.0, active: false }
    }
}

fn main() {
    // wait for fb0
    let mut attempts = 0;
    while !std::path::Path::new("/dev/fb0").exists() && attempts < 50 {
        std::thread::sleep(Duration::from_millis(100));
        attempts += 1;
    }
    if !std::path::Path::new("/dev/fb0").exists() {
        return;
    }

    // hide TTY
    let _ = std::fs::write("/sys/class/vtconsole/vtcon0/bind", "0");
    let _ = std::fs::write("/sys/class/vtconsole/vtcon1/bind", "0");

    // load images
    let logo = image::load_from_memory(LOGO_BYTES).expect("logo").into_rgba8();
    let glow = image::load_from_memory(GLOW_BYTES).expect("glow").into_rgba8();
    let (logo_w, logo_h) = logo.dimensions();

    // scale to 60% screen width
    let scale = if logo_w > FB_WIDTH * 6 / 10 {
        (FB_WIDTH * 6 / 10) as f32 / logo_w as f32
    } else {
        1.0f32
    };
    let scaled_w = (logo_w as f32 * scale) as u32;
    let scaled_h = (logo_h as f32 * scale) as u32;

    let logo = image::imageops::resize(&logo, scaled_w, scaled_h, image::imageops::FilterType::Nearest);
    let glow = image::imageops::resize(&glow, scaled_w, scaled_h, image::imageops::FilterType::Nearest);

    let x_off = (FB_WIDTH - scaled_w) / 2;
    let y_off = (FB_HEIGHT - scaled_h) / 2;

    // pre-compute valid sparkle spawn points (non-transparent logo pixels)
    let mut spawn_points: Vec<(f32, f32)> = Vec::new();
    for (x, y, pixel) in logo.enumerate_pixels() {
        if pixel[3] > 30 {
            spawn_points.push((x_off as f32 + x as f32, y_off as f32 + y as f32));
        }
    }

    // sparkles
    let mut sparkles: Vec<Sparkle> = (0..MAX_SPARKLES).map(|_| Sparkle::new()).collect();
    let mut rng_state: u64 = 12345;

    let lcg_rand = |state: &mut u64| -> f32 {
        *state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((*state >> 33) as f32) / (u32::MAX as f32)
    };

    // framebuffer
    let mut framebuf = vec![0u8; (FB_WIDTH * FB_HEIGHT * 2) as usize];
    let mut fb = OpenOptions::new().write(true).open("/dev/fb0").expect("fb0");

    let start = Instant::now();
    let mut _frame: u64 = 0;

    loop {
        let frame_start = Instant::now();
        let t = start.elapsed().as_secs_f32();

        // pulse: 0.9 + 0.2 * sin²(πt/2)
        let pulse = 0.9 + 0.2 * (std::f32::consts::PI * t / 2.0).sin().powi(2);
        let logo_alpha = pulse.min(1.0);
        let glow_alpha = (pulse - 1.0).max(0.0) * 5.0; // scale glow intensity

        // clear to black
        for pixel in framebuf.chunks_mut(2) {
            pixel[0] = 0;
            pixel[1] = 0;
        }

        // draw glow layer
        if glow_alpha > 0.0 {
            for (x, y, pixel) in glow.enumerate_pixels() {
                let px = x_off + x;
                let py = y_off + y;
                if px < FB_WIDTH && py < FB_HEIGHT && pixel[3] > 0 {
                    let a = (pixel[3] as f32 / 255.0) * glow_alpha;
                    let idx = ((py * FB_WIDTH + px) * 2) as usize;
                    let existing = u16::from_le_bytes([framebuf[idx], framebuf[idx+1]]);
                    let er = ((existing >> 11) & 0x1F) as u8;
                    let eg = ((existing >> 5) & 0x3F) as u8;
                    let eb = (existing & 0x1F) as u8;
                    let (r, g, b) = blend(pixel[0], pixel[1], pixel[2], a, er << 3, eg << 2, eb << 3);
                    let color = rgb_to_rgb565(r, g, b);
                    let bytes = color.to_le_bytes();
                    framebuf[idx] = bytes[0];
                    framebuf[idx+1] = bytes[1];
                }
            }
        }

        // draw logo
        for (x, y, pixel) in logo.enumerate_pixels() {
            let px = x_off + x;
            let py = y_off + y;
            if px < FB_WIDTH && py < FB_HEIGHT && pixel[3] > 0 {
                let a = (pixel[3] as f32 / 255.0) * logo_alpha;
                let idx = ((py * FB_WIDTH + px) * 2) as usize;
                let existing = u16::from_le_bytes([framebuf[idx], framebuf[idx+1]]);
                let er = ((existing >> 11) & 0x1F) as u8;
                let eg = ((existing >> 5) & 0x3F) as u8;
                let eb = (existing & 0x1F) as u8;
                let (r, g, b) = blend(pixel[0], pixel[1], pixel[2], a, er << 3, eg << 2, eb << 3);
                let color = rgb_to_rgb565(r, g, b);
                let bytes = color.to_le_bytes();
                framebuf[idx] = bytes[0];
                framebuf[idx+1] = bytes[1];
            }
        }

        // spawn sparkles to maintain 50-100
        let active_count = sparkles.iter().filter(|s| s.active).count();
        if active_count < MIN_SPARKLES || (active_count < MAX_SPARKLES && lcg_rand(&mut rng_state) < 0.3) {
            if let Some(s) = sparkles.iter_mut().find(|s| !s.active) {
                if !spawn_points.is_empty() {
                    let idx = (lcg_rand(&mut rng_state) * spawn_points.len() as f32) as usize;
                    let idx = idx.min(spawn_points.len() - 1);
                    s.x = spawn_points[idx].0;
                    s.y = spawn_points[idx].1;
                    s.speed = if lcg_rand(&mut rng_state) < 0.5 { 15.0 } else { 30.0 };
                    s.life = 1.0;
                    s.active = true;
                }
            }
        }

        // update and draw sparkles
        let dt = 1.0 / FPS as f32;
        for s in sparkles.iter_mut() {
            if !s.active { continue; }
            s.y -= s.speed * dt;
            s.life -= dt * 0.5; // fade over 2 seconds
            if s.life <= 0.0 || s.y < 0.0 {
                s.active = false;
                continue;
            }
            // golden sparkle: RGB(255, 200, 50)
            let a = s.life;
            let (r, g, b) = (255u8, 200u8, 50u8);
            // draw 2x2
            for dy in 0..2i32 {
                for dx in 0..2i32 {
                    let px = (s.x as i32 + dx) as u32;
                    let py = (s.y as i32 + dy) as u32;
                    if px < FB_WIDTH && py < FB_HEIGHT {
                        let idx = ((py * FB_WIDTH + px) * 2) as usize;
                        let existing = u16::from_le_bytes([framebuf[idx], framebuf[idx+1]]);
                        let er = ((existing >> 11) & 0x1F) as u8;
                        let eg = ((existing >> 5) & 0x3F) as u8;
                        let eb = (existing & 0x1F) as u8;
                        let (br, bg, bb) = blend(r, g, b, a, er << 3, eg << 2, eb << 3);
                        let color = rgb_to_rgb565(br, bg, bb);
                        let bytes = color.to_le_bytes();
                        framebuf[idx] = bytes[0];
                        framebuf[idx+1] = bytes[1];
                    }
                }
            }
        }

        // write framebuffer
        fb.seek(SeekFrom::Start(0)).ok();
        fb.write_all(&framebuf).ok();

        _frame += 1;
        let elapsed = frame_start.elapsed().as_millis() as u64;
        if elapsed < FRAME_MS {
            std::thread::sleep(Duration::from_millis(FRAME_MS - elapsed));
        }
    }
}