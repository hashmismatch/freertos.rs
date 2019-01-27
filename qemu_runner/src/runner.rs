use builder::*;

use std::process::{Command, Stdio};

use quale::which;

#[derive(Debug)]
pub struct RunTestOptions {
	pub test_name_filter: Option<String>,
	pub qemu_path: String
}

pub fn run_tests_with_qemu(options: &RunTestOptions, binaries: &Stm32Binaries) {

	let timeout = which("timeout");
	
	for test in &binaries.binaries {
		println!("Test '{}'", test.name);

		let mut qemu_cmd = if let Some(ref timeout) = timeout {
			let mut c = Command::new(timeout);
			c.arg("15s")
			 .arg(&options.qemu_path);
			c
		} else {
			Command::new(&options.qemu_path)
		};

		qemu_cmd.args(&["--verbose", "--board", "STM32F4-Discovery", "--mcu", "STM32F407VG", "--nographic"])
				.arg("--image").arg(&test.absolute_elf_path)
				.stdin(Stdio::null())
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit());

		let status = qemu_cmd.status().expect("Error running QEMU");
		if !status.success() {
			let code = status.code().unwrap();
			panic!("Test '{}' failed, exit code {}.", &test.name, code);
		}

		println!("Test '{}'... OK", &test.name);
		println!("");
	}

}
