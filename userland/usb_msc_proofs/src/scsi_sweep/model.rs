// NONOS Operating System (AGPL-3.0-or-later)
//! What a sound host makes of a device's answers, stated apart from the
//! driver (SBC, SPC, BOT 1.0 6.7, as recalled, unverified). Each command's
//! residue is the larger of the bytes that never came and the CSW's, a
//! CSW residue past the length a lost phase; a whole READ CAPACITY(10)
//! that passes under a residue gives the device's residues away for good.
//! Sizing: READ CAPACITY(10) must pass whole; a last LBA of 2^32 - 1 sends
//! the host to READ CAPACITY(16), whose first 12 bytes must come, and whose
//! last LBA must leave room for one more. A flush that fails is done only
//! if its sense, from the bytes sent and at least 14 of them, fixed format,
//! says ILLEGAL REQUEST. A failed command is asked its sense; it fails all
//! the same. Every failure is an I/O error. `script` is what is asked for.

use super::build::Reply;
use crate::xhci::Answer;

pub const E_IO: i32 = -5;

pub struct Host {
    pub ignore: bool,
    pub script: Vec<Answer>,
}

impl Host {
    /// One command of `len` bytes asked and answered with `r`.
    pub fn ask(&mut self, r: &Reply, len: usize, probe: bool) -> Result<(u8, u32), i32> {
        self.script.extend(r.answers());
        let (short, res) = ((len - r.sent) as u32, if self.ignore { 0 } else { r.residue });
        if res as usize > len || r.status > 1 {
            return Err(E_IO);
        }
        if probe && r.status == 0 && short == 0 && res != 0 {
            self.ignore = true;
        }
        Ok((r.status, if self.ignore || res == 0 { short } else { short.max(res) }))
    }

    /// The sense of a failed command, ILLEGAL REQUEST or not.
    pub fn illegal(&mut self, s: &Reply) -> Result<bool, i32> {
        let (0, res) = self.ask(s, 18, false)? else { return Err(E_IO) };
        let sent = &s.data[..18 - res as usize];
        if sent.len() < 14 || !matches!(sent[0] & 0x7F, 0x70 | 0x71) {
            return Err(E_IO);
        }
        Ok(sent[2] & 0x0F == 0x05)
    }

    pub fn capacity(&mut self, c10: &Reply, c16: &Reply, sense: &Reply) -> Result<(u64, u32), i32> {
        match self.ask(c10, 8, true)? {
            (0, 0) => {}
            (0, _) => return Err(E_IO),
            _ => return self.illegal(sense).and(Err(E_IO)),
        }
        let word = |d: &[u8], at: usize| u32::from_be_bytes(d[at..at + 4].try_into().unwrap());
        if word(&c10.data, 0) != u32::MAX {
            return Ok((word(&c10.data, 0) as u64 + 1, word(&c10.data, 4)));
        }
        match self.ask(c16, 32, false)? {
            (0, res) if res <= 20 => {}
            (0, _) => return Err(E_IO),
            _ => return self.illegal(sense).and(Err(E_IO)),
        }
        let last = u64::from_be_bytes(c16.data[..8].try_into().unwrap());
        Ok((last.checked_add(1).ok_or(E_IO)?, word(&c16.data, 8)))
    }

    pub fn flush(&mut self, cmd: &Reply, sense: &Reply) -> Result<(), i32> {
        match self.ask(cmd, 0, false)? {
            (0, _) => Ok(()),
            _ if self.illegal(sense)? => Ok(()),
            _ => Err(E_IO),
        }
    }
}
