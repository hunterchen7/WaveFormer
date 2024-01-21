use image::{GenericImageView, DynamicImage, GenericImage};

fn get_image(path: &str) -> DynamicImage {
  image::open(path).unwrap()
} // fn get_image()

fn first_black(img: &DynamicImage) -> Option<(u32, u32)> {
  let dims = img.dimensions();

  for x in 0..dims.0 {
    for y in 0..dims.1 {
      if img.get_pixel(x, y) == image::Rgba([0, 0, 0, 255]) {
        return Some((x,y));
      }
    }
  }
  None
} // fn first_black(

pub fn img_to_line(path: &str) -> Vec<(u32, u32)> {
  let mut img = get_image(path);
  
  let fb = first_black(&img);
  let mut line = vec![];

  let mut curr = match fb { 
    Some((x,y)) => (x,y),
    None => return line,
  };

  img.put_pixel(curr.0, curr.1, image::Rgba([255, 255, 255, 255])); // set black to white
  line.push(curr); // add to line
  
  // dfs
  loop {
    let mut found = false;
    for x in (curr.0-1)..(curr.0+2) { 
      for y in (curr.1-1)..(curr.1+2) { // loop through surrounding 3x3
        if img.get_pixel(x, y) == image::Rgba([0, 0, 0, 255]) { // matching self is ok because it's white now
          curr = (x,y);
          img.put_pixel(curr.0, curr.1, image::Rgba([255, 255, 255, 255])); // set black to white
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

  // println!("Line: {:?}", line);
  line
}