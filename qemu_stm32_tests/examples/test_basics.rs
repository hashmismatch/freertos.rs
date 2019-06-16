#![no_main]
#![no_std]

#[macro_use]
extern crate qemu_stm32_tests;

use qemu_stm32_tests::prelude::v1::*;

freertos_rs_test!(TestBasics);

pub struct TestBasics;
impl Test for TestBasics {
	fn run<T: Testbed>(tb: &T) {
		let check = shim_sanity_check();
		if check.is_err() {
			T::debug_print(&format!("Shim sanity check failed: {:?}", check));
			T::exit_test(1);
		}

		T::debug_print("Type sizes are OK!");

		T::exit_test(0);
	}
}