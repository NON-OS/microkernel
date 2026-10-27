// NONOS Operating System (AGPL-3.0-or-later)
//! The modelled board's efuse: a logical map with the fields the driver
//! reads, packed into physical cells the way the chip records them (a
//! two-byte header naming an 8-byte block and the words it skips, then the
//! words), and the phycap cells at 0x580. Unprogrammed cells read 0xff.

pub const PHYSICAL: usize = 0x800;
pub const RFE_TYPE: u8 = 1;
pub const XTAL_K: u8 = 0x3c;
pub const THERM: [u8; 2] = [0x1d, 0x1e];
/// The station address the card was programmed with; the driver must never
/// read it, use it or print it.
pub const EFUSE_MAC: [u8; 6] = [0x00, 0xe0, 0x4c, 0x88, 0x52, 0xbe];
const MAC_AT: usize = 0x400; // the PCIe map's address field

/// The logical map: everything 0xff but the fields set here.
pub fn logical() -> Vec<u8> {
    let mut log = vec![0xff; 2048];
    log[0x2ca] = RFE_TYPE;
    log[0x2b9] = XTAL_K;
    log[0x2d0..0x2d2].copy_from_slice(&THERM);
    for (i, b) in log[0x210..0x230].iter_mut().enumerate() {
        *b = 0x20 + i as u8; // path A TSSI offsets
    }
    for (i, b) in log[0x23a..0x25a].iter_mut().enumerate() {
        *b = 0x40 + i as u8; // path B
    }
    log[0x2d6] = 0x12; // CCK RX gain offsets, A then B nibble
    log[MAC_AT..MAC_AT + 6].copy_from_slice(&EFUSE_MAC);
    log
}

/// Pack `log` into physical cells after the 4-byte section control.
pub fn pack(log: &[u8], phy: &mut [u8]) {
    let mut at = 4;
    for (blk, b) in log.chunks(8).enumerate() {
        let skip = (0..4).filter(|&i| b[2 * i] == 0xff && b[2 * i + 1] == 0xff);
        let word_en = skip.fold(0u8, |m, i| m | 1 << i);
        if word_en == 0xf {
            continue;
        }
        phy[at] = 0x30 | (blk >> 4) as u8;
        phy[at + 1] = ((blk & 0xf) as u8) << 4 | word_en;
        at += 2;
        for i in (0..4).filter(|i| word_en & (1 << i) == 0) {
            phy[at..at + 2].copy_from_slice(&b[2 * i..2 * i + 2]);
            at += 2;
        }
    }
}

pub fn image() -> Vec<u8> {
    let mut phy = vec![0xff; PHYSICAL];
    pack(&logical(), &mut phy[..0x4c0]);
    phy[0x5e9] = 0xaa; // power calibration done (PWR_K_CHK)
    phy[0x5df] = 0x05; // thermal trim, path A
    phy[0x5dc] = 0x03; // path B
    phy
}
