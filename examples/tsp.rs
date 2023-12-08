
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;

use geo::Point;

fn main() -> std::io::Result<()> {
    let points = read_input(PathBuf::from("data/tsp_input.txt"))?;

    // TODO: TSP
    let length = 12345.6789;

    println!("Grade: {}", grade(length));

    output(length, points);

    Ok(())
}

fn read_input(path: PathBuf) -> std::io::Result<Vec<Point>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let _n: usize = lines.next().unwrap()?.parse().unwrap();
    let points = lines
        .map(|line| {
            let bind = line.unwrap();
            let mut values = bind.split_whitespace();

            let x = values.next().unwrap().parse().unwrap();
            let y = values.next().unwrap().parse().unwrap();

            (x, y)
        })
        .map(|xy| Point::from(xy))
        .collect();

    Ok(points)
}

fn output(length: f64, points: Vec<Point>) {
    println!("{}", length);
    for point in points {
        println!("{} {}", point.x(), point.y());
    }
}

fn grade(length: f64) -> f64 {
    -1.009 * 10f64.powf(-5.) * length + 224.
}
