use image::{GenericImageView, DynamicImage, GenericImage};

fn get_image(path: &str) -> DynamicImage {
  image::open(path).unwrap()
} // fn get_image()

fn first_black(img: &DynamicImage) -> Option<(i32, i32)> {
  let dims = img.dimensions();

  for x in 0..dims.0 {
    for y in 0..dims.1 {
      if img.get_pixel(x, y) == image::Rgba([0, 0, 0, 255]) {
        return Some((x as i32,y as i32));
      }
    }
  }
  None
} // fn first_black(

pub fn img_to_line(path: &str) -> Vec<(i32, i32)> {
  let mut img = get_image(path);
  
  let fb = first_black(&img);
  let mut line = vec![];

  let mut curr = match fb { 
    Some((x,y)) => (x,y),
    None => return line,
  };

  img.put_pixel(curr.0 as u32, curr.1 as u32, image::Rgba([255, 255, 255, 255])); // set black to white
  line.push(curr); // add to line
  
  // dfs
  loop {
    let mut found = false;
    for x in (curr.0-1)..(curr.0+2) { 
      for y in (curr.1-1)..(curr.1+2) { // loop through surrounding 3x3
        if img.get_pixel(x as u32, y as u32) == image::Rgba([0, 0, 0, 255]) { // matching self is ok because it's white now
          curr = (x,y);
          img.put_pixel(curr.0 as u32, curr.1 as u32, image::Rgba([255, 255, 255, 255])); // set black to white
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

pub fn line_to_img(line: &[(i32, i32)]) {
  let mut img = DynamicImage::new_rgb8(500, 500);
  for point in line.iter() {
    img.put_pixel(point.0 as u32, point.1 as u32, image::Rgba([255, 255, 255, 255]));
  }
  img.save("images/line.png").unwrap();
} // fn line_to_img()