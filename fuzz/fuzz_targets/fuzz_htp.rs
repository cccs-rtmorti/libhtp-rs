#![allow(non_snake_case)]
#![no_main]
#[macro_use] extern crate libfuzzer_sys;

extern crate htp;

use htp::test::{Test, TestConfig};
use std::env;


fuzz_target!(|data: &[u8]| {
    // dummy value for env variable to make config happy
    env::set_var("srcdir", ".");
    let mut t = Test::new(TestConfig());
    t.run_slice(data);
});
