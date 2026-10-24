// NONOS Operating System (AGPL-3.0-or-later)
//! What a device answers one command with: the bytes it has to send (SBC
//! READ CAPACITY(10) and (16), SPC fixed-format sense, as recalled,
//! unverified), how many it sends, and its CSW's status and residue. An
//! honest device sends all, with residue 0; READ CAPACITY(10) names a last
//! LBA under 2^32 - 1 and a block of 512, 1024, 2048 or 4096 bytes.

use crate::xhci::Answer;

use crate::bot_sweep::rng::Rng;

#[derive(Clone, Debug)]
pub struct Reply {
    pub data: Vec<u8>,
    pub sent: usize,
    pub status: u8,
    pub residue: u32,
}

impl Reply {
    pub fn whole(data: Vec<u8>) -> Self {
        Reply { sent: data.len(), data, status: 0, residue: 0 }
    }

    /// The bulk INs the device answers with.
    pub fn answers(&self) -> Vec<Answer> {
        let data = match self.sent {
            _ if self.data.is_empty() => vec![],
            0 => vec![Answer::Zlp],
            n => vec![Answer::Data(self.data[..n].to_vec())],
        };
        let csw = Answer::Csw { residue: self.residue, status: self.status };
        data.into_iter().chain([csw]).collect()
    }
}

pub fn cap10(last_lba: u32, block_len: u32) -> Reply {
    Reply::whole([last_lba.to_be_bytes(), block_len.to_be_bytes()].concat())
}

pub fn cap16(last_lba: u64, block_len: u32) -> Reply {
    let mut d = [&last_lba.to_be_bytes()[..], &block_len.to_be_bytes()].concat();
    d.resize(32, 0);
    Reply::whole(d)
}

/// Fixed-format sense of `code` (70h current, 71h deferred) and `key`.
pub fn sense(code: u8, key: u8, asc: u8) -> Reply {
    let mut d = vec![0u8; 18];
    (d[0], d[2], d[7], d[12]) = (code, key, 10, asc);
    Reply::whole(d)
}

pub fn served(r: &mut Rng) -> u32 {
    [512, 1024, 2048, 4096][r.range(0, 3)]
}

pub fn lba32(r: &mut Rng) -> u32 {
    let some = r.range(0, u32::MAX as usize - 1) as u32;
    [0, 1, 0xFFFF_FFFE, some][r.range(0, 3)]
}
