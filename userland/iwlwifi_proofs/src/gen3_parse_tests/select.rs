// NONOS Operating System (AGPL-3.0-or-later)

//! Firmware is chosen by MAC and RF type.

use crate::gen3::select::{mac_type, rf_type, select, Image, Refusal};

#[test]
fn firmware_is_chosen_by_mac_and_rf_type() {
    let so = 0x0000_0370;
    let sof = 0x0000_0430;
    let gf = 0x0010_D000;
    let hr1 = 0x0010_C000;
    let hr2 = 0x0010_A000;
    assert_eq!(mac_type(so), 0x37);
    // CSR_HW_REV_TYPE masks 0xFFF0: bits above it (for example the step
    // field some parts report in bits 16 to 19) do not change the type.
    assert_eq!(mac_type(so | 0x000F_0000), 0x37);
    assert_eq!(select(0x51F0, so | 0x0003_0000, gf).map(|r| r.0), Ok(Image::SoGf));
    assert_eq!(rf_type(gf), 0x10D);
    assert_eq!(rf_type(gf | 0xF000_0FFF), 0x10D);
    assert_eq!(select(0x51F0, so, gf).map(|r| r.0), Ok(Image::SoGf));
    assert_eq!(select(0x7AF0, sof, hr1).map(|r| r.0), Ok(Image::SoHr));
    assert_eq!(select(0x51F1, so, hr2).map(|r| r.0), Ok(Image::SoHr));
    assert_eq!(
        select(0x51F0, so, gf | 0x1000_0000),
        Err(Refusal::CdbNotBundled),
        "gf4 is not bundled"
    );
    assert_eq!(select(0x51F0, so, 0x0010_5000), Err(Refusal::RfNotBundled(0x105)), "JF");
    assert_eq!(select(0x51F0, so, 0), Err(Refusal::RfNotBundled(0)), "blank OTP");
    assert_eq!(
        select(0x2723, so, gf),
        Err(Refusal::NotSoDevice(0x2723)),
        "AX200 is not an AX210-family platform"
    );
    assert_eq!(
        select(0x51F0, 0x0000_0500, gf),
        Err(Refusal::MacNotBundled(0x50)),
        "an unknown MAC"
    );
}
