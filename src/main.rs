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

    glitch(pixels, width, height);

    image::save_buffer(out_file,
                       &image,
                       width as u32,
                       height as u32,
                       image::ColorType::RGB(8)).unwrap();

    // for y in 0..YRES {
    //     for x in 0..XRES {
    //         let mut b = [0; 3];

    //         if in_file.read(&mut b).unwrap() != b.len() {
    //             panic!("Short read");
    //         }

    //         buffer[y][x] = (b[0], b[1], b[2]);
    //     }
    // }

    // for y in 0..YRES {
    //     // let line: Vec<_> =
    //     //     buffer[y].iter().filter_map(|&c| {
    //     //         let (r, g, b) = c;

    //     //         let r = r as u32;
    //     //         let g = g as u32;
    //     //         let b = b as u32;

    //     //         if r + g + b > 130 {
    //     //             Some (c)
    //     //         } else {
    //     //             None
    //     //         }
    //     //     }).collect();

    //     // let start = (XRES - line.len()) / 2;

    //     buffer[y].sort_by(|&a, &b| {
    //         a.0.cmp(&b.0)
    //     });

    //     // for i in 0..line.len() {
    //     //     buffer[y][start + i] = line[i];
    //     // }
    // }

    // for y in 0..YRES {
    //     for x in 0..XRES {
    //         let col = buffer[y][x];
    //         let b = [col.0, col.1, col.2];

    //         out_file.write(&b).unwrap();
    //     }
    // }
}

fn glitch(pixels: &mut[(u8, u8, u8)], width: usize, height: usize) {
    
    for y in 0..height {
        let offset = y * width;

        let line = &mut pixels[offset..offset + width];

        line.sort_by(|&a, &b| {
            a.0.cmp(&b.0)
        });
    }

}
