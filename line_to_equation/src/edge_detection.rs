use image::{DynamicImage, GenericImageView, GenericImage};

pub fn sobel(img: &DynamicImage) -> DynamicImage {
  let mut new_img = img.clone();

  for x in 1..(img.width() - 1) {
    for y in 1..(img.height() - 1) {
      let mut gx = 0;
      let mut gy = 0;

      for i in 0..3 {
        for j in 0..3 {
          let pixel = img.get_pixel(x + i - 1, y + j - 1)[0] as i32;
          let kernel_x = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
          let kernel_y = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

          gx += kernel_x[i as usize][j as usize] * pixel;
          gy += kernel_y[i as usize][j as usize] * pixel;
        }
      }

      let g = ((gx.pow(2) + gy.pow(2)) as f32).sqrt() as u8;
      if g > 128 {
        new_img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
      } else {
        new_img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
      }
    }
  }
  new_img
}

