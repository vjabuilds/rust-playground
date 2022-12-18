use num::Complex;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex{re:0.0, im:0.0};
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn pixel_to_point(bounds: (usize, usize), pixel: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) -> Complex<f64>{
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

fn plot_madelbrot(pixels: &mut[u8], bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row*bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8
            };
        }
    }
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32,ColorType::Gray(8))?;
    Ok(())
}

fn main() {
    let bounds = (3000, 3000);
    let upper_left = Complex{re:-1.0, im:-1.0};
    let lower_right = Complex{re:1.0, im:1.0};
    let mut pixels = vec![0; bounds.0 * bounds.1];
    plot_madelbrot(&mut pixels, bounds ,upper_left, lower_right);
    write_image("test.png", &pixels, bounds).expect("error writing file!");

}
