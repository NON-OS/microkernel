// NONOS Operating System (AGPL-3.0-or-later)

//! The embedded firmware image, checked before the card is touched.

use super::FwLoadError;
use crate::fw::header::parse;
use crate::status;

/// The RTL8821CE firmware image, redistributed by linux-firmware and linked from
/// the tree's committed firmware set so no filesystem is needed at bring-up and a
/// clean checkout builds without staging a blob.
pub(super) const FW: &[u8] =
    include_bytes!("../../../../nonos-bootloader/firmware/realtek/rtw8821c_fw.bin");

/// Confirm the embedded blob is this chip's firmware, and note its version on
/// the console, before touching the card.
pub(super) fn check_image() -> Result<(), FwLoadError> {
    let hdr = parse(FW).ok_or(FwLoadError::BadImage)?;
    if !hdr.is_for_8821c() {
        return Err(FwLoadError::BadImage);
    }
    let (major, minor) = hdr.version();
    status::line(b"[rtl8821ce] firmware v");
    status::number(major as u32);
    status::line(b".");
    status::number(minor as u32);
    status::line(b"\n");
    Ok(())
}
