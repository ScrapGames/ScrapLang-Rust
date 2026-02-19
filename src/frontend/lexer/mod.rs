mod helper;
use std::{
    fs::File,
    io::{ BufReader, Seek, SeekFrom }
};

use crate::position::Position;

pub struct Lexer {
    pub position: Position,
    reader: BufReader<File>,
}

impl Lexer {
    pub fn new(path: &str) -> Self {
        let file = File::open(path).expect("Non valid file path");
        let reader = BufReader::new(file);
        let position = Position::new(0, 1, 0);

        Self { position, reader }
    }

    pub fn move_n(&mut self, n: i64) {
        self
            .reader
            .seek_relative(n)
            .expect("Error desconocido xd?");
    }

    /// As `BufReader::peek` still is unstable
    /// here is a own implementation
    pub fn peek(&mut self) -> u64 {
        self
            .reader
            .seek(SeekFrom::Current(0))
            .expect("Could not get the current position")
    }
}
