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
const FB_HEIGHT: u32 = 600;
const FPS: u64 = 30;
const FRAME_MS: u64 = 1000 / FPS;
const MAX_SPARKLES: usize = 100;
const MIN_SPARKLES: usize = 50;

// 4x4 Bayer ordered-dither matrix, values 0..15 normalized to roughly [-0.5, 0.5].
// Used to scatter the RGB888 -> RGB565 quantization error so smooth gradients
// (the glow especially) don't show hard banding stair-steps.
const BAYER4: [[f32; 4]; 4] = [
    [0.0 / 16.0, 8.0 / 16.0, 2.0 / 16.0, 10.0 / 16.0],
    [12.0 / 16.0, 4.0 / 16.0, 14.0 / 16.0, 6.0 / 16.0],
    [3.0 / 16.0, 11.0 / 16.0, 1.0 / 16.0, 9.0 / 16.0],
    [15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0, 5.0 / 16.0],
];

// Quantize one 8-bit channel down to `bits` with a dither bias added before
// truncation. `bias` is the Bayer threshold for this pixel, scaled to the size
// of one quantization step so the error is spread evenly.
#[inline]
fn quantize_channel(value: u8, bits: u32, bias: f32) -> u16 {
    let levels = (1u16 << bits) - 1; // 31 for 5-bit, 63 for 6-bit
    let step = 255.0 / levels as f32; // size of one output step in 0..255 space
    // center the bias around 0 so it pushes some pixels up, some down
    let v = value as f32 + (bias - 0.5) * step;
    let v = v.clamp(0.0, 255.0);
    ((v / 255.0) * levels as f32 + 0.5) as u16 & levels
}

#[inline]
fn rgb_to_rgb565_dithered(r: u8, g: u8, b: u8, x: u32, y: u32) -> u16 {
    let bias = BAYER4[(y & 3) as usize][(x & 3) as usize];
    let r = quantize_channel(r, 5, bias);
    let g = quantize_channel(g, 6, bias);
    let b = quantize_channel(b, 5, bias);
    (r << 11) | (g << 5) | b
}

