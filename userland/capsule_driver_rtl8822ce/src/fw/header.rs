// NONOS Operating System (AGPL-3.0-or-later)

//! The 64-byte header of an rtw88 firmware image (`struct rtw_fw_hdr`,
//! fw.h): signature, version, the H2C command format, and the size and
//! on-chip address of the DMEM, IMEM and EMEM sections the body is split
//! into. The layout is the RTL8821CE's parse; the signature this driver
//! takes is the 8822C's.

/// The header's length, and the offset of the firmware body.
pub const FW_HDR_LEN: usize = 64;
/// The signature an RTL8822C image carries in its first two bytes.
pub const SIGNATURE_8822C: u16 = 0x8822;

#[derive(Clone, Copy)]
pub struct FwHeader {
    pub signature: u16,
    pub version: u16,
    pub subversion: u8,
    /// Memory-usage flags; bit 4 says the image carries an EMEM section.
    pub mem_usage: u8,
    pub h2c_fmt_ver: u16,
    pub dmem_addr: u32,
    pub dmem_size: u32,
    pub imem_addr: u32,
    pub imem_size: u32,
    pub emem_addr: u32,
    pub emem_size: u32,
}

impl FwHeader {
    /// An 8822C image the 8051 can run: its signature, and an H2C format.
    pub fn is_for_8822c(&self) -> bool {
        self.signature == SIGNATURE_8822C && self.h2c_fmt_ver != 0
    }

    /// The version and subversion, for the console.
    pub fn version(&self) -> (u16, u8) {
        (self.version, self.subversion)
    }
}

/// The header, or `None` for an image shorter than one.
pub fn parse(fw: &[u8]) -> Option<FwHeader> {
    let b = fw.get(..FW_HDR_LEN)?;
    let le16 = |o: usize| u16::from_le_bytes([b[o], b[o + 1]]);
    let le32 = |o: usize| u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
    Some(FwHeader {
        signature: le16(0x00),
        version: le16(0x04),
        subversion: b[0x06],
        mem_usage: b[0x18],
        h2c_fmt_ver: le16(0x1C),
        dmem_addr: le32(0x20),
        dmem_size: le32(0x24),
        imem_size: le32(0x30),
        emem_size: le32(0x34),
        emem_addr: le32(0x38),
        imem_addr: le32(0x3C),
    })
}
