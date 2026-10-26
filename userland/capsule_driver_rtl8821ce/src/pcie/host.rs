// NONOS Operating System (AGPL-3.0-or-later)

//! The host's Link Control register, found by walking the capability list of
//! the card's configuration space through the broker's reads.

/// Status register: the function has a capability list.
const STATUS: u32 = 0x06;
const STATUS_CAP_LIST: u32 = 1 << 4;
const CAP_PTR: u32 = 0x34;
/// The PCI Express capability, and Link Control within it.
const CAP_ID_EXP: u32 = 0x10;
const LNKCTL: u32 = 0x10;
/// More links than a 256-byte space can hold distinct capabilities: a list
/// that runs past this loops.
const MAX_CAPS: usize = 48;

/// Link Control, read through `read(offset, width)`; `None` when the card
/// has no PCI Express capability, the list loops or leaves the header, or a
/// read is refused.
pub fn link_control(read: impl Fn(u32, u32) -> Option<u32>) -> Option<u16> {
    if read(STATUS, 2)? & STATUS_CAP_LIST == 0 {
        return None;
    }
    let mut at = read(CAP_PTR, 1)? & 0xFC;
    for _ in 0..MAX_CAPS {
        if at < 0x40 {
            return None;
        }
        let head = read(at, 2)?;
        if head & 0xFF == CAP_ID_EXP {
            return read(at + LNKCTL, 2).map(|v| v as u16);
        }
        at = (head >> 8) & 0xFC;
    }
    None
}
