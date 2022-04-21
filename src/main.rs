use image::{Rgb, RgbImage};
use ndarray::{Array, Array2};
use num::Complex;

fn main() {
    let height = 2000;
    let width = 2000;
    let div = 400.;

    let mut mandelbrot_image = get_back_image(width, height);

    for y in 0..height {
        for x in 0..width {
            let mut z: Complex<f64> = Complex { re: 0., im: 0. };
            let c = Complex {
                re: (x as f64 - 0.6 * width as f64) / div,
                im: (y as f64 - 0.5 * height as f64) / div,
            };
            let mut n = 0;
            while (z.norm_sqr() < 1000000000000.) & (n < 30) {
                z = z * z + c;
                n += 1;
            }
            if n == 30 {
                // Nothing
            } else {
                mandelbrot_image[[y, x]] = true;
            }
        }
    }
    let result_image = cvt_bool2rgb(&mandelbrot_image);
    result_image.save("./image/mandelbrot.jpg").unwrap();
}

fn get_back_image(width: usize, height: usize) -> Array2<bool> {
    let mut mask_array = vec![];
    for _ in 0..width {
        for _ in 0..height {
            mask_array.push(false);
        }
    }
    Array::from_shape_vec((height, width), mask_array).unwrap()
}

fn cvt_bool2rgb(bin_array: &Array2<bool>) -> RgbImage {
    let width = bin_array.shape()[1];
    let height = bin_array.shape()[0];
    let mut rgb_image = RgbImage::new(width as u32, height as u32);
    for x in 0..width as usize {
        for y in 0..height as usize {
            if bin_array[[y, x]] {
                rgb_image.put_pixel(x as u32, y as u32, Rgb([255, 255, 255]));
            }
        }
    }
    rgb_image
}
