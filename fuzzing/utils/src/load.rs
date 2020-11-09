use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::env;
use std::process::exit;
use arbitrary_rust::Arbitrary;
use graph_harness::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let filename = &args[2];
            let mut f = File::open(&filename).expect(&format!("File found - {}", filename));
            let metadata = fs::metadata(&filename).expect("unable to read metadata");
            let mut buffer = vec![0; metadata.len() as usize];
            f.read(&mut buffer).expect("buffer overflow");

            match args[1].as_str() {
                "from_csv" => {
                    println!("{:?}", FromCsvHarnessParams::from_bytes(buffer));
                },
                "from_vec" => {
                    println!("{:?}", FromVecHarnessParams::from_bytes(buffer));
                }
                _ => {
                    eprintln!("The available methods are `from_csv` and `from_vec`.");
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("The usage is: load from_csv ./path");
            exit(1);
        }
    }
    exit(0);
}