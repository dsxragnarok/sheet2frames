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
                    .arg(Arg::with_name("outputfile")
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

    println!("{:?}", matches);
}
