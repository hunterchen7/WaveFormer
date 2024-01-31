use image::{DynamicImage, GenericImageView, GenericImage};

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

      let g = ((gx.pow(2) + gy.pow(2)) as f32).sqrt() as u8;
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
  sobel_threshold(img, 0, false)
}

#[allow(dead_code)]
pub fn sobel_default(img: &DynamicImage) -> DynamicImage {
  sobel_threshold(img, 128, true)
}

const GAUSSIAN_KERNEL_5X5: [[f32; 5]; 5] = [
  [1.0, 4.0, 6.0, 4.0, 1.0],
  [4.0, 16.0, 24.0, 16.0, 4.0],
  [6.0, 24.0, 36.0, 24.0, 6.0],
  [4.0, 16.0, 24.0, 16.0, 4.0],
  [1.0, 4.0, 6.0, 4.0, 1.0]
];
pub fn gaussian_blur_5x5(img: &DynamicImage) -> DynamicImage {
  let mut new_img = img.clone();
    let kernel = GAUSSIAN_KERNEL_5X5;

  for x in 0..img.width() {
    for y in 0..img.height() {
      let mut r = 0.0;
      let mut g = 0.0;
      let mut b = 0.0;

      for i in 0..5 {
        for j in 0..5 {
          let new_x = ((x as i32 + i - 2).max(0).min(img.width() as i32 - 1)) as u32;
          let new_y = ((y as i32 + j - 2).max(0).min(img.height() as i32 - 1)) as u32;

          let pixel = img.get_pixel(new_x, new_y);
          let kernel_val = kernel[i as usize][j as usize];

          r += pixel[0] as f32 * kernel_val;
          g += pixel[1] as f32 * kernel_val;
          b += pixel[2] as f32 * kernel_val;
        }
      }

      // normalize
      r /= 255.0;
      g /= 255.0;
      b /= 255.0;

      new_img.put_pixel(x, y, image::Rgba([r as u8, g as u8, b as u8, 255]));
    }
  }
  new_img
}
