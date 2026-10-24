// NONOS Operating System (AGPL-3.0-or-later)
//! The TPM's register window out of the tree: a memory-mapped TIS TPM is
//! found where its `reg` says, in the bus's own cells; a disabled one, one
//! on another bus (SPI) and a tree with none answer `None`. The kernel maps
//! that window and no other, and refuses one short of five localities.

use crate::arch::fdt::find::tpm::find;
use crate::arch::fdt::Fdt;
use crate::builder::{cells, text, Builder, END};

/// A root of two-cell addresses, as QEMU's virt writes it, with one node.
fn tree(compatible: &str, status: Option<&str>) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[2])).prop("#size-cells", &cells(&[2]));
    b = b
        .begin("tpm@c000000")
        .prop("compatible", &text(compatible))
        .prop("reg", &cells(&[0, 0x0c00_0000, 0, 0x5000]));
    if let Some(status) = status {
        b = b.prop("status", &text(status));
    }
    b.end().end().token(END).finish()
}

fn window(blob: &[u8]) -> Option<(u64, u64)> {
    find(&Fdt::from_bytes(blob).unwrap()).unwrap()
}

#[test]
fn a_tis_tpm_is_found_where_its_reg_says() {
    assert_eq!(window(&tree("tcg,tpm-tis-mmio", None)), Some((0x0c00_0000, 0x5000)));
    assert_eq!(window(&tree("tcg,tpm-tis-mmio", Some("okay"))), Some((0x0c00_0000, 0x5000)));
}

#[test]
fn none_where_the_tree_has_no_usable_one() {
    assert_eq!(window(&tree("tcg,tpm-tis-mmio", Some("disabled"))), None);
    assert_eq!(window(&tree("tcg,tpm_tis-spi", None)), None);
    let empty = Builder::default().begin("").end().token(END).finish();
    assert_eq!(window(&empty), None);
}

#[test]
fn the_kernel_maps_the_platform_window_and_refuses_a_short_one() {
    let map = std::fs::read_to_string("../../src/security/tpm/mmio/map.rs").unwrap();
    assert!(map.contains("let (base, len) = crate::arch::tpm_window()?;"));
    assert!(map.contains("(len >= TPM_MMIO_SIZE as u64).then_some(base)"));
    assert!(map.contains("let base = tpm_mmio_base().ok_or(TpmError::NotPresent)?;"));
    assert!(map.contains("map_device_memory(PhysAddr::new(base), TPM_MMIO_SIZE)"));
    assert!(
        !map.contains("0xFED4_0000"),
        "the PC address is the platform's answer, not the driver's"
    );
    let crb = std::fs::read_to_string("../../src/security/tpm/crb/area.rs").unwrap();
    assert!(crb.contains(
        "(phys >= base && phys.saturating_add(size) <= base + WINDOW_SIZE).then(|| phys - base)"
    ));
    let arch = std::fs::read_to_string("../../src/arch/tpm_window.rs").unwrap();
    assert!(arch.contains("return crate::arch::aarch64::tpm_tree::get();"));
    let parse =
        std::fs::read_to_string("../../src/arch/aarch64/boot/dtb_adapter/parse/platform.rs")
            .unwrap();
    assert!(parse.contains("crate::arch::aarch64::tpm_tree::set(base, len);"));
}
