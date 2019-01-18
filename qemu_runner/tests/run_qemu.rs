extern crate qemu_runner;
use qemu_runner::*;

extern crate quale;
use quale::which;

#[test]
pub fn build_and_run_tests() {
	let options = CrossbuildOptions {
		tests_project_path: "../qemu_stm32_tests/".into(),
		target_arch: "thumbv7m-none-eabi".into()
	};
	let rust = crossbuild_rust_tests(&options);
	println!("Rust: {:?}", rust);

	let binaries = build_test_binaries(&options, &rust);
	println!("GCC: {:?}", binaries);

	let run_options = RunTestOptions {
		test_name_filter: None,
		qemu_path: which("qemu-system-gnuarmeclipse")
					.expect("Missing executable 'qemu-system-gnuarmeclipse', please add it to PATH.")
					.to_str().expect("Non UTF-8 path to QEMU?").into()
	};
	println!("Run options: {:?}", run_options);

	run_tests_with_qemu(&run_options, &binaries);
}