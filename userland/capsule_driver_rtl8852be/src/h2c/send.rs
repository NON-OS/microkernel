// NONOS Operating System (AGPL-3.0-or-later)

//! A firmware command: `rtw89_h2c_pkt_set_hdr` and `rtw89_h2c_tx`
//! (Linux v6.12 rtw89 `fw.c`, `core.c`), queued by `post`.

use super::packet::{h2c_hdr, Cmd, H2C_HDR, WD_BODY};
pub use super::post::post;
use super::Fw;
use crate::dma::DmaMem;
use crate::regs::Mmio;
use crate::ring::tx::{TxRing, SLOT};

/// The largest command body after its H2C header.
pub const BODY_MAX: usize = SLOT - WD_BODY - H2C_HDR;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SendFail {
    TooLong,
    /// The engine did not take earlier packets within `ROOM_MS`.
    NoRoom,
}

/// Send command `cmd` with `body`, asking a done acknowledgement when
/// `dack`; a receive acknowledgement is asked of every fourth.
pub fn h2c<M: DmaMem>(
    r: &impl Mmio,
    ring: &mut TxRing<M>,
    fw: &mut Fw,
    cmd: Cmd,
    body: &[u8],
    dack: bool,
) -> Result<(), SendFail> {
    h2c_acks(r, ring, fw, cmd, body, false, dack)
}

/// `rtw89_h2c_pkt_set_hdr` then `rtw89_h2c_tx`: the sequence stamped and
/// stepped, `rack` forced on every fourth, as rtw89 forces it.
pub fn h2c_acks<M: DmaMem>(
    r: &impl Mmio,
    ring: &mut TxRing<M>,
    fw: &mut Fw,
    cmd: Cmd,
    body: &[u8],
    rack: bool,
    dack: bool,
) -> Result<(), SendFail> {
    if body.len() > BODY_MAX {
        return Err(SendFail::TooLong);
    }
    let rack = rack || fw.h2c_seq % 4 == 0;
    let mut pkt = [0u8; SLOT - WD_BODY];
    pkt[..H2C_HDR].copy_from_slice(&h2c_hdr(cmd, fw.h2c_seq, body.len(), rack, dack));
    pkt[H2C_HDR..H2C_HDR + body.len()].copy_from_slice(body);
    fw.h2c_seq = fw.h2c_seq.wrapping_add(1);
    post(r, ring, &pkt[..H2C_HDR + body.len()], false)
}
