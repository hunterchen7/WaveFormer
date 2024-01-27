use image::{GenericImageView, DynamicImage, GenericImage};
use rand::Rng;

pub fn get_image(path: &str) -> DynamicImage {
  image::open(path).unwrap()
} // fn get_image()

fn first_col(img: &DynamicImage, col: image::Rgba<u8>) -> Option<(i32, i32)> {
  let dims = img.dimensions();

  for x in 0..dims.0 {
    for y in 0..dims.1 {
      if img.get_pixel(x, y) == col {
        return Some((x as i32,y as i32));
      }
    }
  }
  None
}

#[allow(dead_code)]
fn first_black(img: &DynamicImage) -> Option<(i32, i32)> {
  first_col(img, image::Rgba([0, 0, 0, 255]))
}

fn first_white(img: &DynamicImage) -> Option<(i32, i32)> {
  first_col(img, image::Rgba([255, 255, 255, 255]))
}

pub fn img_to_line(img: &mut DynamicImage) -> Vec<(i32, i32)> {
  let fb = first_white(img);
  let mut line = vec![];

  let mut curr = match fb { 
    Some((x,y)) => (x,y),
    None => return line,
  };

  img.put_pixel(curr.0 as u32, curr.1 as u32, image::Rgba([0, 0, 0, 255])); // set black to white
  line.push(curr); // add to line
  
  // dfs
  loop {
    let mut found = false;
    for x in (curr.0-1)..(curr.0+2) { 
      for y in (curr.1-1)..(curr.1+2) { // loop through surrounding 3x3
        if img.get_pixel(x as u32, y as u32) == image::Rgba([255, 255, 255, 255]) { // matching self is ok because it's white now
          curr = (x,y);
          img.put_pixel(curr.0 as u32, curr.1 as u32, image::Rgba([0, 0, 0, 255])); // set black to white
          line.push(curr); // add to line
          found = true;
          break;
        }
      }
      if found {
        break;
      }
    }
    if !found {
      break;
    }
  }
  line
}

pub fn img_to_lines(img: &mut DynamicImage) -> Vec<Vec<(i32, i32)>> {
  let mut lines = vec![];
  loop {
    let line = img_to_line(img);
    if line.is_empty() {
      break;
    }
    if line.len() > 4 { // ignore arbitarily small lines
      lines.push(line);
    }
  }
  lines
}

pub fn line_to_img(img: &mut DynamicImage, line: &[(i32, i32)], col: image::Rgba<u8>) {
  for point in line.iter() {
    img.put_pixel(point.0 as u32, point.1 as u32, col);
  }
} // fn line_to_img()

fn random_col() -> image::Rgba<u8> {
  let col1 = rand::thread_rng().gen_range(100..255);
  let col2 = rand::thread_rng().gen_range(100..255);
  let col3 = rand::thread_rng().gen_range(100..255);
  image::Rgba([col1, col2, col3, 255])
}

pub fn lines_to_img(lines: &[Vec<(i32, i32)>]) {
  let (mut max_x, mut max_y) = (0, 0);
  for line in lines.iter() {
    for point in line.iter() {
      if point.0 > max_x {
        max_x = point.0;
      }
      if point.1 > max_y {
        max_y = point.1;
      }
    }
  }
  let mut img = DynamicImage::new_rgb8(max_x as u32 + 1, max_y as u32 + 1);
  for line in lines.iter() {
    let col = random_col();
    line_to_img(&mut img, line, col);
  }
  img.save("images/generated/lines.png").unwrap();
} // fn lines_to_img()