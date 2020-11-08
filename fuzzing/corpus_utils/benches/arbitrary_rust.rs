#![feature(test)]
extern crate arbitrary_rust;
use arbitrary_rust::Arbitrary;
extern crate test;
use test::{Bencher, black_box};
use std::fs::File;
use std::fs;
use std::io::prelude::*;

#[bench]
fn bench_from_bytes(b: &mut Bencher) {
    let filename = "../corpus/from_csv/";
    let mut f = File::open(filename).expect(&format!("File found - {}", filename));
    let metadata = fs::metadata(filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    b.iter(|| {
        Vec::<u64>::from_bytes(buffer.clone())
    });
}