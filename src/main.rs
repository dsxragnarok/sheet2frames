extern crate clap;
extern crate raster;
use clap::{Arg, App};
use raster::Image;
use std::process;
use std::path::Path;
use std::ffi::OsStr;

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

    let source_image = raster::open(input_file);
    let source_image = match source_image {
        Ok(file) => file,
        Err(error) => panic!("{:?}", error)
    };

    let frame_width = source_image.width / columns;
    let frame_height = source_image.height / rows;

    println!("width: {}, height: {}", source_image.width, source_image.height);
    println!("fwidth: {}, fheight: {}", frame_width, frame_height);

    // let input_filepath = Path::new(input_file);
    // let extension = input_filepath.extension().unwrap_or(OsStr::new("")).to_str().unwrap();
    let base_filename = Path::new(input_file).file_stem().and_then(OsStr::to_str).unwrap();

    for row in 0..rows {
        for column in 0..columns {
            let frame_index = (columns * row) + column;
            let x = column * frame_width;
            let y = row * frame_height;
            let filename = format!("{} ({})", base_filename, frame_index);

            println!("{}", filename);
        }
    }
}
