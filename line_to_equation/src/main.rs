mod img_to_line;
mod fourier;
mod edge_detection;
use std::{fs::File, io::Write};
use std::time::Instant;
use std::thread;

fn main() -> std::io::Result<()> {
    let builder = thread::Builder::new().stack_size(32 * 1024 * 1024);

    let handler = builder.spawn(|| {
        let mut img = img_to_line::get_image("images/toronto.png");

        let now = Instant::now();
        let mut edges = edge_detection::sobel(&img);
        println!("Sobel: {:?}", now.elapsed());
        edges.save("generated/edges.png").unwrap();

        let now = Instant::now();
        let mut lines = img_to_line::edges_to_lines_w(&mut edges);
        lines.sort_by(|a, b| b.len().cmp(&a.len())); // sort by length
        lines.truncate(32); // only take n longest lines

        let mut file = File::create("generated/equations.txt").unwrap();

        let now = Instant::now();
        let mut equations = Vec::new();
        for line in lines.iter() {
            let equation = fourier::construct_equation(line);
            // construct equation but it only uses every nth point
            // let equation = fourier::construct_equation(&line.iter().step_by(10).map(|p| *p).collect::<Vec<_>>());
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