// NONOS Operating System (AGPL-3.0-or-later)
/* The relay's side of a hop, and the replies a relay sends, built with the
 * real cell and onion code so the client's checks meet real cells. */

use crate::cell::{pack, Cell, RelayHeader, CELL_CREATED2, CELL_RELAY};
use crate::circuit::{seal, Hop};
use crate::constants::KEY_MATERIAL_BYTES;

pub const ID: u32 = 0x8000_0001;

pub fn keys(seed: u8) -> [u8; KEY_MATERIAL_BYTES] {
    core::array::from_fn(|i| seed.wrapping_mul(31).wrapping_add(i as u8))
}

/// The relay's copy of a hop: what it sends backward is keyed with Db and
/// Kb, so its forward direction is the client's backward one.
pub fn mirror(k: &[u8; KEY_MATERIAL_BYTES]) -> Hop {
    let mut m = *k;
    m[..20].copy_from_slice(&k[20..40]);
    m[20..40].copy_from_slice(&k[..20]);
    m[40..56].copy_from_slice(&k[56..72]);
    m[56..72].copy_from_slice(&k[40..56]);
    Hop::new(&m)
}

pub fn created2(reply: &[u8]) -> Cell {
    let mut cell = Cell::new(ID, CELL_CREATED2);
    cell.payload[..2].copy_from_slice(&(reply.len() as u16).to_be_bytes());
    cell.payload[2..2 + reply.len()].copy_from_slice(reply);
    cell
}

/// A relay message of `command` carrying `reply`, sent back by hop `from`
/// and wrapped by every hop between it and the client.
pub fn backward(relays: &mut [Hop], from: usize, command: u8, reply: &[u8]) -> Cell {
    let mut body = (reply.len() as u16).to_be_bytes().to_vec();
    body.extend_from_slice(reply);
    let header = RelayHeader { command, recognized: 0, stream: 0, length: body.len() as u16 };
    let mut cell = Cell::new(ID, CELL_RELAY);
    cell.payload = pack(&header, &body).expect("fits one cell");
    seal(relays, from, &mut cell.payload).expect("hop exists");
    cell
}
