// NONOS Operating System (AGPL-3.0-or-later)
//! The capabilities each USB NIC capsule's manifest asks for, read from the
//! Capsule.mk the build signs: IPC, Memory and Crypto, with Debug only as
//! the optional bit a serial-debug build grants. A required capability is
//! installed whatever the spawn site asks, so an extra bit here ships.

/// IPC 0x08, Memory 0x10, Crypto 0x20 (src/capabilities/types/defs.rs).
const REQUIRED: u64 = 0x08 | 0x10 | 0x20;
/// Debug, 1 << 8.
const OPTIONAL: u64 = 1 << 8;

const MANIFESTS: [(&str, &str); 3] = [
    ("cdc_ecm", include_str!("../../capsule_driver_cdc_ecm/Capsule.mk")),
    ("cdc_ncm", include_str!("../../capsule_driver_cdc_ncm/Capsule.mk")),
    ("rndis", include_str!("../../capsule_driver_rndis/Capsule.mk")),
];

/// The value of the one `name := 0x...` line in `mk`.
fn mask(mk: &str, name: &str) -> Option<u64> {
    let mut lines = mk.lines().filter(|l| l.split_whitespace().next() == Some(name));
    let line = lines.next()?;
    assert!(lines.next().is_none(), "{name} set twice");
    let hex = line.split(":=").nth(1)?.trim().strip_prefix("0x")?;
    u64::from_str_radix(hex, 16).ok()
}

#[test]
fn each_usb_nic_asks_for_ipc_memory_and_crypto_and_debug_only_as_optional() {
    for (name, mk) in MANIFESTS {
        assert_eq!(mask(mk, "CAPSULE_REQUIRED_CAPS"), Some(REQUIRED), "{name}");
        assert_eq!(mask(mk, "CAPSULE_OPTIONAL_CAPS"), Some(OPTIONAL), "{name}");
        assert_eq!(mask(mk, "CAPSULE_CAPS_CEILING"), None, "{name}: the ceiling is their union");
    }
}
