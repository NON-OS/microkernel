// NONOS Operating System (AGPL-3.0-or-later)
//! A device on the modelled bus: 256 bytes behind an address pointer, as an
//! SPD EEPROM is, and the block a block read returns.

#[derive(Clone)]
pub struct Device {
    pub addr: u8,
    pub mem: [u8; 256],
    pub ptr: u8,
    /// What a block read returns, its length byte first as the device sends
    /// it, so a length outside 1 to 32 can be modelled.
    pub block: Vec<u8>,
    /// The bytes the last block write carried.
    pub written: Vec<u8>,
}

impl Device {
    pub fn eeprom(addr: u8) -> Self {
        let mut mem = [0u8; 256];
        for (i, b) in mem.iter_mut().enumerate() {
            *b = i as u8 ^ 0x5A;
        }
        Self { addr, mem, ptr: 0, block: vec![3, 0xAA, 0xBB, 0xCC], written: vec![] }
    }
}
