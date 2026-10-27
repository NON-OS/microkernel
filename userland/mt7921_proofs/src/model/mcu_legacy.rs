// NONOS Operating System (AGPL-3.0-or-later)

//! The download and offload commands as the MCU answers them: the patch
//! semaphore (`PATCH_NOT_DL_SEM_SUCCESS`, or `PATCH_IS_DL` once patched;
//! `PATCH_REL_SEM_SUCCESS` on a release it held), an announcement opening
//! a piece, the patch finish, the start (the RAM code then runs), the
//! capability query; anything else a generic reply.

use super::chip::Chip;
use super::mcu::Piece;
use super::mcu_cmds::le32;
use super::reply;

/// A factory address the capability reply carries, which no line, frame
/// or command of the driver may ever hold.
pub const FACTORY: [u8; 6] = [0x00, 0x0c, 0xe7, 0x5a, 0x5b, 0x5c];

impl Chip {
    pub(super) fn legacy(&mut self, cid: u8, seq: u8, body: &[u8]) -> Option<Vec<u8>> {
        let m = &mut self.mcu;
        Some(match cid {
            0x10 if le32(body, 0) == 1 => {
                m.sem_held = true;
                reply::byte(seq, self.faults.sem.unwrap_or(if m.patched { 1 } else { 2 }))
            }
            0x10 => {
                let held = core::mem::replace(&mut m.sem_held, false);
                reply::byte(seq, self.faults.sem.unwrap_or(if held { 3 } else { 0 }))
            }
            0x01 | 0x05 => {
                let (addr, len, mode) = (le32(body, 0), le32(body, 4), le32(body, 8));
                m.piece = Some(Piece { cmd: cid, addr, len, mode, bytes: Vec::new() });
                reply::generic(seq, 0x01)
            }
            0x07 => {
                m.patched = self.faults.finish == 0;
                reply::byte(seq, self.faults.finish)
            }
            0x02 => {
                (m.started, m.running) = (Some((le32(body, 0), le32(body, 4))), true);
                reply::generic(seq, 0x01)
            }
            0x8a => reply::event(0x01, seq, 0, 0, &reply::capab(FACTORY)),
            0xed => reply::generic(seq, 0xed),
            _ => reply::generic(seq, 0x01),
        })
    }
}
