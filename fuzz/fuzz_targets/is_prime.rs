#![no_main]
use libfuzzer_sys::fuzz_target;
use fhe_util::is_prime;

fuzz_target!(|value: u64| {
    is_prime(value);
});