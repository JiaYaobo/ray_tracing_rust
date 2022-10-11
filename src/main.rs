mod vec;
use std::io::{stderr, Write};
use vec::{Vec3, Color};


fn main() {
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            // let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            // let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            // let b = 0.5;
            //
            //
            // let ir = (255.999 * r) as u64;
            // let ig: u64 = (255.999 * g) as u64;
            // let ib: u64 = (255.999 * b) as u64;
            //
            // println!("{} {} {}", ir, ig, ib);
            let pixel_color = Color::new((i as f64) / ((IMAGE_WIDTH - 1) as f64),
                                         (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                                         0.25);

            println!("{}", pixel_color.format_color());

        }
    }
    eprintln!("Done.");
}
