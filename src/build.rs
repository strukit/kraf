fn emit_cfg_host() {
    println!(
        "cargo::rustc-check-cfg=cfg(host, values(\"linux\", \"mac\", \"windows\", \"wasm\", \"unknown\"))"
    );

    let target_family =
        std::env::var("CARGO_CFG_TARGET_FAMILY").expect("cargo sets CARGO_CFG_TARGET_FAMILY");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").expect("cargo sets CARGO_CFG_TARGET_OS");
    let host = if target_family == "wasm" {
        "wasm"
    } else {
        match target_os.as_str() {
            "macos" => "mac",
            "linux" => "linux",
            "windows" => "windows",
            _ => "unknown",
        }
    };

    println!("cargo::rustc-cfg=host=\"{host}\"");
    println!("cargo::rerun-if-changed=../build.rs");
}
