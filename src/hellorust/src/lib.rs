use std::ptr;
use std::mem::transmute;

use r_sys;

extern "C" fn shift(x: r_sys::SEXP) -> r_sys::SEXP {
    x
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn R_init_hellorust(dll: *mut r_sys::DllInfo) {

    let linkage: &[r_sys::R_CallMethodDef] = &[
	r_sys::R_CallMethodDef {name: b"iota_rust\0".as_ptr() as *const i8,
				fun: unsafe {transmute(shift as usize)},
				numArgs: 1},
	r_sys::R_CallMethodDef {name: ptr::null(),
				fun: None,
				numArgs: 0},
    ];

    unsafe {
	r_sys::R_registerRoutines(dll, ptr::null(), linkage.as_ptr(),
				  ptr::null(), ptr::null());
	r_sys::R_useDynamicSymbols(dll, r_sys::Rboolean::TRUE);
	r_sys::R_forceSymbols(dll, r_sys::Rboolean::FALSE);
    }
}
