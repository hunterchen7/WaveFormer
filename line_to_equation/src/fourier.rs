use std::f64::consts::PI;
use num::Complex;

pub fn construct_equation(points: &[Complex<f64>]) -> String {
  let mut equation = String::new();
  let mut xt = vec![];
  let mut yt = vec![];
  for num in points.iter().enumerate() {
    let (a, b) = (num.1.re, num.1.im);
    let pin2 = 2.0 * PI * num.0 as f64;
    xt.push(format!("({} * cos({}t) - {} * sin({}t))", a, pin2, b, pin2));
    yt.push(format!("({} * sin({}t) + {} * cos({}t))", a, pin2, b, pin2));
  }
  // println!("xt: {:?}", xt);
  // println!("yt: {:?}", yt);
  equation.push_str("((");
  equation.push_str(&xt.join("+"));
  equation.push_str("),(");
  equation.push_str(&yt.join("+"));
  equation.push_str("))");
  equation
}
