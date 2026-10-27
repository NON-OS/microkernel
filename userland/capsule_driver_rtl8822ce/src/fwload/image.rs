// NONOS Operating System (AGPL-3.0-or-later)

//! The embedded RTL8822C firmware, checked before the card is touched. The
//! image is linux-firmware's `rtw88/rtw8822c_fw.bin` (firmware/README.md),
//! pinned by sha256 in scripts/baselines/upstream.sha256 and by the proofs.

use super::FwLoadError;
use crate::fw::header::{parse, FwHeader};
use crate::status;

/// The RTL8822C firmware image, linked from the tree's committed firmware set.
pub(super) const FW: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/realtek/rtw8822c_fw.bin");

/// "[rtl8822ce] firmware v<major>.<minor>", whole, into `out`.
pub fn version_line(hdr: &FwHeader, out: &mut [u8; 48]) -> usize {
    let (major, minor) = hdr.version();
    let mut digits = [0u8; 10];
    let mut n = 0;
    let mut put = |b: &[u8]| {
        out[n..n + b.len()].copy_from_slice(b);
        n += b.len();
    };
    put(b"[rtl8822ce] firmware v");
    put(status::digits::decimal(u32::from(major), &mut digits));
    put(b".");
    put(status::digits::decimal(u32::from(minor), &mut digits));
    put(b"\n");
    n
}

/// Confirm the embedded blob is this chip's firmware, and say its version.
pub(super) fn check_image() -> Result<(), FwLoadError> {
    let hdr = parse(FW).ok_or(FwLoadError::BadImage)?;
    if !hdr.is_for_8822c() {
        return Err(FwLoadError::BadImage);
    }
    let mut out = [0u8; 48];
    let n = version_line(&hdr, &mut out);
    status::line(&out[..n]);
    Ok(())
}
