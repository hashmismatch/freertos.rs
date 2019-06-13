use std::env;
use std::fs::File;
use std::fs::{read_dir, create_dir, copy, remove_dir_all, canonicalize};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::ffi::OsString;


#[derive(Clone, Debug)]
pub struct FoundFile {
	name: String,
	absolute_path: String
}

fn find_files<F: Fn(&str) -> bool>(dir: &str, filter: F) -> Vec<FoundFile> {
	let mut ret = vec![];

	let dir_absolute = canonicalize(&dir).expect(&format!("Couldn't find the absolute path of directory: {}", &dir));
	let dir_absolute_str = dir_absolute.to_str().unwrap();

	for entry in read_dir(&dir_absolute).expect(&format!("Directory not found: {}", &dir)) {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let name = entry.file_name().into_string().unwrap();
            if let Ok(name) = entry.file_name().into_string() {
            	if filter(&name) {
            		let absolute_path = format!("{}/{}", &dir_absolute_str, &name);

            		ret.push(FoundFile { name: name, absolute_path: absolute_path });
            	}
            }
        }
    }

    ret
}

#[derive(Debug)]
pub struct CrossbuildOptions {
	pub tests_project_path: String,
	pub target_arch: String
}


#[derive(Debug)]
pub struct CrossbuiltTests {
	pub object_paths: Vec<String>,
	pub tests: Vec<String>
}

pub fn crossbuild_rust_tests(options: &CrossbuildOptions) -> CrossbuiltTests {

	let build_proj_root = {
		let p = Path::new(&options.tests_project_path);
		let mut absolute_path = ::std::env::current_dir().expect("Can't find current dir?");
		absolute_path.push(p);
		p.canonicalize().expect("Error canonicalizing")
	};

	// cross-build the tests library
	let cargo_build = Command::new("cargo")
	            .current_dir(&options.tests_project_path)
	            .arg("build")
	            .arg("--verbose")
	            .arg("--target")
	            .arg(&options.target_arch)
				.env("CARGO_INCREMENTAL", "")
	            .env("RUSTFLAGS", "--emit=obj")
				.env("RUST_TARGET_PATH", &build_proj_root.to_str().expect("Missing path to proj root for target path?"))
	            .stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
	            .output();

	let output = cargo_build.expect("Cargo build of the tests projects failed");
	if !output.status.success() {
		panic!("cargo build failed");
	}

	// grab the list of tests to compile binaries for
	let tests = {
		// slightly hackish way that requires each test entrypoint to be in its
		// own source file with a matching name

		let dir = format!("{}/src/", &options.tests_project_path);
		let tests = find_files(&dir, |n| {
			n.starts_with("test_") && n.ends_with(".rs")
		}).iter().cloned().map(|f| f.name).map(|n| { n.replace(".rs", "") }).collect();

		tests
	};

	let object_paths = {
		let active_toolchain: String = {
			let output = Command::new("rustup")
				.arg("show")
				.arg("active-toolchain")
				.stderr(Stdio::inherit())
				.output()
				.expect("Can't get a current toolchain");

			let active_toolchain = String::from_utf8_lossy(&output.stdout);
			let mut split = active_toolchain.split_whitespace();
			split.next().expect("active toolchain missing").trim().to_owned()
		};

		let rustup_sysroot = {
			let home = env::home_dir().expect("missing profile home dir");
			format!("{}/.rustup/toolchains/{}/lib/rustlib/{}/lib/",
					home.to_str().unwrap(), active_toolchain, &options.target_arch)
		};

		let mut sysroot_rlibs: Vec<FoundFile> = find_files(&rustup_sysroot, |n| {
			n.ends_with(".rlib")
		}).iter().cloned().collect();

		let tests_deps_dir = format!("{}/target/{}/debug/deps/", &options.tests_project_path, &options.target_arch);

		for sysroot_rlib in sysroot_rlibs {
			copy(sysroot_rlib.absolute_path, format!("{}/{}.o", tests_deps_dir, sysroot_rlib.name.trim_right_matches(".rlib")));
		}

		let mut test_objects: Vec<String> = find_files(&tests_deps_dir, |n| {
			n.ends_with(".o")
		}).iter().cloned().map(|f| f.absolute_path).collect();

		let mut objects = vec![];
		objects.append(&mut test_objects);
		objects
	};

	CrossbuiltTests {
		object_paths: object_paths,
		tests: tests
	}
}

#[derive(Debug, Clone)]
pub struct Stm32Test {
	pub name: String,
	pub absolute_elf_path: String
}

#[derive(Debug, Clone)]
pub struct Stm32Binaries {
	pub binaries: Vec<Stm32Test>
}

pub fn build_test_binaries(options: &CrossbuildOptions, tests: &CrossbuiltTests) -> Stm32Binaries {

	let mut binaries = vec![];
	let gcc_proj_dir = format!("{}/gcc/", options.tests_project_path);
	let test_objects = tests.object_paths.join(" ");

	for test in &tests.tests {
		let mut test_renames = "".to_string();

		if test.contains("isr_timer4") {
			test_renames.push_str(&format!("testbed_timer4_isr = {}_timer4_isr;", test));
		}

		let test_binary_build = Command::new("make")
				.current_dir(&gcc_proj_dir)
				.env("TEST_ENTRY", test.clone())
				.env("TEST_OBJECTS", &test_objects)
				.env("TEST_RENAMES", test_renames)
				.stdout(Stdio::inherit())
				.stderr(Stdio::inherit())
				.output();

		let output = test_binary_build.unwrap();
		if !output.status.success() {
			panic!(format!("GCC ARM build for '{}' failed", test));
		}

		binaries.push(Stm32Test {
			name: test.clone(),
			absolute_elf_path: canonicalize(&format!("{}/build/stm32_{}.elf", &gcc_proj_dir, &test)).unwrap().to_str().unwrap().into()
		});
	}

	Stm32Binaries {
		binaries: binaries
	}
}