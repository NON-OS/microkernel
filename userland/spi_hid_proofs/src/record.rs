// NONOS Operating System (AGPL-3.0-or-later)
//! A SERIAL reply built as F 65 lays it out: a touchpad on the SPI bus of
//! the controller keyed `KEY`, its line pin 7 on GPIO controller `_UID` 2.

pub const KEY: u64 = 1 << 63 | 0x42;

pub struct Rec {
    pub kind: u8,
    pub flags: u8,
    pub chip_select: u16,
    pub hz: u32,
    pub gpio_uid: u64,
    pub key: u64,
    pub spi: [u8; 4],
    pub present: u8,
    pub hid_flags: u16,
}

impl Default for Rec {
    fn default() -> Self {
        Self {
            kind: 2,
            flags: 1 << 3 | 1 << 6,
            chip_select: 0,
            hz: 17_000_000,
            gpio_uid: 2,
            key: KEY,
            spi: [8, 1, 1, 0],
            present: 0b11_1111,
            hid_flags: 0,
        }
    }
}

pub fn bytes(r: &Rec) -> Vec<u8> {
    let mut b = Vec::new();
    b.extend(0i32.to_le_bytes());
    b.extend(1u32.to_le_bytes());
    b.extend([r.kind, r.flags]);
    b.extend(r.chip_select.to_le_bytes());
    b.extend(r.hz.to_le_bytes());
    b.extend(0u16.to_le_bytes());
    b.extend(7u32.to_le_bytes());
    b.extend(u64::MAX.to_le_bytes());
    b.extend(1u64.to_le_bytes());
    b.extend(r.gpio_uid.to_le_bytes());
    for s in ["\\_SB.PC00.SPI1.TPD0", "PNP0C51", "\\_SB.PC00.SPI1", "\\_SB.GPI0"] {
        b.push(s.len() as u8);
        b.extend(s.bytes());
    }
    b.extend(r.key.to_le_bytes());
    b.extend(r.spi);
    b.push(r.present);
    for a in [0x1000u32, 0x1004, 0x2000] {
        b.extend(a.to_le_bytes());
    }
    b.extend([0x0B, 0x02]);
    b.extend(r.hid_flags.to_le_bytes());
    b
}
