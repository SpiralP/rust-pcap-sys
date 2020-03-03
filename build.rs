use std::{env, path::PathBuf};

#[cfg(target_os = "linux")]
pub const HEADER_PATH: &str = "/usr/include/pcap.h";

#[cfg(target_os = "windows")]
pub const HEADER_PATH: &str = "./npcap-sdk-1.04/Include/pcap.h";

#[cfg(target_os = "macos")]
pub const HEADER_PATH: &str = "/usr/local/opt/libpcap/include/pcap.h";

fn main() {
  println!("cargo:rerun-if-env-changed=PCAP_LIBDIR");
  if let Ok(libdir) = env::var("PCAP_LIBDIR") {
    println!("cargo:rustc-link-search=native={}", libdir);
  } else {
    // this "./npcap-sdk-1.04/Lib/x64" path won't work for other crates!!
    // so use PCAP_LIBDIR!
    // TODO maybe copy .libs to OUT?
    // also you need to set PATH to point to "Windows\System32\Npcap" (x64)
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-search=native=./npcap-sdk-1.04/Lib/x64");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-search=native=/usr/local/opt/libpcap/lib");
  }

  #[cfg(not(target_os = "windows"))]
  {
    println!("cargo:rustc-link-lib=pcap");
  }

  #[cfg(target_os = "windows")]
  {
    println!("cargo:rustc-link-lib=wpcap");
  }

  let bindings = bindgen::builder()
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .whitelist_function("pcap.*")
    .whitelist_type("pcap.*")
    .whitelist_var("PCAP.*")
    .header(HEADER_PATH);

  #[cfg(target_os = "windows")]
  let bindings = bindings.clang_arg("-I./npcap-sdk-1.04/Include");

  #[cfg(target_os = "macos")]
  let bindings = bindings.clang_arg("-I/usr/local/opt/libpcap/include");

  let bindings = bindings.generate().unwrap();

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
