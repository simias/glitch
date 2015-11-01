use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

const XRES: usize = 800;
const YRES: usize = 1200;

fn main() {
    let argv: Vec<_> = std::env::args().collect();

    if argv.len() < 3 {
        println!("Usage: {} <in-file> <out-file>", argv[0]);
        return;
    }

    let mut buffer = Box::new([[(0, 0, 0); XRES]; YRES]);

    let mut in_file = File::open(&argv[1]).unwrap();
    let mut out_file =
        OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&argv[2]).unwrap();

    for y in 0..YRES {
        for x in 0..XRES {
            let mut b = [0; 3];

            if in_file.read(&mut b).unwrap() != b.len() {
                panic!("Short read");
            }

            buffer[y][x] = (b[0], b[1], b[2]);
        }
    }

    for y in 0..YRES {
        let line: Vec<_> = 
            buffer[y].iter().filter_map(|&c| {
                let (r, g, b) = c;

                let r = r as u32;
                let g = g as u32;
                let b = b as u32;

                if r + g + b > 130 {
                    Some (c)
                } else {
                    None
                }
            }).collect();

        let start = (XRES - line.len()) / 2;

        buffer[y].sort_by(|&a, &b| {
            a.0.cmp(&b.0)
        });

        for i in 0..line.len() {
            buffer[y][start + i] = line[i];
        }
    }

    for y in 0..YRES {
        for x in 0..XRES {
            let col = buffer[y][x];
            let b = [col.0, col.1, col.2];

            out_file.write(&b).unwrap();
        }
    }
}
