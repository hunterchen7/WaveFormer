mod img_to_line;
mod fourier;
mod edge_detection;
use std::{fs::File, io::Write};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let img = img_to_line::get_image("images/islands1.png");

    let now = Instant::now();
    let mut edges = edge_detection::sobel(&img);
    println!("Sobel: {:?}", now.elapsed());
    edges.save("images/generated/edges.png").unwrap();

    let now = Instant::now();
    let lines = img_to_line::img_to_lines(&mut edges);
    println!("Img to lines: {:?}", now.elapsed());
    img_to_line::lines_to_img(&lines);

    
    let mut file = File::create("images/generated/equations.txt").unwrap();

    let mut equations = Vec::new();
    for line in lines.iter() {
        let equation = fourier::construct_equation(line);
        let _ = file.write(equation.as_bytes());
        let _ = file.write(b"\n");
        equations.push(equation);
    // println!("Equation: {}", equation);
    }

    // println!("{:?}",equations);
    Ok(())
}