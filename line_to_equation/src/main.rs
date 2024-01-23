mod img_to_line;
mod fourier;
use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
  let mut img = img_to_line::get_image("images/islands2.png");
  let lines = img_to_line::img_to_lines(&mut img);
  img_to_line::lines_to_img(&lines);

  let mut file = File::create("images/equations.txt").unwrap();
  
  let mut equations = Vec::new();
  for line in lines.iter() {
    let equation = fourier::construct_equation(line);
    let _ = file.write(equation.as_bytes());
    let _ = file.write(b"\n\n");
    equations.push(equation);
    // println!("Equation: {}", equation);
  }

  println!("{:?}",equations);
  Ok(())

  // println!("Line: {:?}", line);

  // let skip = 7;
  // let skipped_line: Vec<(i32,i32)> = line.iter().skip(skip - 1).step_by(skip).copied().collect();
  
}