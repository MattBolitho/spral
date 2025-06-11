use ralna_spral_sys::*;
use std::{error::Error, ptr::addr_of_mut};

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = SPRAL_RANDOM_INITIAL_SEED as i32;
    let initial_seed = state;

    println!("Some random values");
    println!("Sample Unif(-1,1)               = {}", unsafe {
        spral_random_real(addr_of_mut!(state), /* positive */ false)
    });
    println!("Sample Unif(0,1)                = {}", unsafe {
        spral_random_real(addr_of_mut!(state), /* positive */ true)
    });
    println!("Sample Unif(1, ..., 20)         = {}", unsafe {
        spral_random_integer(addr_of_mut!(state), 20)
    });
    println!("Sample Unif(1, ..., 20*INT_MAX) = {}", unsafe {
        spral_random_long(addr_of_mut!(state), 20 as i64 * i32::MAX as i64)
    });
    println!("Sample B(1,0.5)                 = {}", unsafe {
        spral_random_logical(addr_of_mut!(state))
    });

    state = initial_seed;

    println!("\nThe same random values again");
    println!("Sample Unif(-1,1)               = {}", unsafe {
        spral_random_real(addr_of_mut!(state), /* positive */ false)
    });
    println!("Sample Unif(0,1)                = {}", unsafe {
        spral_random_real(addr_of_mut!(state), /* positive */ true)
    });
    println!("Sample Unif(1, ..., 20)         = {}", unsafe {
        spral_random_integer(addr_of_mut!(state), 20)
    });
    println!("Sample Unif(1, ..., 20*INT_MAX) = {}", unsafe {
        spral_random_long(addr_of_mut!(state), 20 as i64 * i32::MAX as i64)
    });
    println!("Sample B(1,0.5)                 = {}", unsafe {
        spral_random_logical(addr_of_mut!(state))
    });

    Ok(())
}
