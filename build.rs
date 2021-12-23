#[cfg(all(windows, not(feature = "dynamic")))]
fn main() {
	use std::env;
	use std::path::PathBuf;
	if let Ok(path) = env::var("SCITER_STATIC_LIBRARY") {
		let lib_dir = PathBuf::from(path);
		println!("cargo:rustc-link-search=native={}", lib_dir.display());

		add_msvc_dirs();

		if cfg!(feature = "nightly") {
			// -bundle allow msvc linker link the library with ltcg
			// this is a nightly feature now: https://github.com/rust-lang/rust/issues/81490
			println!("cargo:rustc-link-lib=static:-bundle={}", "sciter.static");
			if cfg!(feature = "skia") {
				println!("cargo:rustc-link-lib=static:-bundle={}", "atls");
			}
		} else {
			println!("cargo:rustc-link-lib=static={}", "sciter.static");
			if cfg!(feature = "skia") {
				println!("cargo:rustc-link-lib=static={}", "atls");
			}
		}

		println!("cargo:rustc-link-lib={}", "Comdlg32");
		println!("cargo:rustc-link-lib={}", "windowscodecs");
		println!("cargo:rustc-link-lib={}", "Wininet");
		println!("cargo:rustc-link-lib=static={}", "Winspool");
	} else {
		println!("cargo:warning=Set SCITER_STATIC_LIBRARY to link static library");
	}
}

#[cfg(all(windows, not(feature = "dynamic")))]
fn add_msvc_dirs() {
	use std::env;
	use std::ffi::OsStr;
	use std::path::Path;
	use std::path::PathBuf;

	fn lib_subdir(target: &str) -> Option<&'static str> {
		let arch = target.split('-').next().unwrap();
		match arch {
			"i586" | "i686" => Some("x86"),
			"x86_64" => Some("x64"),
			"arm" | "thumbv7a" => Some("arm"),
			"aarch64" => Some("arm64"),
			_ => None,
		}
	}

	let target = env::var("TARGET").unwrap();
	let mut atlmfc_path = PathBuf::new();
	let mut atlmfc_included = false;
	let tool = cc::windows_registry::find_tool(&target, "link.exe").unwrap();
	for (kind, lib_paths) in tool.env().iter() {
		if kind.as_os_str() != OsStr::new("LIB") {
			continue;
		}
		for path in env::split_paths(lib_paths) {
			if !path.exists() {
				continue;
			}
			let sub = Path::new("lib").join(lib_subdir(&target).unwrap());
			if path.ends_with(sub) {
				atlmfc_path = path
					.parent()
					.and_then(|p| p.parent())
					.unwrap()
					.join(r"atlmfc\lib")
					.join(lib_subdir(&target).unwrap());
			}
			let sub = Path::new(r"atlmfc\lib");
			if path.ends_with(sub) || path.parent().map_or(false, |p| p.ends_with(sub)) {
				atlmfc_included = true;
			}
			println!("cargo:rustc-link-search=native={}", path.to_string_lossy());
		}
	}
	if !atlmfc_included && atlmfc_path.exists() {
		println!("cargo:rustc-link-search=native={}", atlmfc_path.to_string_lossy());
	}
}

#[cfg(not(all(windows, not(feature = "dynamic"))))]
fn main() {}
