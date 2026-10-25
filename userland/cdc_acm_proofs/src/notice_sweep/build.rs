// NONOS Operating System (AGPL-3.0-or-later)
//! Notifications as a modem sends them (CDC PSTN 1.2, 6.5.4, as recalled,
//! unverified), on communications interface `comm`: SERIAL_STATE, an
//! 8-byte header of bmRequestType A1h, bNotification 20h, wIndex its
//! interface, wLength 2, then the state's 16 bits, of which 7 are
//! defined. Some states set reserved bits, and some notices say more than
//! two bytes: both leave the defined bits to read. One notice in a run is
//! told one lie, and is no SERIAL_STATE of this interface.

use super::rng::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lie {
    Sound,
    Short,
    Length,
    Type,
    Kind,
    Index,
}

pub const LIES: [Lie; 6] = [Lie::Sound, Lie::Short, Lie::Length, Lie::Type, Lie::Kind, Lie::Index];

pub fn notice(comm: u8, bits: u16, more: usize) -> Vec<u8> {
    let length = (2 + more as u16).to_le_bytes();
    let mut n = vec![0xA1, 0x20, 0, 0, comm, 0, length[0], length[1]];
    n.extend(bits.to_le_bytes());
    n.extend((0..more).map(|i| i as u8 ^ 0x5A));
    n
}

/// A sound notice and the defined bits it carries.
pub fn sound(r: &mut Rng, comm: u8) -> (Vec<u8>, u16) {
    let bits = r.range(0, 0x7F) as u16;
    let high = [0, 0, 0x80, 0xFF80][r.range(0, 3)];
    let more = [0, 0, 0, r.range(1, 40)][r.range(0, 3)];
    (notice(comm, bits | high, more), bits)
}

/// A notice told `lie`: none of its bits is to be read.
pub fn lied(r: &mut Rng, comm: u8, lie: Lie) -> Vec<u8> {
    let (mut n, _) = sound(r, comm);
    match lie {
        Lie::Short => n.truncate(r.range(0, 9)),
        Lie::Length => n[6..8].copy_from_slice(&(r.range(0, 1) as u16).to_le_bytes()),
        Lie::Type => n[0] = [0x21, 0xA0, 0x80, 0xC1][r.range(0, 3)],
        Lie::Kind => n[1] = [0x00, 0x01, 0x2A, 0x21][r.range(0, 3)],
        Lie::Index => {
            let other = [comm.wrapping_add(r.range(1, 255) as u8) as u16, comm as u16 | 0x100];
            n[4..6].copy_from_slice(&other[r.range(0, 1)].to_le_bytes());
        }
        Lie::Sound => {}
    }
    n
}
