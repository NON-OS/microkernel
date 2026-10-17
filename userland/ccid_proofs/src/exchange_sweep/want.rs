// NONOS Operating System (AGPL-3.0-or-later)
//! How a lied-to exchange must end, and the failure a transfer's errno
//! earns: a halt reads as an I/O error once its pipe is cleared.

use super::model::{E_IO, E_PIPE};
use super::rng::Rng;

pub struct Want {
    pub result: Result<Vec<u8>, i32>,
    pub resets: Vec<bool>,
}

pub fn errno(r: &mut Rng, dir_in: bool) -> (Result<Vec<u8>, i32>, Vec<bool>, i32) {
    let e = [E_PIPE, E_IO, -19, -71][r.range(0, 3)];
    let pipe = e == E_PIPE;
    (Err(if pipe { E_IO } else { e }), if pipe { vec![dir_in] } else { vec![] }, e)
}
