use bindgen;

#[cfg(target_os = "linux")]
mod vars {
  pub const OS: &str = "linux";
  pub const HEADER_PATH: &str = "/usr/include/pcap.h";
}

#[cfg(target_os = "windows")]
mod vars {
  pub const OS: &str = "windows";
  pub const HEADER_PATH: &str = "./npcap-sdk-1.01/Include/pcap.h";
}

fn main() {
  use self::vars::*;

  let bindings = bindgen::builder()
    .raw_line("#![allow(non_snake_case)]")
    .raw_line("#![allow(non_camel_case_types)]")
    .raw_line("#![allow(non_upper_case_globals)]")
    .whitelist_function("pcap.*")
    .whitelist_function("PCAP.*")
    .header(HEADER_PATH);

  #[cfg(target_os = "windows")]
  let bindings = bindings.clang_arg("-I./npcap-sdk-1.01/Include");

  let bindings = bindings.generate().unwrap();

  bindings
    .write_to_file(&format!("./src/os/{}.rs", OS))
    .unwrap();
}
