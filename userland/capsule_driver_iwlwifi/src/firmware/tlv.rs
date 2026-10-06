use crate::constants::{
    FW_API_VERSION_MASK, IWL_FW_MAGIC, MAX_FW_API_VERSION, MIN_FW_API_VERSION,
};

// iwl_ucode_tlv_type. The runtime image is 19; 20 is INIT and 21 WoWLAN, so
// taking 20/21 staged the init and wake-on-LAN images and never the runtime.
pub const TLV_SEC_RT: u32 = 19;
pub const TLV_SEC_INIT: u32 = 20;
pub const TLV_PAGING: u32 = 32;

/*
 * iwl_tlv_ucode_header: zero, magic, a 64-byte human-readable name, then the
 * version and build words, 8 reserved bytes, and the TLVs from byte 88.
 * Reading the version at 8 read the name ("Core", "rele", "jenk" on the
 * bundled images), so every image was refused as an unknown API.
 */
pub const TLV_HEADER_LEN: usize = 88;
const VER_OFF: usize = 72;
const BUILD_OFF: usize = 76;

#[derive(Clone, Copy)]
pub struct Header {
    pub major: u16,
    pub minor: u16,
    pub api: u16,
    pub build: u32,
}

pub fn parse_header(data: &[u8]) -> Option<Header> {
    if data.len() < TLV_HEADER_LEN {
        return None;
    }
    let zero = le32(data, 0)?;
    let magic = le32(data, 4)?;
    if zero != 0 || magic != IWL_FW_MAGIC {
        return None;
    }
    let ver = le32(data, VER_OFF)?;
    let api = (ver & FW_API_VERSION_MASK) as u16;
    if !(MIN_FW_API_VERSION..=MAX_FW_API_VERSION).contains(&api) {
        return None;
    }
    Some(Header {
        major: ((ver >> 24) & 0xFF) as u16,
        minor: ((ver >> 16) & 0xFF) as u16,
        api,
        build: le32(data, BUILD_OFF)?,
    })
}

pub fn le32(data: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(data.get(off..off + 4)?.try_into().ok()?))
}
