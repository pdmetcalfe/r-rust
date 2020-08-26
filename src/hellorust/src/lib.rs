use ::byte_strings::c_str;

use rustistics::types::{RArg, RRes, RDll};
use rustistics::binding::{RBinding, register, R1};

extern "C" fn shift(x: RArg) -> RRes {
    RRes::from(x)
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn R_init_hellorust(dll: RDll) {

    let linkage = &[RBinding::new(shift as R1, c_str!("iota_rust")),
		    RBinding::null()];
    
    register(dll, linkage);
}
