use ralna_spral_sys::*;
use std::ptr::addr_of_mut;

fn main() {
    let mut state = SPRAL_RANDOM_INITIAL_SEED as i32;

    const MATRIX_TYPE: i32 = 0;
    const M: i32 = 4;
    const N: i32 = 5;
    const NNZ: i32 = 8;
    let mut ptr = [0; N as usize + 1];
    let mut row = [0; NNZ as usize];
    let mut val = [0.0; NNZ as usize];

    // Generate matrix
    println!("Generating a {0} x {1} non-singular matrix with {2} non-zeroes", M, N, NNZ);
    unsafe {
        spral_random_matrix_generate(
            addr_of_mut!(state),
            MATRIX_TYPE,
            M,
            N,
            NNZ,
            ptr.as_mut_ptr(),
            row.as_mut_ptr(),
            val.as_mut_ptr(),
            /* flags */ SPRAL_RANDOM_MATRIX_NONSINGULAR as i32
        )
    };

    println!("Generated matrix:");
    unsafe {
        spral_print_matrix(
            /* lines */ -1,
            MATRIX_TYPE,
            M,
            N,
            ptr.as_ptr(),
            row.as_ptr(),
            val.as_ptr(),
            0
        )
    };
}
