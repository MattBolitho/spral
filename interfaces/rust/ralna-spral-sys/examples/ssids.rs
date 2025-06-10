use ralna_spral_sys::*;
use std::{error::Error, ffi::c_void};

fn assert_inform_flag(inform: &spral_ssids_inform, mut akeep: *mut c_void, mut fkeep: *mut c_void) {
    if inform.flag < 0 {
        unsafe {
            spral_ssids_free(std::ptr::addr_of_mut!(akeep), std::ptr::addr_of_mut!(fkeep));
        }
        std::process::exit(1);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut akeep: *mut c_void = std::ptr::null_mut();
    let mut fkeep: *mut c_void = std::ptr::null_mut();

    let mut options = spral_ssids_options::default();
    let mut inform = spral_ssids_inform::default();
    unsafe { spral_ssids_default_options(std::ptr::addr_of_mut!(options)) };
    options.array_base = 1; // For Fortran indexing

    /* Data for matrix:
     * ( 2  1         )
     * ( 1  4  1    1 )
     * (    1  3  2   )
     * (       2 -1   )
     * (    1       2 ) */
    let posdef = false;
    let n = 5;
    let ptr: [i64; 6] = [1, 3, 6, 8, 9, 10];
    let row = [1, 2, 2, 3, 5, 3, 4, 4, 5];
    let val = [2.0, 1.0, 4.0, 1.0, 1.0, 3.0, 2.0, -1.0, 2.0];

    // The right-hand side with solution (1.0, 2.0, 3.0, 4.0, 5.0)
    let mut x = [4.0, 17.0, 19.0, 2.0, 12.0];

    // Perform analyse and factorise with data checking
    let check = true;
    unsafe {
        spral_ssids_analyse(
            check,
            n,
            std::ptr::null_mut(),
            ptr.as_ptr(),
            row.as_ptr(),
            std::ptr::null_mut(),
            std::ptr::addr_of_mut!(akeep),
            std::ptr::addr_of!(options),
            std::ptr::addr_of_mut!(inform),
        );
    }
    assert_inform_flag(&inform, akeep, fkeep);
    unsafe {
        spral_ssids_factor(
            posdef,
            std::ptr::null(),
            std::ptr::null(),
            val.as_ptr(),
            std::ptr::null_mut(),
            akeep,
            std::ptr::addr_of_mut!(fkeep),
            std::ptr::addr_of!(options),
            std::ptr::addr_of_mut!(inform),
        );
    }
    assert_inform_flag(&inform, akeep, fkeep);

    // Solve
    unsafe {
        spral_ssids_solve1(
            0,
            x.as_mut_ptr(),
            akeep,
            fkeep,
            std::ptr::addr_of!(options),
            std::ptr::addr_of_mut!(inform),
        );
    }
    assert_inform_flag(&inform, akeep, fkeep);
    println!("Solution: {:?}", x);

    // Determine and print the pivot order
    let mut piv_order = [0; 5];
    unsafe {
        spral_ssids_enquire_indef(
            akeep,
            fkeep,
            std::ptr::addr_of!(options),
            std::ptr::addr_of_mut!(inform),
            piv_order.as_mut_ptr(),
            std::ptr::null_mut(),
        );
    }
    println!("Pivot order: {:?}", piv_order);

    let error =
        unsafe { spral_ssids_free(std::ptr::addr_of_mut!(akeep), std::ptr::addr_of_mut!(fkeep)) };
    assert!(error == 0);

    Ok(())
}
