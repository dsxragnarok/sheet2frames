extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Sheet 2 Frames")
                    .version("0.1.0")
                    .author("Kevin Phung <kevin.codererrant@gmail.com>")
                    .about("Script to cut a sprite sheet or strip into individual frame files")
                    .arg(Arg::with_name("inputfile")
                        .short("i")
                        .long("input")
                        .required(true)
                        .takes_value(true)
                        .index(1)
                        .help("input file path"))
                    .arg(Arg::with_name("outputDir")
                        .short("o")
                        .long("output")
                        .required(false)
                        .takes_value(true)
                        .help("output directory path"))
                    .arg(Arg::with_name("columns")
                        .short("c")
                        .long("columns")
                        .required(true)
                        .takes_value(true)
                        .index(2)
                        .help("the number of columns in the sprite sheet"))
                    .arg(Arg::with_name("rows")
                        .short("r")
                        .long("rows")
                        .required(false)
                        .takes_value(true)
                        .help("the number of rows in the sprite sheet"))
                    .get_matches();

    let input_file = matches.value_of("inputfile").unwrap();
    let output_path = matches.value_of("outputDir").unwrap_or(".");
    let columns: i32 = matches.value_of("columns").unwrap().parse().unwrap();
    let rows: i32 = matches.value_of("rows").unwrap_or("1").parse().unwrap();

    println!("Input: {}", input_file);
    println!("Output: {}", output_path);
    println!("c: {}, r: {}", columns, rows);
}