// Alpha-over blend of a straight-alpha foreground onto an opaque RGB background,
// all in full 8-bit precision. No RGB565 round-trip here anymore — compositing
// happens entirely in the RGB888 offscreen buffer, and 565 conversion is a single
// pass at the very end. That keeps anti-aliased logo edges crisp instead of
// losing precision every frame.
#[inline]
fn blend_over(fg_r: u8, fg_g: u8, fg_b: u8, fg_a: f32, bg: &mut [u8]) {
    bg[0] = (fg_r as f32 * fg_a + bg[0] as f32 * (1.0 - fg_a)) as u8;
    bg[1] = (fg_g as f32 * fg_a + bg[1] as f32 * (1.0 - fg_a)) as u8;
    bg[2] = (fg_b as f32 * fg_a + bg[2] as f32 * (1.0 - fg_a)) as u8;
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
        Sparkle {
            x: 0.0,
            y: 0.0,
            speed: 0.0,
            life: 0.0,
            active: false,
        }
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
    let logo = image::load_from_memory(LOGO_BYTES)
        .expect("logo")
        .into_rgba8();
    let glow = image::load_from_memory(GLOW_BYTES)
        .expect("glow")
        .into_rgba8();
    let (logo_w, logo_h) = logo.dimensions();

    // scale to 60% screen width
    let scale = if logo_w > FB_WIDTH * 6 / 10 {
        (FB_WIDTH * 6 / 10) as f32 / logo_w as f32
    } else {
        1.0f32
    };
    let scaled_w = (logo_w as f32 * scale) as u32;
    let scaled_h = (logo_h as f32 * scale) as u32;

    // Lanczos3: high-quality resampling with a windowed-sinc kernel. This is where
    // the pixelation was coming from — Nearest just grabbed the closest source pixel
    // with no blending, so every downscale got hard jaggies. Lanczos3 blends a
    // neighborhood and gives clean edges. Runs once at startup, so the cost is free.
    let logo = image::imageops::resize(
        &logo,
        scaled_w,
        scaled_h,
        image::imageops::FilterType::Lanczos3,
    );
    let glow = image::imageops::resize(
        &glow,
        scaled_w,
        scaled_h,
        image::imageops::FilterType::Lanczos3,
    );

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
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((*state >> 33) as f32) / (u32::MAX as f32)
    };

    // Two buffers now:
    //  - `composite` is RGB888 (3 bytes/px) where all drawing/blending happens in
    //    full precision.
    //  - `framebuf` is RGB565 (2 bytes/px), produced once per frame from `composite`
    //    via a single dithered conversion pass, then written to /dev/fb0.
    let mut composite = vec![0u8; (FB_WIDTH * FB_HEIGHT * 3) as usize];
    let mut framebuf = vec![0u8; (FB_WIDTH * FB_HEIGHT * 2) as usize];
    let mut fb = OpenOptions::new()
        .write(true)
        .open("/dev/fb0")
        .expect("fb0");

    let start = Instant::now();
    let mut _frame: u64 = 0;

    loop {
        let frame_start = Instant::now();
        let t = start.elapsed().as_secs_f32();

        // pulse: 0.9 + 0.2 * sin²(πt/2)
        let pulse = 0.9 + 0.2 * (std::f32::consts::PI * t / 2.0).sin().powi(2);
        let logo_alpha = pulse.min(1.0);
        let glow_alpha = (pulse - 1.0).max(0.0) * 5.0; // scale glow intensity

        // clear composite to black
        for b in composite.iter_mut() {
            *b = 0;
        }

        // draw glow layer (full-precision blend into RGB888 composite)
        if glow_alpha > 0.0 {
            for (x, y, pixel) in glow.enumerate_pixels() {
                let px = x_off + x;
                let py = y_off + y;
                if px < FB_WIDTH && py < FB_HEIGHT && pixel[3] > 0 {
                    let a = (pixel[3] as f32 / 255.0) * glow_alpha;
                    let idx = ((py * FB_WIDTH + px) * 3) as usize;
                    blend_over(
                        pixel[0],
                        pixel[1],
                        pixel[2],
                        a,
                        &mut composite[idx..idx + 3],
                    );
                }
            }
        }

        // draw logo (full-precision blend into RGB888 composite)
        for (x, y, pixel) in logo.enumerate_pixels() {
            let px = x_off + x;
            let py = y_off + y;
            if px < FB_WIDTH && py < FB_HEIGHT && pixel[3] > 0 {
                let a = (pixel[3] as f32 / 255.0) * logo_alpha;
                let idx = ((py * FB_WIDTH + px) * 3) as usize;
                blend_over(
                    pixel[0],
                    pixel[1],
                    pixel[2],
                    a,
                    &mut composite[idx..idx + 3],
                );
            }
        }

        // spawn sparkles to maintain 50-100
        let active_count = sparkles.iter().filter(|s| s.active).count();
        #[allow(clippy::collapsible_if)]
        if active_count < MIN_SPARKLES
            || (active_count < MAX_SPARKLES && lcg_rand(&mut rng_state) < 0.3)
        {
            if let Some(s) = sparkles.iter_mut().find(|s| !s.active)
                && !spawn_points.is_empty()
            {
                let idx = ((lcg_rand(&mut rng_state) * spawn_points.len() as f32) as usize)
                    .min(spawn_points.len() - 1);
                s.x = spawn_points[idx].0;
                s.y = spawn_points[idx].1;
                s.speed = if lcg_rand(&mut rng_state) < 0.5 {
                    15.0
                } else {
                    30.0
                };
                s.life = 1.0;
                s.active = true;
            }
        }

        // update and draw sparkles (full-precision blend into RGB888 composite)
        let dt = 1.0 / FPS as f32;
        for s in sparkles.iter_mut() {
            if !s.active {
                continue;
            }
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
                        let idx = ((py * FB_WIDTH + px) * 3) as usize;
                        blend_over(r, g, b, a, &mut composite[idx..idx + 3]);
                    }
                }
            }
        }

        // single dithered RGB888 -> RGB565 conversion pass for the whole frame
        for y in 0..FB_HEIGHT {
            for x in 0..FB_WIDTH {
                let cidx = ((y * FB_WIDTH + x) * 3) as usize;
                let color = rgb_to_rgb565_dithered(
                    composite[cidx],
                    composite[cidx + 1],
                    composite[cidx + 2],
                    x,
                    y,
                );
                let fidx = ((y * FB_WIDTH + x) * 2) as usize;
                let bytes = color.to_le_bytes();
                framebuf[fidx] = bytes[0];
                framebuf[fidx + 1] = bytes[1];
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

