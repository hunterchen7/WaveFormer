use image::{DynamicImage, GenericImageView, GenericImage};

pub fn sobel(img: &DynamicImage) -> DynamicImage {
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
      if g > 128 && x > 0 && y > 0 { // threshold for white
        new_img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
      } else {
        new_img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
      }
    }
  }
  new_img
}

