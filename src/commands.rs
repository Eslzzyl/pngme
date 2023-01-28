use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use crate::args::*;
use crate::chunk_type::ChunkType;
use crate::chunk::Chunk;
use crate::png::Png;

pub fn encode(params: &EncodeCommand) {
    let mut png = read_png(&params.image_path);
    let chunk_type = ChunkType::from_str(&params.chunk_type).unwrap();
    let message = params.message.clone().into_bytes();
    // println!("{}", String::from_utf8(message.clone()).unwrap());
    let message_chunk = Chunk::new(chunk_type, message);
    // println!("{:?}", &chunk_type);
    png.append_chunk(message_chunk);
    match &params.output_file {
        Some(dest_path) => write_png(&dest_path, &png),
        None => write_png(&params.image_path, &png)
    };
    println!("The message has been encoded into the file.");
}

pub fn decode(params: &DecodeCommand) {
    let png = read_png(&params.image_path);
    let chunk_type = &params.chunk_type;
    let chunk = png.chunk_by_type(&chunk_type).expect("Chunk not found!");
    println!("The message is:\n{}", String::from_utf8(chunk.data().to_vec()).unwrap());
}

pub fn remove(params: &RemoveCommand) {
    let mut png = read_png(&params.image_path);
    let chunk_type = &params.chunk_type;
    png.remove_chunk(&chunk_type).expect("Chunk not found!");
    println!("Successfully removed chunk with pattern {} in file {}.",
        chunk_type.to_string(), &params.image_path.to_str().unwrap());
}

pub fn print(params: &PrintCommand) {
    let png = read_png(&params.image_path);
    let mut i: usize = 0;
    for chunk in png.chunks() {
        println!("Chunk {}; Type: {}; Data Length: {} Byte", i, chunk.chunk_type(), chunk.length());
        i += 1;
    }
}

fn read_png(path: &PathBuf) -> Png {
    let mut png_file = File::open(path).expect("Couldn't open file.");
    let mut png_bytes = Vec::<u8>::new();
    png_file.read_to_end(&mut png_bytes).expect("Error when reading PNG file.");
    // let png_bytes: &[u8] = &fs::read(path).expect("Couldn't open file.");
    let png = Png::try_from(png_bytes.as_slice()).expect("Error when creating PNG object.");
    png
}

fn write_png(path: &PathBuf, png: &Png) {
    // fs::write(path, png.to_string()).expect("Couldn't write PNG file.");
    let mut png_file = match File::open(path) {
        Ok(png_file) => png_file,
        Err(_) => File::create(path).expect("Error when creating file.")
    };
    png_file.write_all(&png.as_bytes()).expect("Error when reading PNG file.");
}