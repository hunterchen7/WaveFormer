mod img_to_line;
mod fourier;

fn main() {
  let line = img_to_line::img_to_line("images/eg1.png");
  img_to_line::line_to_img(&line); // to check generated line
  // println!("Line: {:?}", line);
  // println!("cos: {:?}", (10_f32).cos());
  // let mut complex_line = line.iter().map(|&(x,y)| Complex::new(x as f64, y as f64)).collect::<Vec<Complex<f64>>>();
  // println!("Line: {:?}", line);
  // println!("Complex Line: {:?}", complex_line);
  
  let skip = 7;
  let skipped_line: Vec<(i32,i32)> = line.iter().skip(skip - 1).step_by(skip).copied().collect();
  // img_to_line::line_to_img(&skipped_line.iter().map(&|c: &Complex<f64>| (c.re as i32, c.im as i32)).collect::<Vec<(i32, i32)>>());
  // let mut planner: FftPlanner<f64>  = FftPlanner::new();
  // let fft = planner.plan_fft_forward(skipped_line.len());

  // fft.process(&mut skipped_line);

  // println!("FFT: {:?}", complex_line);
  let equation = fourier::construct_equation(&line);
  println!("Equation: {}", equation);
}