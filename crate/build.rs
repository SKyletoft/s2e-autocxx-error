use std::path::PathBuf;

fn main() -> miette::Result<()> {
	let klee = PathBuf::from("../s2e/klee/include");
	let coroutine = PathBuf::from("../s2e/libcoroutine/include");
	let cpu = PathBuf::from("../s2e/libcpu/include");
	let fsigcxx = PathBuf::from("../s2e/libfsigc++/include");
	let q = PathBuf::from("../s2e/libq/include");
	let s2e = PathBuf::from("../s2e/libs2e/include");
	let s2ecore = PathBuf::from("../s2e/libs2ecore/include");
	let s2eplugins = PathBuf::from("../s2e/libs2eplugins/include");
	let tcg = PathBuf::from("../s2e/libtcg/include");
	let vmi = PathBuf::from("../s2e/libvmi/include");
	let config_host = PathBuf::from("../s2e/tools/lib/X8664BitcodeLibrary");

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

	println!("cargo:rerun-if-changed=src/lib.rs");

	Ok(())
}
