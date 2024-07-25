extern crate image;

use std::str::FromStr;
use std::path::Path;
use color::{Color, HdrColor};

mod color;

fn main() {
    let argv: Vec<_> = std::env::args().collect();

    if argv.len() < 4 {
        println!("Usage: {} <in-file> <out-file> <lim> [debug]", argv[0]);
        return;
    }

    let in_file = &Path::new(&argv[1]);
    let out_file = &Path::new(&argv[2]);

    let image = image::open(in_file).unwrap();

    let image = image.into_rgb8();
    let width = image.width() as usize;
    let height = image.height() as usize;
    let mut image = image.into_raw();

    // Convert to an array of one tuple (u8, u8, u8) per pixel
    let pixels = unsafe {
        let raw = image.as_mut_ptr() as *mut Color;

        std::slice::from_raw_parts_mut(raw, width * height)
    };

    let mut hdr_pixels: Vec<HdrColor> = pixels.iter().map(|&p| p.into()).collect();

    //let stripw = width / 3;

    // glitch_filter_sort(pixels, width, height, 0, stripw);
    // glitch_block(pixels, width, height, stripw, stripw * 2);
    // glitch_block_sin(pixels, width, height, stripw * 2, width);

    let lim = f32::from_str(&argv[3]).unwrap() / 100.;

    let edges: Vec<_> = sobel(&hdr_pixels, width, height).iter()
        .map(|c| c.avg() > lim).collect();

    pixel_sort(&mut hdr_pixels, &edges, width, height);

    for i in 0..hdr_pixels.len() {
        pixels[i] = hdr_pixels[i].into();
    }

    if argv.len() > 4 {
        for i in 0..pixels.len() {
            if edges[i] {
                pixels[i] = Color(255, 0, 0)
            }
        }
    }

    image::save_buffer(out_file,
                       &image,
                       width as u32,
                       height as u32,
                       image::ColorType::Rgb8).unwrap();
}

fn sobel(pixels: &[HdrColor], width: usize, height: usize) -> Vec<HdrColor> {
    let mut out = vec![HdrColor::black(); width * height];

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut cx = HdrColor::black();

            cx = cx + pixels[(y - 1) * width + (x - 1)] * -1.;
            cx = cx + pixels[(y + 0) * width + (x - 1)] * -2.;
            cx = cx + pixels[(y + 1) * width + (x - 1)] * -1.;

            cx = cx + pixels[(y - 1) * width + (x + 1)] * 1.;
            cx = cx + pixels[(y + 0) * width + (x + 1)] * 2.;
            cx = cx + pixels[(y + 1) * width + (x + 1)] * 1.;

            let mut cy = HdrColor::black();

            cy = cy + pixels[(y - 1) * width + (x - 1)] * -1.;
            cy = cy + pixels[(y - 1) * width + (x + 0)] * -2.;
            cy = cy + pixels[(y - 1) * width + (x + 1)] * -1.;

            cy = cy + pixels[(y + 1) * width + (x - 1)] * 1.;
            cy = cy + pixels[(y + 1) * width + (x + 0)] * 2.;
            cy = cy + pixels[(y + 1) * width + (x + 1)] * 1.;

            out[y * width + x] = (cx * cx + cy * cy).sqrt();
        }
    }

    out
}

fn pixel_sort(pixels: &mut[HdrColor],
              edges: &[bool],
              width: usize,
              height: usize) {

    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset..offset + width];
        let edges = &edges[offset..offset + width];

        let mut start = 0;
        let mut started = false;

        for x in 0..width {
            if started {
                if edges[x] {
                    line[start..x].sort();
                    started = false;
                }
            } else {
                start = x;
                started = !edges[x];
            }
        }

        line[start..width].sort();
    }
}

