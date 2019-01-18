use pcap_sys;

#[test]
fn test_linkage() {
  unsafe {
    println!(
      "{:#?}",
      std::ffi::CStr::from_ptr(pcap_sys::pcap_lib_version())
        .to_str()
        .unwrap()
    );
  }
}
