mod img_to_line;
mod fourier;
use num::complex::Complex;
use rustfft::FftPlanner;

fn main() {
  let line = img_to_line::img_to_line("images/checkmark.png");
  img_to_line::line_to_img(&line); // to check generated line
  let mut complex_line = line.iter().map(|&(x,y)| Complex::new(x as f64, y as f64)).collect::<Vec<Complex<f64>>>();
  // println!("Line: {:?}", line);
  // println!("Complex Line: {:?}", complex_line);

  let mut planner: FftPlanner<f64>  = FftPlanner::new();
  let fft = planner.plan_fft_forward(line.len());

  fft.process(&mut complex_line);
  // println!("FFT: {:?}", complex_line);
  let equation = fourier::construct_equation(&complex_line);
  println!("Equation: {}", equation);
}