extern crate crossbeam;
extern crate image;
extern crate num;

use image::png::PngEncoder;
use image::ColorType;
use num::Complex;
use std::fs::File;
use std::io::Result;

pub fn main_single(bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    let mut pixels = vec![0; bounds.0 * bounds.1];
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    for (i, band) in bands.into_iter().enumerate() {
        let top = rows_per_band * i;
        let height = band.len() / bounds.0;
        let band_bounds = (bounds.0, height);
        let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
        let band_lower_right =
            pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

        render(band, band_bounds, band_upper_left, band_lower_right);
    }

    write_image("mandelbrot-single.png", &pixels, bounds).expect("error writing PNG file");
}

pub fn main_multi(bounds: (usize, usize), upper_left: Complex<f64>, lower_right: Complex<f64>) {
    let mut pixels = vec![0; bounds.0 * bounds.1];
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * bounds.0).collect();

    let result = crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / bounds.0;
            let band_bounds = (bounds.0, height);
            let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
            let band_lower_right =
                pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

            spawner.spawn(move |_| {
                render(band, band_bounds, band_upper_left, band_lower_right);
            });
        }
    });

    if result.is_err() {
        println!("{:?}", result);
        panic!("error in crossbeam scope");
    }

    write_image("mandelbrot-multi.png", &pixels, bounds).expect("error writing PNG file");
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<()> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    let result = encoder.encode(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::L8);
    if result.is_err() {
        println!("{:?}", result);
        panic!("error encoding PNG file");
    }

    Ok(())
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            }
        }
    }
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}
