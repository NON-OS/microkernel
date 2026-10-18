// NONOS Operating System (AGPL-3.0-or-later)
//! A disk behind the scripted device (`xhci`): its blocks in memory, and the
//! READ and WRITE CBWs, ten and sixteen bytes, answered from them. Anything
//! else the CBW asks is answered good with no data.

use std::collections::VecDeque;

use crate::xhci::{Answer, BULK_MAX};

pub struct Store {
    pub len: usize,
    pub bytes: Vec<u8>,
    /// READ and WRITE commands taken.
    pub reads: usize,
    pub writes: usize,
    /// Where the write data now arriving goes, and how much is still to come.
    writing: Option<(usize, usize)>,
}

impl Store {
    pub fn new(len: usize, blocks: usize) -> Self {
        let bytes = (0..len * blocks).map(|i| (i * 7 + i / 512) as u8).collect();
        Self { len, bytes, reads: 0, writes: 0, writing: None }
    }

    /// Take a bulk OUT, a CBW or write data, and queue what the device sends.
    pub fn out(&mut self, data: &[u8], answers: &mut VecDeque<Answer>) {
        if let Some((at, left)) = self.writing {
            let n = data.len().min(left);
            self.bytes[at..at + n].copy_from_slice(&data[..n]);
            self.writing = (left > n).then_some((at + n, left - n));
            if self.writing.is_none() {
                answers.push_back(Answer::Csw { residue: 0, status: 0 });
            }
            return;
        }
        let cdb = &data[15..31];
        let (lba, count) = match cdb[0] {
            0x28 | 0x2A => (be(&cdb[2..6]), be(&cdb[7..9])),
            0x88 | 0x8A => (be(&cdb[2..10]), be(&cdb[10..14])),
            _ => return answers.push_back(Answer::Csw { residue: 0, status: 0 }),
        };
        let (at, n) = (lba as usize * self.len, count as usize * self.len);
        if cdb[0] & 0x02 != 0 {
            self.writes += 1;
            self.writing = Some((at, n));
            return;
        }
        self.reads += 1;
        for chunk in self.bytes[at..at + n].chunks(BULK_MAX) {
            answers.push_back(Answer::Data(chunk.to_vec()));
        }
        answers.push_back(Answer::Csw { residue: 0, status: 0 });
    }
}

fn be(b: &[u8]) -> u64 {
    b.iter().fold(0, |v, &x| (v << 8) | x as u64)
}
