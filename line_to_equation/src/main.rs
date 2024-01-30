mod img_to_line;
mod fourier;
mod edge_detection;
use std::{fs::File, io::Write};
use std::time::Instant;
use std::thread;

fn main() -> std::io::Result<()> {
    let builder = thread::Builder::new().stack_size(32 * 1024 * 1024);

    let handler = builder.spawn(|| {
        let img = img_to_line::get_image("images/smile.png");

        /*let now = Instant::now();
        let mut edges = edge_detection::sobel(&img);
        println!("Sobel: {:?}", now.elapsed());
        edges.save("generated/edges.png").unwrap();*/

        let now = Instant::now();
        let lines = img_to_line::edges_to_lines_b(&mut img);
        println!("Img to lines: {:?}", now.elapsed());
        img_to_line::lines_to_img(&lines);

        let mut file = File::create("generated/equations.txt").unwrap();

        let now = Instant::now();
        let mut equations = Vec::new();
        for line in lines.iter() {
            // let equation = fourier::construct_equation(line);
            // construct equation but it only uses every nth point
            let equation = fourier::construct_equation(&line.iter().step_by(10).map(|p| *p).collect::<Vec<_>>());
            let _ = file.write(equation.as_bytes());
            let _ = file.write(b"\n");
            equations.push(equation);
            // println!("Equation: {}", equation);
        }
        println!("Construct equations: {:?}", now.elapsed());
    }).unwrap();

    handler.join().unwrap();
    Ok(())
}