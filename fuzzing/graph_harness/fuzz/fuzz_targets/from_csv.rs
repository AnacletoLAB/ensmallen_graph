#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate graph_harness;
use graph_harness::*;
use arbitrary_rust::Arbitrary;

fuzz_target!(|data: Vec<u8>| {
    // We ignore this error because we execute only the fuzzing to find
    // the panic situations that are NOT just errors, but unhandled errors.
            let _ = from_csv_harness(FromCsvHarnessParams::from_bytes(data));
});