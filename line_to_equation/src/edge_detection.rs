use image::{DynamicImage, GenericImage, GenericImageView};
use num::integer::Roots;

type SobelPoint = (i32, i32);

const SOBEL_X: [[i32; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
const SOBEL_Y: [[i32; 3]; 3] = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];


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
    (gy as f64).atan2(gx as f64)
}

pub fn edge_magnitude((gx, gy): (i32, i32)) -> f64 {
    ((gx.pow(2) + gy.pow(2)) as f64).sqrt()
}

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

pub fn intensity_gradient(img: &DynamicImage) -> Vec<Vec<(f64, f64)>> {
    let mut gradient = vec![vec![(0.0, 0.0); img.height() as usize]; img.width() as usize];

    for x in 0..img.width() {
        for y in 0..img.height() {
            let magnitude = edge_magnitude(gx_gy(img, x, y));
            let direction = edge_direction(gx_gy(img, x, y));
            gradient[x as usize][y as usize] = (magnitude, direction);
        }
    }
    gradient
}

// return offset for the pixels in the direction of the angle
pub fn pixel_dir_offsets(angle: f64) -> ((i32, i32), (i32, i32)) {
    match angle {
        _ if (-22.5..22.5).contains(&angle) || (157.5..202.5).contains(&angle) => ((1, 0), (-1, 0)),
        _ if (22.5..67.5).contains(&angle) || (202.5..247.5).contains(&angle) => ((1, -1), (-1, 1)),
        _ if (67.5..112.5).contains(&angle) || (247.5..292.5).contains(&angle) => ((0, -1), (0, 1)),
        _ if (112.5..157.5).contains(&angle) || (292.5..337.5).contains(&angle) => ((-1, -1), (1, 1)),
        _ => ((0, 0), (0, 0)), // case shouldn't ever be reached
    }
}

pub fn lower_bound_cutoff_supression(img: &DynamicImage) -> DynamicImage {
    let mut new_img = img.clone();
    let gradient = intensity_gradient(img);

    for x in 0..img.width() {
        for y in 0..img.height() {
            let (offset1, offset2) = pixel_dir_offsets(gradient[x as usize][y as usize].1);
            let (x1, y1) = (x as i32 + offset1.0, y as i32 + offset1.1);
            let (x2, y2) = (x as i32 + offset2.0, y as i32 + offset2.1);
            let curr_mag = gradient[x as usize][y as usize].0;
            if curr_mag < gradient[x1 as usize][y1 as usize].0 || curr_mag < gradient[x2 as usize][y2 as usize].0 {
                new_img.put_pixel(x, y, image::Rgba([0, 0, 0, 255])); // suppress if weak
            }
        }
    }

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

// Gaussian filtering for FPGA based image processing with High-Level Synthesis tools -
// Scientific Figure on ResearchGate. Available from:
// https://www.researchgate.net/figure/Discrete-approximation-of-the-Gaussian-kernels-3x3-5x5-7x7_fig2_325768087
// [accessed 31 Jan, 2024]
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
