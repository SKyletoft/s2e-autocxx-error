use std::path::PathBuf;

fn main() -> miette::Result<()> {
	// s2e subprojects
	let klee = PathBuf::from("../klee/include");
	let coroutine = PathBuf::from("../libcoroutine/include");
	let cpu = PathBuf::from("../libcpu/include");
	let fsigcxx = PathBuf::from("../libfsigc++/include");
	let q = PathBuf::from("../libq/include");
	let s2e = PathBuf::from("../libs2e/include");
	let s2ecore = PathBuf::from("../libs2ecore/include");
	let s2eplugins = PathBuf::from("../libs2eplugins/include");
	let tcg = PathBuf::from("../libtcg/include");
	let vmi = PathBuf::from("../libvmi/include");

	// There are 5 files with the same name in s2e. 3 x86 and 2 x86_64
	// versions. I'm using the first x86_64 version.
	let config_host = PathBuf::from("../tools/lib/X8664BitcodeLibrary");

	// Dependencies
	let glibc = PathBuf::from(env!("GLIBC_PATH"));
	let gcc_libs = PathBuf::from(env!("GCCLIBS_PATH"));
	let gcc_libs_l = PathBuf::from(env!("GCCLIBS_PATH_L"));
	let clang_libs = PathBuf::from(env!("CLANGLIBS_PATH"));
	let boost_libs = PathBuf::from(env!("BOOST_PATH"));
	let llvm_libs = PathBuf::from(env!("LLVM_PATH"));

	let helpers = PathBuf::from("./cpp_src");

	// Breaks on reordering!!
	let libraries = [
		&klee,
		&coroutine,
		&cpu,
		&fsigcxx,
		&q,
		&s2e,
		&s2ecore,
		&s2eplugins,
		&tcg,
		&vmi,
		&config_host,
		&helpers,
		&gcc_libs,
		&gcc_libs_l,
		&clang_libs,
		&boost_libs,
		&llvm_libs,
		&glibc,
	];

	autocxx_build::Builder::new(
		"src/lib.rs",
		&libraries
	)
	.extra_clang_args(&[
		"-DBOOST_BIND_GLOBAL_PLACEHOLDERS=1",
		"-DTARGET_PAGE_BITS=12",
		"-DSE_RAM_OBJECT_BITS=12",
		&format!("-DSE_RAM_OBJECT_MASK={}", !11),
		"-w",
		"-std=gnu++17",
	])
	.build()?;

	cc::Build::new()
		.cpp(true)
		.compiler("clang++")
		.includes(&libraries)
		.warnings(false)
		.extra_warnings(false)
		.flag("-w")
		.flag("-std=gnu++17")
		.define("BOOST_BIND_GLOBAL_PLACEHOLDERS", "1")
		.define("TARGET_PAGE_BITS", "12")
		.define("SE_RAM_OBJECT_BITS", "12")
		.define(
			"SE_RAM_OBJECT_MASK",
			Some((!11).to_string().as_str()),
		)
		.file("cpp_src/RustPlugin.cpp")
		.file("cpp_src/Helpers.cpp")
		.compile("rustplugin");

	println!("cargo:rerun-if-changed=src/lib.rs");

	Ok(())
}
