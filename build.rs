use std::{env, path::PathBuf};

#[cfg(target_os = "linux")]
pub const INCLUDE_PATH: &str = "/usr/include";

// https://nmap.org/npcap/
#[cfg(target_os = "windows")]
pub const INCLUDE_PATH: &str = "./npcap-sdk/Include";

#[cfg(target_os = "macos")]
pub const INCLUDE_PATH: &str = "/usr/local/opt/libpcap/include";

fn main() {
  println!("cargo:rerun-if-env-changed=PCAP_LIBDIR");
  if let Ok(libdir) = env::var("PCAP_LIBDIR") {
    println!("cargo:rustc-link-search=native={}", libdir);
  } else {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-search=native=./npcap-sdk/Lib/x64");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-search=native=/usr/local/opt/libpcap/lib");
  }

  #[cfg(not(target_os = "windows"))]
  {
    // you also need to have PATH to point to "Windows\System32\Npcap" (x64)
    println!("cargo:rustc-link-lib=pcap");
  }

  #[cfg(target_os = "windows")]
  {
    println!("cargo:rustc-link-lib=wpcap");
  }

  let bindings = bindgen::builder()
    .clang_args(vec!["-I", INCLUDE_PATH])
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .whitelist_function("pcap.*")
    .whitelist_type("pcap.*")
    .whitelist_var("PCAP.*")
    .header(format!("{}/pcap.h", INCLUDE_PATH))
    .generate()
    .unwrap();

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}
