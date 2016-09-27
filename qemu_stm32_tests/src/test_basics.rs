use super::*;
use freertos_rs::*;

#[no_mangle]
pub extern fn test_basics() -> i8 {
	let check = shim_sanity_check();
	if check.is_err() {
		debug_print(&format!("Shim sanity check failed: {:?}", check));
		return 1;
	}

	0
}