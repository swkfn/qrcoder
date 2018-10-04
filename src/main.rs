extern crate clap;
extern crate image;
extern crate qrcode;

#[macro_use]
extern crate failure;

use std::io;
use std::path::{Path, PathBuf};
use std::process;

use clap::{App, Arg};

use image::Luma;

use qrcode::QrCode;
use qrcode::types::QrError;

struct GenerateOptions{
    text: String,
    output: Option<PathBuf>,
}

fn main(){
    println!("Hello, world!");
}
