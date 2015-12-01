extern crate image;

use std::path::Path;

fn main() {
    let argv: Vec<_> = std::env::args().collect();

    if argv.len() < 3 {
        println!("Usage: {} <in-file> <out-file>", argv[0]);
        return;
    }

    let in_file = &Path::new(&argv[1]);
    let out_file = &Path::new(&argv[2]);

    let image = image::open(in_file).unwrap();

    let image = image.to_rgb();
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut image = image.into_raw();

    // Convert to an array of one tuple (u8, u8, u8) per pixel
    let pixels = unsafe {
        let raw = image.as_mut_ptr() as *mut (u8, u8, u8);

        std::slice::from_raw_parts_mut(raw, width * height)
    };

    glitch_hf(pixels, width, height);

    image::save_buffer(out_file,
                       &image,
                       width as u32,
                       height as u32,
                       image::ColorType::RGB(8)).unwrap();
}

fn glitch_sort(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {

    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset..offset + width];

        line.sort_by(|&a, &b| {
            a.0.cmp(&b.0)
        });
    }

}

fn glitch_filter(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {

    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset..offset + width];

        let filtered: Vec<_> = line.iter().filter_map(|&c| {
            let (r, g, b) = c;

            let r = r as u32;
            let g = g as u32;
            let b = b as u32;

            if g < r && g < b { //if r + g + b > 130 {
                Some (c)
            } else {
                None
            }
        }).collect();

        // line.sort_by(|&a, &b| {
        //     a.0.cmp(&b.0)
        // });

        for i in 0..line.len() {
            line[i] = (0xff, 0xff, 0xff);
        }

        let start = (line.len() - filtered.len()) / 2;

        for i in 0..filtered.len() {
            line[start + i] = filtered[i];
        }
    }

}

fn glitch_filter_sort(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {

    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset..offset + width];

        let mut filtered: Vec<_> = line.iter().filter_map(|&c| {
            let (r, g, b) = c;

            let r = r as u32;
            let g = g as u32;
            let b = b as u32;

            if g < r && g < b { //if r + g + b > 130 {
                Some (c)
            } else {
                None
            }
        }).collect();

        filtered.sort_by(|&a, &b| {
            a.0.cmp(&b.0)
        });

        for i in 0..line.len() {
            line[i] = (0xff, 0xff, 0xff);
        }

        let start = (line.len() - filtered.len()) / 2;

        for i in 0..filtered.len() {
            line[start + i] = filtered[i];
        }
    }

}

fn glitch_detour(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {

    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset..offset + width];

        let mut start = 0;

        for i in 0..line.len() {
            let (r, g, b) = line[i];

            if r < 245 || g < 245 || b < 245 {
                start = i;
                break;
            }
        }

        let mut end = line.len();

        for i in (0..line.len()).rev() {
            let (r, g, b) = line[i];

            if r < 245 || g < 245 || b < 245 {
                end = i;
                break;
            }
        }

        if start >= end {
            continue;
        }

        let detour = &mut line[start..end];

        detour.sort_by(|&a, &b| {
            let a = col_sum(a);
            let b = col_sum(b);
            a.cmp(&b)
        });

    }
}

fn glitch_block(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {

    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset..offset + width];

        let mut start = 0;

        for i in 0..line.len() {
            let (r, g, b) = line[i];

            if r < 245 || g < 245 || b < 245 {
                start = i;
                break;
            }
        }

        let mut end = line.len();

        for i in (0..line.len()).rev() {
            let (r, g, b) = line[i];

            if r < 245 || g < 245 || b < 245 {
                end = i;
                break;
            }
        }

        if start >= end {
            continue;
        }

        let detour = &mut line[start..end];

        for i in 0..detour.len() {
            detour[i] = detour[i & !0xff]
        }

        // detour.sort_by(|&a, &b| {
        //     let a = col_sum(a);
        //     let b = col_sum(b);
        //     a.cmp(&b)
        // });

    }
}

fn glitch_block_sin(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {

    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset..offset + width];

        let mut start = 0;

        for i in 0..line.len() {
            let (r, g, b) = line[i];

            if r < 245 || g < 245 || b < 245 {
                start = i;
                break;
            }
        }

        let mut end = line.len();

        for i in (0..line.len()).rev() {
            let (r, g, b) = line[i];

            if r < 245 || g < 245 || b < 245 {
                end = i;
                break;
            }
        }

        if start >= end {
            continue;
        }

        let detour = &mut line[start..end];

        let align = ((y as f32).sin() * 256.).abs() as usize + 1;

        for i in 0..detour.len() {

            let p = i - (i % align);

            detour[i] = detour[p]
        }

        // detour.sort_by(|&a, &b| {
        //     let a = col_sum(a);
        //     let b = col_sum(b);
        //     a.cmp(&b)
        // });

    }
}

fn glitch_hf(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {

    for y in 0..height {
        let offset = y * width;

        let diff = 100;

        let line = &mut pixels[offset..offset + width];

        let mut out = vec![(0u8, 0u8, 0u8); width];

        let edge = 100;

        for i in 0..diff {
            out[i] = line[i & !0xff];
        }

        for i in diff..width {
            out[i] = line[i & !0xff];
        }

        for i in diff..(line.len() - diff) {
            let a = col_sum(line[i - diff]) as i32;
            let b = col_sum(line[i]) as i32;
            let c = col_sum(line[i + diff]) as i32;

            if (a - b).abs() > edge && (b - c).abs() > edge {
                out[i] = line[i]
            } else {
                out[i] = line[i & !0xff]
            }
        }

        for i in 0..line.len() {
            line[i] = out[i];
        }
    }
}

fn glitch_hf_detour(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {

    for y in 0..height {
        let offset = y * width;

        let diff = 100;

        let line = &mut pixels[offset..offset + width];

        let mut start = 0;

        for i in 0..line.len() {
            let (r, g, b) = line[i];

            if r < 245 || g < 245 || b < 245 {
                start = i;
                break;
            }
        }

        let mut end = line.len();

        for i in (0..line.len()).rev() {
            let (r, g, b) = line[i];

            if r < 245 || g < 245 || b < 245 {
                end = i;
                break;
            }
        }

        if start >= end {
            continue;
        }

        if start < diff {
            start = 0;
        } else {
            start = start - diff;
        }

        if end + diff > width {
            end = width;
        } else {
            end = end + diff;
        }

        let mut out = vec![(0u8, 0u8, 0u8); end - start];

        {
            let detour = &mut line[start..end];

            let edge = 100;

            for i in diff..(detour.len() - diff) {
                let a = col_sum(detour[i - diff]) as i32;
                let b = col_sum(detour[i]) as i32;
                let c = col_sum(detour[i + diff]) as i32;

                if (a - b).abs() > edge && (b - c).abs() > edge {
                    out[i] = detour[i]
                } else {
                    out[i] = detour[i & !0xff]
                }
            }
        }

        for i in (start + diff)..(end - diff) {
             line[i] = out[i - start];
        }
    }
}

fn col_sum(c: (u8, u8, u8)) -> u16 {
    let (r, g, b) = c;

    r as u16 + g as u16 + b as u16
}

fn complement(c: (u8, u8, u8)) -> (u8, u8, u8) {
    let (r, g, b) = c;

    (!r, !g, !b)
}

fn rotate(c: (u8, u8, u8)) -> (u8, u8, u8) {
    let (r, g, b) = c;

    (g, b, r)
}
