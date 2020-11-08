#[macro_use] extern crate honggfuzz;
extern crate graph_harness;
use graph_harness::*;
use arbitrary_rust::Arbitrary;

fn main() {
    loop {
        fuzz!(|data: Vec<u8>| {
            // We ignore this error because we execute only the fuzzing to find
            // the panic situations that are NOT just errors, but unhandled errors.
            let _ = from_vec_harness(FromVecHarnessParams::from_bytes(data));
        });
    }
}