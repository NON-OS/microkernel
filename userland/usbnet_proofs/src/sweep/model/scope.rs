// NONOS Operating System (AGPL-3.0-or-later)
//! What follows one interface. A whole interface descriptor naming
//! `number`, alternate 0, opens its scope; any other interface descriptor,
//! whole or cut short, ends it.

use super::least::whole;
use crate::bus::Notify;

fn scoped(descs: &[Vec<u8>], number: u8) -> impl Iterator<Item = &Vec<u8>> {
    let mut inside = false;
    descs.iter().filter(move |d| {
        if d[1] == 0x04 {
            inside = whole(d) && d[2] == number && d[3] == 0;
            return false;
        }
        inside
    })
}

/// Its class-specific descriptors that hold a subtype.
pub fn functional(descs: &[Vec<u8>], number: u8) -> Vec<Vec<u8>> {
    scoped(descs, number).filter(|d| d[1] == 0x24 && d.len() >= 3).cloned().collect()
}

/// Its first interrupt IN endpoint whose size and interval USB allows:
/// bits 15:13 clear, never 3 added transactions, 1 to 1024 bytes, an
/// interval of 1 or more (USB 2.0 9.6.6, as recalled, unverified).
pub fn notify(descs: &[Vec<u8>], number: u8) -> Option<Notify> {
    scoped(descs, number).find_map(|d| {
        let ok = d[1] == 0x05 && whole(d) && d[2] & 0x80 != 0 && d[3] & 0x3 == 0x3;
        let raw = u16::from_le_bytes([*d.get(4)?, *d.get(5)?]);
        let sized = raw >> 13 == 0 && (raw >> 11) & 3 != 3 && (1..=1024).contains(&(raw & 0x7FF));
        let interval = *d.get(6)?;
        (ok && sized && interval >= 1).then_some(Notify {
            address: d[2],
            max_packet: raw,
            interval,
        })
    })
}

/// A hub's status change endpoint: the first interrupt IN endpoint after
/// a whole hub-class interface descriptor (USB 2.0 11.12.1). A hub has the
/// one, so when it is out of bounds the hub has none.
pub fn status(descs: &[Vec<u8>]) -> Option<(u8, u16, u8)> {
    let mut in_hub = false;
    for d in &descs[1..] {
        match d[1] {
            0x04 => in_hub = whole(d) && d[5] == 0x09,
            0x05 if in_hub && whole(d) && d[2] & 0x80 != 0 && d[3] & 0x3 == 0x3 => {
                let raw = u16::from_le_bytes([d[4], d[5]]);
                let sized = raw >> 11 == 0 && (1..=1024).contains(&raw) && d[6] != 0;
                return sized.then(|| (d[2], raw, d[6]));
            }
            _ => {}
        }
    }
    None
}
