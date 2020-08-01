use std::ptr;

#[repr(C)]
pub struct SEXPREC {
    _private: [u8;0]
}
pub type SEXP = *mut SEXPREC;

#[repr(C)]
pub struct DllInfo {
    _private: [u8;0]
}

#[repr(C)]
struct CMethodDef {
    _private: [u8;0]
}

#[repr(C)]
struct CallMethodDef {
    name: *const u8,
    func: *const libc::c_void,
    num_args: libc::c_int
}

#[link(name="R")]
#[allow(non_snake_case)]
extern "C" {
    fn R_registerRoutines(_: *mut DllInfo,
			  _: *const CMethodDef,
			  _: *const CallMethodDef,
			  _: *const CMethodDef,
			  _: *const CMethodDef) -> libc::c_int;
    fn R_useDynamicSymbols(_: *mut DllInfo, _: libc::c_int) -> libc::c_int;
    fn R_forceSymbols(_: *mut DllInfo, _: libc::c_int) -> libc::c_int;
}

#[link(name="R")]
extern "C" {
    static R_NilValue: SEXP;
}

extern "C" fn shift(x: SEXP) -> SEXP {
    unsafe { R_NilValue }
}

const LINKAGE: &'static [CallMethodDef] = &[
    CallMethodDef {name: b"iota_rust\0".as_ptr(),
		   func: shift as *const libc::c_void,
		   num_args: 1},
    CallMethodDef {name: ptr::null(),
		   func: ptr::null(),
		   num_args: 0},
];

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn R_init_hellorust(dll: *mut DllInfo) {
    unsafe {
	R_registerRoutines(dll, ptr::null(), LINKAGE.as_ptr(),
			   ptr::null(), ptr::null());
	R_useDynamicSymbols(dll, 1);
	R_forceSymbols(dll, 0);
    }
}
