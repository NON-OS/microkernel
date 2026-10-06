use std::path::Path;

/// The kernel and userland, and the crates every gate runs: the loader, the
/// attestation path parser, the boot measurement check and the device proof.
pub fn root_dirs() -> &'static [&'static str] {
    &[
        "src",
        "userland",
        "nonos-bootloader/src",
        "nonos-attest-path/src",
        "nonos-boot-measure/src",
        "nonos-device-attest/src",
    ]
}

pub fn skip(path: &Path) -> bool {
    let p = path.to_string_lossy();
    p.contains("/target/")
        || p.contains("_proofs/")
        || p.contains("/boot_proofs/")
        || p.contains("/tests/")
        // A cfg(test) module in the conventional src/tests.rs layout. Test
        // code is allowed to unwrap: a failing assertion is the point, and
        // this check is about what ships.
        || p.ends_with("/tests.rs")
        // Vendored crates and unmodified upstream program source are third
        // party. They are not held to the NONOS no-panic production standard;
        // the point of vendoring is to build them as published.
        || p.contains("/vendor/")
        || p.contains("/upstream-src/")
        || p.ends_with("/build.rs")
}

pub fn is_rust(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "rs")
}
