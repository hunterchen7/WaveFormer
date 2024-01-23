use std::f64::consts::PI;

pub fn construct_equation(points: &[(i32,i32)]) -> String {
  let mut equation = String::new();
  let mut xt = vec![];
  let mut yt = vec![];

  let iterations = 25;

  let mut new_points = points.to_vec();

  for points in points.iter().rev() {
    new_points.push((points.0, points.1));
  }

  let len = new_points.len() as f64;

  for n in 0..(iterations*2 + 1) {
    let (mut cx, mut cy) = (0.0, 0.0);
    let k = (n - iterations) as f64; // goes from -iterations to iterations
    let pik2 = 2.0 * PI * k;

    for (i, p) in new_points.iter().enumerate() {
      let f_num = (p.0 as f64, p.1 as f64);
      let m = i as f64;
      cx += (pik2 * m / len).cos() * f_num.0 + (pik2 * m / len).sin() * f_num.1;
      cy += (pik2 * m / len).cos() * f_num.1 - (pik2 * m / len).sin() * f_num.0;
    }

    xt.push(format!("{} cos({}t) - {} sin({}t)", cx / len, k * PI, cy / len, k * PI));
    yt.push(format!("{} sin({}t) + {} cos({}t)", cx / len, k * PI, cy / len, k * PI));
  }
  // println!("xt: {:?}", xt);
  // println!("yt: {:?}", yt);
  equation.push_str("((");
  equation.push_str(&xt.join("+"));
  equation.push_str("),-(");
  equation.push_str(&yt.join("+"));
  equation.push_str("))");
  equation
}
