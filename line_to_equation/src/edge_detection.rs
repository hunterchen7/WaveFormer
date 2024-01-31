use image::{DynamicImage, GenericImageView, GenericImage};
use num::integer::Roots;

#[allow(dead_code)]
pub fn sobel_threshold(img: &DynamicImage, threshold: u8, use_g: bool) -> DynamicImage {
  let mut new_img = img.clone();
  let kernel_x = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
  let kernel_y = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];  

  for x in 0..img.width() {
    for y in 0..img.height() {
      let mut gx = 0;
      let mut gy = 0;

      for i in 0..3 {
        for j in 0..3 {
          let new_x = ((x as i32 + i - 1).max(0) as u32).min(img.width() - 1);
          let new_y = ((y as i32 + j - 1).max(0) as u32).min(img.height() - 1);

          let pixel = img.get_pixel(new_x, new_y)[0] as i32;
          gx += kernel_x[i as usize][j as usize] * pixel;
          gy += kernel_y[i as usize][j as usize] * pixel;
        }
      }

      let g = ((gx.pow(2) + gy.pow(2)) as f64).sqrt() as u8;
      if g >= threshold && x > 0 && y > 0 { // threshold for white
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

pub fn gaussian_blur(img: &DynamicImage, kernel: &[f64]) -> DynamicImage {
  let mut new_img = img.clone();
  let size = kernel.len().sqrt() as i32; // square matrix

  for x in 0..img.width() {
    for y in 0..img.height() {
      let mut r = 0.0;
      let mut g = 0.0;
      let mut b = 0.0;

      for i in 0..size {
        for j in 0..size {
          let new_x = ((x as i32 + i - 2).max(0).min(img.width() as i32 - 1)) as u32;
          let new_y = ((y as i32 + j - 2).max(0).min(img.height() as i32 - 1)) as u32;

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
  new_img
}

// Gaussian filtering for FPGA based image processing with High-Level Synthesis tools -
// Scientific Figure on ResearchGate. Available from:
// https://www.researchgate.net/figure/Discrete-approximation-of-the-Gaussian-kernels-3x3-5x5-7x7_fig2_325768087
// [accessed 31 Jan, 2024]
#[allow(dead_code)]
pub fn gaussian_blur_5x5(img: &DynamicImage) -> DynamicImage {
  gaussian_blur(img, &[
      1.0, 4.0, 7.0, 4.0, 1.0,
      4.0, 16.0, 26.0, 16.0, 4.0,
      7.0, 26.0, 41.0, 26.0, 7.0,
      4.0, 16.0, 26.0, 16.0, 4.0,
      1.0, 4.0, 7.0, 4.0, 1.0
    ].iter().map(|x| x / 273.0).collect::<Vec<f64>>()
  )
}

#[allow(dead_code)]
pub fn gaussian_blur_3x3(img: &DynamicImage) -> DynamicImage {
  gaussian_blur(img, &[
      1.0, 2.0, 1.0,
      2.0, 4.0, 2.0,
      1.0, 2.0, 1.0
    ].iter().map(|x| x / 16.0).collect::<Vec<f64>>()
  )
}

#[allow(dead_code)]
pub fn gaussian_blur_7x7(img: &DynamicImage) -> DynamicImage {
  gaussian_blur(img, &[
      0.0, 0.0, 1.0, 2.0, 1.0, 0.0, 0.0,
      0.0, 3.0, 13.0, 22.0, 13.0, 3.0, 0.0,
      1.0, 13.0, 59.0, 97.0, 59.0, 13.0, 1.0,
      2.0, 22.0, 97.0, 159.0, 97.0, 22.0, 2.0,
      1.0, 13.0, 59.0, 97.0, 59.0, 13.0, 1.0,
      0.0, 3.0, 13.0, 22.0, 13.0, 3.0, 0.0,
      0.0, 0.0, 1.0, 2.0, 1.0, 0.0, 0.0
    ].iter().map(|x| x / 1003.0).collect::<Vec<f64>>()
  )
}
