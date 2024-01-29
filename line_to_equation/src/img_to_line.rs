use image::{GenericImageView, DynamicImage, GenericImage};
use rand::Rng;

static WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
static BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);

pub fn get_image(path: &str) -> DynamicImage {
  image::open(path).unwrap()
} // fn get_image()

fn first_col(img: &DynamicImage, col: image::Rgba<u8>) -> Option<(u32, u32)> {
  let dims = img.dimensions();

  for x in 0..dims.0 {
    for y in 0..dims.1 {
      if img.get_pixel(x, y) == col {
        return Some((x, y));
      }
    }
  }
  None
}

#[allow(dead_code)]
fn first_black(img: &DynamicImage) -> Option<(u32, u32)> {
  first_col(img, BLACK)
}

#[allow(dead_code)]
fn first_white(img: &DynamicImage) -> Option<(u32, u32)> {
  first_col(img, WHITE)
}

#[allow(dead_code)]
fn first_col_from(img: &DynamicImage, col: image::Rgba<u8>, start: (u32, u32)) -> Option<(u32, u32)> {
  let dims = img.dimensions();

  for x in start.0..dims.0 {
    for y in start.1..dims.1 {
      if img.get_pixel(x, y) == col {
        return Some((x, y));
      }
    }
  }
  None
}

#[allow(dead_code)]
fn first_white_from(img: &DynamicImage, start: (u32, u32)) -> Option<(u32, u32)> {
  first_col_from(img, WHITE, start)
}

fn oob(x: i32, y: i32, img: &DynamicImage) -> bool {
  x < 0 || y < 0 || x as u32 >= img.width() || y as u32 >= img.height()
}

fn dfs(x: i32, y: i32, visited: &mut [bool], img: &DynamicImage, path: &mut Vec<(i32, i32)>) {
  if visited[(y * img.width() as i32 + x) as usize] {
    path.push((x, y));
    return;
  }
  if oob(x, y, img) || img.get_pixel(x as u32, y as u32) != WHITE { 
    // if out of bounds or already visited or not white
    return;
  }
  path.push((x, y)); // add to path
  visited[(y * img.width() as i32 + x) as usize] = true; // set visited
  for (i, j) in [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)] { // loop through surrounding 3x3
    dfs(x + i, y + j, visited, img, path);
  }
}

pub fn img_to_lines(img: &mut DynamicImage) -> Vec<Vec<(i32, i32)>> {
  let mut lines = vec![];
  let dims = img.dimensions();
  let mut visited = vec![false; (dims.0 * dims.1) as usize];

  for x in 0..dims.0 {
    for y in 0..dims.1 {
      if !visited[(y * dims.0 + x) as usize] && img.get_pixel(x, y) == WHITE {
        let mut path = vec![];
        dfs(x as i32, y as i32, &mut visited, img, &mut path);
        if path.len() > 4 {
          lines.push(path);
        }
      }
    }
  }
  lines
}

fn random_col() -> image::Rgba<u8> {
  let col1 = rand::thread_rng().gen_range(100..255);
  let col2 = rand::thread_rng().gen_range(100..255);
  let col3 = rand::thread_rng().gen_range(100..255);
  image::Rgba([col1, col2, col3, 255])
}

pub fn line_to_img(img: &mut DynamicImage, line: &[(i32, i32)], col: image::Rgba<u8>) {
  for point in line.iter() {
    img.put_pixel(point.0 as u32, point.1 as u32, col);
  }
} // fn line_to_img()

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
  let mut img = DynamicImage::new_rgb8(max_x as u32 + 50, max_y as u32 + 50);
  for line in lines.iter() {
    let col = random_col();
    line_to_img(&mut img, line, col);
  }
  img.save("images/generated/lines.png").unwrap();
} // fn lines_to_img()