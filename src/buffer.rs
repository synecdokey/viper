use std::fs::File;
use std::io::BufReader;

use ropey::Rope;

pub struct Buffer<'a> {
    pub filename: &'a str,
    pub text: Rope,
}

impl<'a> Buffer<'a> {
    pub fn new(filename: &'a str) -> Self {
        Buffer {
            text: Rope::from_reader(BufReader::new(File::open(filename).unwrap())).unwrap(),
            filename,
        }
    }
}