/*
fn glitch_sort(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {

    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset..offset + width];

        line.sort_by(|&a, &b| {
            col_sum(a).cmp(&col_sum(b))
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

#[inline(never)]
fn glitch_filter_sort(pixels: &mut[(u8, u8, u8)], width: usize, height: usize, start: usize, stop: usize) {

    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset + start .. offset + stop];

        let mut filtered: Vec<_> = line.iter().filter_map(|&c| {
            let (r, g, b) = c;

            let r = r as u32;
            let g = g as u32;
            let b = b as u32;

            if r + g + b > 256 {
                Some (c)
            } else {
                None
            }
        }).collect();

        filtered.sort_by(|&a, &b| {
            a.0.cmp(&b.0)
        });

        line.sort_by(|&a, &b| {
            a.0.cmp(&b.0)
        });

        // for i in 0..line.len() {
        //     line[i] = (0xff, 0xff, 0xff);
        // }

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

            if r < 128 || g < 128 || b < 128 {
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

fn glitch_block(pixels: &mut[(u8, u8, u8)], width: usize, height: usize, start: usize, stop: usize) {

    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset + start..offset + stop];

        let mut start = 0;

        let lim = 150;

        for i in 0..line.len() {
            let (r, g, b) = line[i];

            if r < lim || g < lim || b < lim {
                start = i;
                break;
            }
        }

        let mut end = line.len();

        for i in (0..line.len()).rev() {
            let (r, g, b) = line[i];

            if r < lim || g < lim || b < lim {
                end = i;
                break;
            }
        }

        if start >= end {
            continue;
        }

        let detour = &mut line[start..end];

        for i in 0..detour.len() {
            detour[i] = detour[i & !0xf]
        }

        // detour.sort_by(|&a, &b| {
        //     let a = col_sum(a);
        //     let b = col_sum(b);
        //     a.cmp(&b)
        // });

    }
}

fn glitch_block_sin(pixels: &mut[(u8, u8, u8)], width: usize, height: usize, start: usize, stop: usize) {

    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset + start ..offset + stop];

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

        let align = ((y as f32).sin() * 20.).abs() as usize + 1;

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

        let diff = 50;

        let line = &mut pixels[offset..offset + width];

        let mut out = vec![(0xffu8, 0xffu8, 0xffu8); width];

        let edge = 30;

        // for i in 0..diff {
        //     out[i] = line[i & !0xff];
        // }

        // for i in diff..width {
        //     out[i] = line[i & !0xff];
        // }

        for i in 0..line.len() {
            let a = col_sum(line[if i > diff { i - diff} else {0}]) as i32;
            let b = col_sum(line[i]) as i32;
            let c = col_sum(line[if i + diff >= width { width - 1 } else { i + diff }]) as i32;

            if (a - b).abs() > edge && (b - c).abs() > edge {
                out[i] = line[i]
            } else {
                out[i] = line[i - (i % 71)]
            }
        }

        for i in 0..line.len() {
            line[i] = out[i];
        }
    }
}

fn glitch_hf_edge(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {

    let mut start = 0;

    {
        let line = &mut pixels[0..width];

        let mut max_diff = 0;

        for i in 1..line.len() {
            let diff = col_diff(line[i - 1], line[i]);
            if diff > max_diff {
                start = i;
                max_diff = diff;
            }
        }
    }

    for y in 0..height {
        let offset = y * width;

        let diff = 50;

        let line = &mut pixels[offset..offset + width];

        let mut out = vec![(0xffu8, 0xffu8, 0xffu8); width];

        let edge = 30;

        for i in 1..line.len() {
            if col_diff(line[i - 1], line[i]) > 50 && (start as i32 - i as i32).abs() < 10 {
                start = i;
                break;
            }
        }

        let off = start % 71;

        for i in 0..line.len() {
            let a = col_sum(line[if i > diff { i - diff} else {0}]) as i32;
            let b = col_sum(line[i]) as i32;
            let c = col_sum(line[if i + diff >= width { width - 1 } else { i + diff }]) as i32;

            if (a - b).abs() > edge && (b - c).abs() > edge {
                out[i] = line[i]
            } else {
                if i < start {
                    let pos = i - i % 71;

                    if pos > off {
                        out[i] = line[pos + off];
                    } else{
                        out[i] = line[0];
                    }
                } else {
                    out[i] = line[start + ((i - start) - (i - start) % 71)]
                }
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

fn glitch_swap(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {

    for y in 0..height {
        for x in 0..width {
            let pixel = pixels[y * width + x];

            if col_sum(pixel) < 200 {
                let new_x = (x + 100) % width;
                let new_y = (y + 100) % height;

                pixels[y * width + x] = pixels[(new_y * width + new_x) ];
            }
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

fn col_diff(a: (u8, u8, u8), b: (u8, u8, u8)) -> u16 {
    let (ra, ga, ba) = a;
    let (rb, gb, bb) = b;

    let ra = ra as i32;
    let ga = ga as i32;
    let ba = ba as i32;

    let rb = rb as i32;
    let gb = gb as i32;
    let bb = bb as i32;

    ((ra - rb).abs() + (ga - gb).abs() + (ba - bb).abs()) as u16
}
*/
