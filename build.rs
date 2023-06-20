use std::{env, fs, path::PathBuf};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-env-changed=PCAP_LIBDIR");
    if let Ok(libdir) = env::var("PCAP_LIBDIR") {
        println!("cargo:rustc-link-search=native={}", libdir);
    } else if let Ok(library) = pkg_config::probe_library("libpcap") {
        for path in library.link_paths {
            println!("cargo:rustc-link-search=native={}", path.display());
        }
    } else if cfg!(target_os = "macos") {
        // check if brew installed
        println!("cargo:rustc-link-search=native=/usr/local/opt/libpcap/lib");
    } else if cfg!(target_os = "windows") {
        // copy .lib files to OUT_DIR so that other packages get them
        fs::copy("./npcap-sdk/Lib/x64/wpcap.lib", out_path.join("wpcap.lib"))
            .expect("copy wpcap.lib");
        fs::copy(
            "./npcap-sdk/Lib/x64/Packet.lib",
            out_path.join("Packet.lib"),
        )
        .expect("copy Packet.lib");
        println!("cargo:rustc-link-search=native={}", out_path.display());
    }

    if let Ok(library) = pkg_config::probe_library("libpcap") {
        for name in library.libs {
            println!("cargo:rustc-link-lib={name}");
        }
    } else if cfg!(target_os = "windows") {
        // you also need to have PATH to point to "Windows\System32\Npcap" (x64)
        println!("cargo:rustc-link-lib=wpcap");
    } else {
        println!("cargo:rustc-link-lib=pcap");
    }

    println!("cargo:rerun-if-env-changed=PCAP_INCLUDEDIR");
    let dirs = if let Ok(includedir) = env::var("PCAP_INCLUDEDIR") {
        vec![includedir]
    } else if let Ok(library) = pkg_config::probe_library("libpcap") {
        library
            .include_paths
            .into_iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect()
    } else if cfg!(target_os = "windows") {
        // https://nmap.org/npcap/
        vec!["./npcap-sdk/Include".to_string()]
    } else if cfg!(target_os = "macos") {
        vec!["/usr/local/opt/libpcap/include".to_string()]
    } else {
        vec!["/usr/include".to_string()]
    };

    let args = dirs
        .into_iter()
        .map(|dir| format!("-I{dir}"))
        .collect::<Vec<_>>();
    let bindings = bindgen::builder()
        .clang_args(args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_function("pcap.*")
        .allowlist_type("pcap.*")
        .allowlist_var("PCAP.*")
        .header_contents("main.h", "#include <pcap.h>")
        .generate()
        .unwrap();

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
