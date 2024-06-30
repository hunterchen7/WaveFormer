use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
use num::integer::Roots;

type SobelPoint = (i32, i32);

const SOBEL_X: [[i32; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
const SOBEL_Y: [[i32; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

#[allow(dead_code)]
pub fn sobel_threshold(img: &DynamicImage, threshold: u8, use_g: bool) -> DynamicImage {
    let mut new_img = img.clone();

    for x in 0..img.width() {
        for y in 0..img.height() {
            let g = edge_magnitude(gx_gy(img, x, y)) as u8;
            if g >= threshold && x > 0 && y > 0 {
                // threshold for white
                if use_g {
                    new_img.put_pixel(x, y, image::Rgba([g, g, g, 255]));
                } else {
                    new_img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
                }
            } else {
                new_img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
            }
        }
    }
    new_img
}

#[allow(dead_code)]
pub fn sobel(img: &DynamicImage) -> DynamicImage {
    sobel_threshold(img, 0, true)
}

#[allow(dead_code)]
pub fn sobel_default(img: &DynamicImage) -> DynamicImage {
    sobel_threshold(img, 128, false)
}

pub fn gx_gy(img: &DynamicImage, x: u32, y: u32) -> SobelPoint {
    let mut gx = 0;
    let mut gy = 0;

    for i in 0..3 {
        for j in 0..3 {
            let new_x = ((x as i32 + i - 1).max(0) as u32).min(img.width() - 1);
            let new_y = ((y as i32 + j - 1).max(0) as u32).min(img.height() - 1);

            let pixel = img.get_pixel(new_x, new_y)[0] as i32;
            gx += SOBEL_X[i as usize][j as usize] * pixel;
            gy += SOBEL_Y[i as usize][j as usize] * pixel;
        }
    }

    (gx, gy)
}

pub fn edge_direction((gx, gy): (i32, i32)) -> f64 {
    (gy as f64).atan2(gx as f64).to_degrees()
}

pub fn edge_magnitude((gx, gy): (i32, i32)) -> f64 {
    ((gx.pow(2) + gy.pow(2)) as f64).sqrt()
}

// calculate intensity gradient of every pixel
pub fn intensity_gradient(img: &DynamicImage) -> Vec<Vec<(f64, f64)>> {
    (0..img.width())
        .map(|x| {
            (0..img.height())
                .map(move |y| {
                    let (gx, gy) = gx_gy(img, x, y);
                    (edge_magnitude((gx, gy)), edge_direction((gx, gy)))
                })
                .collect::<Vec<(f64, f64)>>()
        })
        .collect::<Vec<Vec<(f64, f64)>>>()
}

// return offset for the pixels in the direction of the angle (in direction and opposite direction)
pub fn pixel_dir_offsets(angle: f64) -> ((i32, i32), (i32, i32)) {
    let angle = (angle + 360.0) % 360.0; // normalize angle
    match angle {
        0.0..=22.5 | 337.5..=360.0 | 157.5..=202.5 => ((1, 0), (-1, 0)), // right & left
        22.5..=67.5 | 202.5..=247.5 => ((1, -1), (-1, 1)),               // top right & bottom left
        67.5..=112.5 | 247.5..=292.5 => ((0, -1), (0, 1)),               // top & bottom
        112.5..=157.5 | 292.5..=337.5 => ((-1, -1), (1, 1)),             // top left & bottom right
        _ => unreachable!("angle out of bounds: {}", angle),
    }
}

pub fn non_maximum_suppression(img: &mut DynamicImage) {
    let gradient = intensity_gradient(img);

    for x in 0..img.width() as i32 {
        for y in 0..img.height() as i32 {
            let (offset_x, offset_y) = pixel_dir_offsets(gradient[x as usize][y as usize].1);
            let (x1, y1) = (x + offset_x.0, y + offset_x.1);
            let (x2, y2) = (x + offset_y.0, y + offset_y.1);

            if x1 < 0
                || x1 >= img.width() as i32
                || y1 < 0
                || y1 >= img.height() as i32
                || x2 < 0
                || x2 >= img.width() as i32
                || y2 < 0
                || y2 >= img.height() as i32
            {
                continue;
            } // bound check

            let (x, y) = (x as usize, y as usize);
            let (x1, y1) = (x1 as usize, y1 as usize);
            let (x2, y2) = (x2 as usize, y2 as usize);

            let curr_mag = gradient[x][y].0;
            if curr_mag < gradient[x1][y1].0 || curr_mag < gradient[x2][y2].0 {
                img.put_pixel(x as u32, y as u32, image::Rgba([0, 0, 0, 255])); // suppress if weak
            }
        }
    }
}

pub fn double_threshold(img: &mut DynamicImage, (low, high): (u8, u8)) {
    let (width, height) = img.dimensions();

    (0..width).for_each(|x| {
        (0..height).for_each(|y| {
            let pixel = img.get_pixel(x, y)[0];
            let new_pixel = if pixel >= high {
                // strong edge
                255
            } else if pixel > low {
                // weak edge
                128
            } else {
                // suppressed
                0
            };
            if new_pixel == 128 {
                img.put_pixel(x, y, Rgba([128, 128, 0, 255]));
            } else {
                img.put_pixel(x, y, Rgba([new_pixel, new_pixel, new_pixel, 255]));
            }
        });
    });
}

// version without dfs
pub fn hysteresis(img: &mut DynamicImage, (low, high): (u8, u8)) {
    let (width, height) = img.dimensions();

    (0..width).for_each(|x| {
        (0..height).for_each(|y| {
            let pixel = img.get_pixel(x, y)[0];
            if pixel == 255 {
                for i in -1..2 {
                    for j in -1..2 {
                        let new_x = x as i32 + i;
                        let new_y = y as i32 + j;
                        if new_x >= 0 && new_x < width as i32 && new_y >= 0 && new_y < height as i32
                        {
                            let new_pixel = img.get_pixel(new_x as u32, new_y as u32)[0];
                            if new_pixel == 128 {
                                img.put_pixel(
                                    new_x as u32,
                                    new_y as u32,
                                    Rgba([255, 255, 255, 255]),
                                );
                            }
                        }
                    }
                }
            }
        });
    });
}

/// The Canny edge detection algorithm
///
/// # Arguments
/// * `img` - The image to be processed
/// * `low_threshold` - The lower threshold for double thresholding
/// * `high_threshold` - The higher threshold for double thresholding
///
/// # Returns
/// A new black and white image, with black representing edges
pub fn canny(img: &DynamicImage, low_threshold: u8, high_threshold: u8) -> DynamicImage {
    assert!(low_threshold < high_threshold);
    let mut new_img = img.grayscale();
    new_img.save("canny/0grayscale.png").unwrap();

    // 1. Gaussian blur
    new_img = gaussian_blur_5x5(&new_img);
    new_img.save("canny/1blur.png").unwrap();

    // 2 & 3. Find intensity gradients & Perform non-maximum suppression
    // doesn't work the way it should
    non_maximum_suppression(&mut new_img);
    new_img.save("canny/2nms.png").unwrap();

    // 4. Double thresholding
    double_threshold(&mut new_img, (low_threshold, high_threshold));
    new_img.save("canny/3threshold.png").unwrap();

    // 5. Edge tracking by hysteresis
    hysteresis(&mut new_img, (low_threshold, high_threshold));

    new_img
}

enum GaussianFilter {
    K3x3([f64; 9]),
    K5x5([f64; 25]),
    K7x7([f64; 49]),
}

fn gaussian_blur(img: &DynamicImage, kernel: GaussianFilter) -> DynamicImage {
    let mut new_img = img.clone();
    match kernel {
        GaussianFilter::K3x3(k) => apply_kernel(img, &mut new_img, &k),
        GaussianFilter::K5x5(k) => apply_kernel(img, &mut new_img, &k),
        GaussianFilter::K7x7(k) => apply_kernel(img, &mut new_img, &k),
    }
    new_img
}

pub fn apply_kernel<const S: usize>(
    img: &DynamicImage,
    new_img: &mut DynamicImage,
    kernel: &[f64; S],
) {
    let size = S.sqrt() as i32;
    for x in 0..img.width() {
        for y in 0..img.height() {
            let mut r = 0.0;
            let mut g = 0.0;
            let mut b = 0.0;

            for i in 0..size {
                for j in 0..size {
                    let new_x = (x as i32 + i - 2).max(0).min(img.width() as i32 - 1) as u32;
                    let new_y = (y as i32 + j - 2).max(0).min(img.height() as i32 - 1) as u32;

                    let pixel = img.get_pixel(new_x, new_y);
                    let kernel_val = kernel[(i * size + j) as usize];

                    r += pixel[0] as f64 * kernel_val;
                    g += pixel[1] as f64 * kernel_val;
                    b += pixel[2] as f64 * kernel_val;
                }
            }
            new_img.put_pixel(x, y, image::Rgba([r as u8, g as u8, b as u8, 255]));
        }
    }
}

/// Gaussian filtering for FPGA based image processing with High-Level Synthesis tools -
/// Scientific Figure on ResearchGate. Available from:
/// https://www.researchgate.net/figure/Discrete-approximation-of-the-Gaussian-kernels-3x3-5x5-7x7_fig2_325768087
/// [accessed 31 Jan, 2024]
/// uses 1D array for 2D kernel, probably shouldn't have performance impact but what is written, is written
const GAUSSIAN_3X3: [f64; 9] = [
    1.0 / 16.0,
    2.0 / 16.0,
    1.0 / 16.0,
    2.0 / 16.0,
    4.0 / 16.0,
    2.0 / 16.0,
    1.0 / 16.0,
    2.0 / 16.0,
    1.0 / 16.0,
];

const GAUSSIAN_5X5: [f64; 25] = [
    1.0 / 273.0,
    4.0 / 273.0,
    7.0 / 273.0,
    4.0 / 273.0,
    1.0 / 273.0,
    4.0 / 273.0,
    16.0 / 273.0,
    26.0 / 273.0,
    16.0 / 273.0,
    4.0 / 273.0,
    7.0 / 273.0,
    26.0 / 273.0,
    41.0 / 273.0,
    26.0 / 273.0,
    7.0 / 273.0,
    4.0 / 273.0,
    16.0 / 273.0,
    26.0 / 273.0,
    16.0 / 273.0,
    4.0 / 273.0,
    1.0 / 273.0,
    4.0 / 273.0,
    7.0 / 273.0,
    4.0 / 273.0,
    1.0 / 273.0,
];

const GAUSSIAN_7X7: [f64; 49] = [
    0.0 / 1003.0,
    0.0 / 1003.0,
    1.0 / 1003.0,
    2.0 / 1003.0,
    1.0 / 1003.0,
    0.0 / 1003.0,
    0.0 / 1003.0,
    0.0 / 1003.0,
    3.0 / 1003.0,
    13.0 / 1003.0,
    22.0 / 1003.0,
    13.0 / 1003.0,
    3.0 / 1003.0,
    0.0 / 1003.0,
    1.0 / 1003.0,
    13.0 / 1003.0,
    59.0 / 1003.0,
    97.0 / 1003.0,
    59.0 / 1003.0,
    13.0 / 1003.0,
    1.0 / 1003.0,
    2.0 / 1003.0,
    22.0 / 1003.0,
    97.0 / 1003.0,
    159.0 / 1003.0,
    97.0 / 1003.0,
    22.0 / 1003.0,
    2.0 / 1003.0,
    1.0 / 1003.0,
    13.0 / 1003.0,
    59.0 / 1003.0,
    97.0 / 1003.0,
    59.0 / 1003.0,
    13.0 / 1003.0,
    1.0 / 1003.0,
    0.0 / 1003.0,
    3.0 / 1003.0,
    13.0 / 1003.0,
    22.0 / 1003.0,
    13.0 / 1003.0,
    3.0 / 1003.0,
    0.0 / 1003.0,
    0.0 / 1003.0,
    0.0 / 1003.0,
    1.0 / 1003.0,
    2.0 / 1003.0,
    1.0 / 1003.0,
    0.0 / 1003.0,
    0.0 / 1003.0,
];

#[allow(dead_code)]
pub fn gaussian_blur_3x3(img: &DynamicImage) -> DynamicImage {
    gaussian_blur(img, GaussianFilter::K3x3(GAUSSIAN_3X3))
}

#[allow(dead_code)]
pub fn gaussian_blur_5x5(img: &DynamicImage) -> DynamicImage {
    gaussian_blur(img, GaussianFilter::K5x5(GAUSSIAN_5X5))
}

#[allow(dead_code)]
pub fn gaussian_blur_7x7(img: &DynamicImage) -> DynamicImage {
    gaussian_blur(img, GaussianFilter::K7x7(GAUSSIAN_7X7))
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_gx_gy() {
        let mut img = DynamicImage::new_luma8(3, 3);
        for x in 0..3 {
            for y in 0..3 {
                img.put_pixel(x, y, image::Rgba([x as u8 * y as u8, 0, 0, 0]));
            }
        }

        let (gx, gy) = gx_gy(&img, 1, 1);
        assert_eq!(gx, 0);
        assert_eq!(gy, 0);
    }

    #[test]
    fn test_edge_magnitude() {
        let magnitude = edge_magnitude((3, 4));
        assert!((magnitude - 5.0).abs() < 1e-10);

        let magnitude = edge_magnitude((0, 0));
        assert!((magnitude - 0.0).abs() < 1e-10);

        let magnitude = edge_magnitude((5, 12));
        assert!((magnitude - 13.0).abs() < 1e-10);
    }

    #[test]
    fn test_intensity_gradient() {
        let mut img = DynamicImage::new_luma8(3, 3);
        for x in 0..3 {
            for y in 0..3 {
                img.put_pixel(x, y, image::Rgba([x as u8 * y as u8, 0, 0, 0]));
            }
        }

        let gradient = intensity_gradient(&img);
        assert_eq!(gradient.len(), 3);
        assert_eq!(gradient[0].len(), 3);

        for x in 0..3 {
            for y in 0..3 {
                let (magnitude, direction) = gradient[x as usize][y as usize];
                assert!(magnitude >= 0.0);
                assert!((-PI..=PI).contains(&direction));
            }
        }
    }

    #[test]
    fn test_pixel_dir_offset_right() {
        let (offset_x, offset_y) = pixel_dir_offsets(0.0);
        assert_eq!(offset_x, (1, 0));
        assert_eq!(offset_y, (-1, 0));
    }
}
