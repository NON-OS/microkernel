// NONOS Operating System (AGPL-3.0-or-later)
//! Spin-table release addresses: one per CPU, in `cpus::find`'s order, and a
//! malformed one refused rather than read short.

use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::find::cpus::{find, release_addresses};
use crate::arch::fdt::Fdt;
use crate::builder::{cells, text, Builder, END};

/// `/cpus` with one node per entry: (reg, enable-method, cpu-release-addr).
fn tree(entries: &[(Option<u32>, Option<&str>, &[u32])]) -> Vec<u8> {
    let mut b = Builder::default().begin("");
    b = b.prop("#address-cells", &cells(&[2])).prop("#size-cells", &cells(&[2]));
    b = b.begin("cpus").prop("#address-cells", &cells(&[1])).prop("#size-cells", &cells(&[0]));
    for (i, (reg, method, release)) in entries.iter().enumerate() {
        b = b.begin(&format!("cpu@{i}")).prop("device_type", &text("cpu"));
        if let Some(reg) = reg {
            b = b.prop("reg", &cells(&[*reg]));
        }
        if let Some(method) = method {
            b = b.prop("enable-method", &text(method));
        }
        if !release.is_empty() {
            b = b.prop("cpu-release-addr", &cells(release));
        }
        b = b.end();
    }
    b.end().end().token(END).finish()
}

fn releases(blob: &[u8]) -> Result<Vec<Option<u64>>, FdtError> {
    let mut out = [None; 8];
    let n = release_addresses(&Fdt::from_bytes(blob).unwrap(), &mut out)?;
    Ok(out[..n].to_vec())
}

#[test]
fn psci_cpus_have_no_release_word_and_spin_table_ones_do() {
    let blob = tree(&[
        (Some(0), Some("psci"), &[0, 0xd8]),
        (Some(1), Some("spin-table"), &[0x1, 0x8000_0000]),
        (Some(2), Some("spin-table"), &[]),
    ]);
    assert_eq!(releases(&blob), Ok(vec![None, Some(0x1_8000_0000), None]));
}

#[test]
fn a_cpu_find_skips_is_skipped_here_too() {
    let blob = tree(&[
        (Some(0), None, &[]),
        (None, Some("spin-table"), &[0, 0xe0]),
        (Some(2), Some("spin-table"), &[0, 0xe8]),
    ]);
    let mut ids = [0u64; 8];
    assert_eq!(find(&Fdt::from_bytes(&blob).unwrap(), &mut ids), Ok(2));
    assert_eq!(&ids[..2], &[0, 2]);
    assert_eq!(releases(&blob), Ok(vec![None, Some(0xe8)]));
}

#[test]
fn a_release_word_not_two_cells_is_refused() {
    for bad in [&[0xd8u32][..], &[0, 0, 0xd8]] {
        let blob = tree(&[(Some(1), Some("spin-table"), bad)]);
        assert_eq!(releases(&blob), Err(FdtError::BadCells));
    }
}
