#[cfg(feature = "bindgen")]
mod builder {
  #[cfg(target_os = "linux")]
  pub const OS: &str = "linux";
  #[cfg(target_os = "linux")]
  pub const HEADER_PATH: &str = "/usr/include/pcap.h";

  #[cfg(target_os = "windows")]
  pub const OS: &str = "windows";
  #[cfg(target_os = "windows")]
  pub const HEADER_PATH: &str = "./npcap-sdk-1.03/Include/pcap.h";

  #[cfg(target_os = "macos")]
  pub const OS: &str = "macos";
  #[cfg(target_os = "macos")]
  pub const HEADER_PATH: &str = "/usr/local/opt/libpcap/include/pcap.h";

  pub fn build_bindings() {
    let bindings = bindgen::builder()
      .raw_line("#![allow(non_snake_case)]")
      .raw_line("#![allow(non_camel_case_types)]")
      .raw_line("#![allow(non_upper_case_globals)]")
      .whitelist_function("pcap.*")
      .whitelist_type("pcap.*")
      .whitelist_var("PCAP.*")
      .header(HEADER_PATH);

    #[cfg(target_os = "windows")]
    let bindings = bindings.clang_arg("-I./npcap-sdk-1.03/Include");

    #[cfg(target_os = "macos")]
    let bindings = bindings.clang_arg("-I/usr/local/opt/libpcap/include");

    let bindings = bindings.generate().unwrap();

    bindings
      .write_to_file(&format!("./src/os/{}.rs", OS))
      .unwrap();
  }
}

fn main() {
  use std::env;
  if let Ok(libdir) = env::var("PCAP_LIBDIR") {
    println!("cargo:rustc-link-search=native={}", libdir);
  } else {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-search=native=./npcap-sdk-1.03/Lib/x64");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-search=native=/usr/local/opt/libpcap/lib");
  }

  #[cfg(feature = "bindgen")]
  self::builder::build_bindings();
}
