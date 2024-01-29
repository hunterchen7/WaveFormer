
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

fn dfs(x: i32, y: i32, visited: &mut [bool], img: &DynamicImage, path: &mut Vec<(i32, i32)>, col: image::Rgba<u8>) {
  if oob(x, y, img) || img.get_pixel(x as u32, y as u32) != col { 
    // if out of bounds or already visited or not white
    return;
  }
  if visited[(y * img.width() as i32 + x) as usize] {
    path.push((x, y));
    return;
  }
  path.push((x, y)); // add to path
  visited[(y * img.width() as i32 + x) as usize] = true; // set visited
  for (i, j) in [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)] { // loop through surrounding 3x3
    dfs(x + i, y + j, visited, img, path, col);
  }
}

fn is_palindrome(s: usize, e: usize, path: &[(i32,i32)]) -> bool {
  let (mut start, mut end) = (s, e);
  while start < end {
    if path[start] != path[end] { return false; }
    start += 1;
    end -= 1;
  }
  true
}

fn remove_end_palindrome(path: &mut Vec<(i32, i32)>) {
  let longest_palindrome = (0..path.len()).rev().find(|&i| is_palindrome(0, i, path));
  if let Some(i) = longest_palindrome {
    path.truncate(path.len() - i / 2);
  }
}

fn remove_start_palindrome(path: &mut Vec<(i32, i32)>) {
  let longest_palindrome = (0..path.len()).rev().find(|&i| is_palindrome(0, i, path));
  if let Some(i) = longest_palindrome {
    // println!("i: {}", i);
    path.drain(0..i / 2);
  }
}

pub fn edges_to_lines(img: &mut DynamicImage, col: image::Rgba<u8>) -> Vec<Vec<(i32, i32)>> {
  let mut lines = vec![];
  let dims = img.dimensions();
  let mut visited = vec![false; (dims.0 * dims.1) as usize];

  for x in 0..dims.0 {
    for y in 0..dims.1 {
      if !visited[(y * dims.0 + x) as usize] && img.get_pixel(x, y) == col {
        let mut path = vec![];
        dfs(x as i32, y as i32, &mut visited, img, &mut path, col);
        // somehow neither of these seem to do anything but they work in tests
        // the idea is that if dfs backtracks to a point that is already in the path
        // from the end of the path, then it can be partially truncated
        remove_start_palindrome(&mut path);
        remove_end_palindrome(&mut path);        
        if path.len() > 32 {
          lines.push(path);
        }
      }
    }
  }
  lines
}

#[allow(dead_code)]
pub fn edges_to_lines_w(img: &mut DynamicImage) -> Vec<Vec<(i32, i32)>> {
  edges_to_lines(img, WHITE)
}

#[allow(dead_code)]
pub fn edges_to_lines_b(img: &mut DynamicImage) -> Vec<Vec<(i32, i32)>> {
  edges_to_lines(img, BLACK)
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remove_end_palindrome() {
    let mut path1 = vec![(0,0),(1,1),(2,2),(3,3),(4,4),(5,5),(6,6)];
    remove_end_palindrome(path1.as_mut());
    assert_eq!(path1, vec![(0,0),(1,1),(2,2),(3,3),(4,4),(5,5),(6,6)]);
  }

  #[test]
  fn test_remove_end_palindrome2() {
    let mut path2 = vec![(0,0),(1,1),(2,2),(3,3),(4,4),(5,5),(6,6),(5,5),(4,4),(3,3),(2,2),(1,1),(0,0)];
    remove_end_palindrome(path2.as_mut());
    assert_eq!(path2, vec![(0,0),(1,1),(2,2),(3,3),(4,4),(5,5),(6,6)]);
  }

  #[test]
  fn test_remove_end_palindrome3() {
    let mut path2 = vec![(0,0),(1,1),(0,1),(2,2),(3,3),(4,4),(5,5),(6,6),(5,5),(4,4),(3,3),(2,2),(0,1),(1,1),(0,0)];
    remove_end_palindrome(path2.as_mut());
    assert_eq!(path2, vec![(0,0),(1,1),(0,1),(2,2),(3,3),(4,4),(5,5),(6,6)]);
  }

  #[test]
  fn test_remove_start_palindrome() {
    let mut path1 = vec![(0,0),(1,1),(2,2),(3,3),(4,4),(5,5),(6,6)];
    remove_start_palindrome(path1.as_mut());
    assert_eq!(path1, vec![(0,0),(1,1),(2,2),(3,3),(4,4),(5,5),(6,6)]);
  }

  #[test]
  fn test_remove_start_palindrome2() {
    let mut path2 = vec![(0,0),(1,1),(2,2),(3,3),(4,4),(5,5),(6,6),(5,5),(4,4),(3,3),(2,2),(1,1),(0,0)];
    remove_start_palindrome(path2.as_mut());
    assert_eq!(path2, vec![(6,6),(5,5),(4,4),(3,3),(2,2),(1,1),(0,0)]);
  }

  #[test]
  fn test_remove_start_palindrome3() {
    let mut path2 = vec![(0,0),(1,1),(0,0),(2,2),(3,3),(4,4),(5,5),(6,6),(5,5),(4,4),(3,3),(2,2),(1,1),(0,0)];
    remove_start_palindrome(path2.as_mut());
    assert_eq!(path2, vec![(1,1),(0,0),(2,2),(3,3),(4,4),(5,5),(6,6),(5,5),(4,4),(3,3),(2,2),(1,1),(0,0)]);
  }
}
