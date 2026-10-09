// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The rollback counter and its base kept as TPM 2.0 Part 3 keeps them. The
//! counter is uninitialized until its first increment, which starts it one
//! above the highest value any counter on the TPM held, and TPM2_Clear does
//! not lower that. The base is an ordinary index: uninitialized until written,
//! and refused every write once locked.

pub const CC_DEFINE: u32 = 0x12A;
pub const CC_INCREMENT: u32 = 0x134;
pub const CC_WRITE: u32 = 0x137;
pub const CC_WRITELOCK: u32 = 0x138;
pub const CC_READ: u32 = 0x14E;

const COUNTER: u8 = 0x20;
const BASE: u8 = 0x21;

#[derive(Default)]
pub struct Tpm {
    pub defined: bool,
    pub value: Option<u64>,
    pub base_defined: bool,
    pub base: Option<u64>,
    pub base_locked: bool,
    pub max: u64, // the highest value any counter on this TPM held
    pub fail_increment: bool,
    pub fail_base_write: bool,
    pub read_rc: Option<u32>, // answer every counter read with this instead
    pub log: Vec<u32>,        // the command codes received, in order
}

fn header(rc: u32, len: usize, out: &mut [u8]) -> usize {
    let tag: u16 = if rc == 0 { 0x8002 } else { 0x8001 };
    out[..10].copy_from_slice(&[&tag.to_be_bytes()[..], &(len as u32).to_be_bytes(), &rc.to_be_bytes()].concat());
    len
}

fn value(v: u64, out: &mut [u8]) -> usize {
    out[10..29].copy_from_slice(&[&[0, 0, 0, 10, 0, 8][..], &v.to_be_bytes(), &[0; 5]].concat());
    header(0, 29, out)
}

impl Tpm {
    /// A counter holding `counter` over a base holding `base`, locked.
    pub fn holding(counter: u64, base: u64) -> Tpm {
        Tpm {
            defined: true,
            value: Some(counter),
            base_defined: true,
            base: Some(base),
            base_locked: true,
            max: counter,
            ..Default::default()
        }
    }

    /// The owner runs TPM2_NV_UndefineSpace on the counter.
    pub fn undefine(&mut self) {
        self.max = self.max.max(self.value.unwrap_or(0));
        (self.defined, self.value) = (false, None);
    }

    /// The owner runs TPM2_NV_UndefineSpace on the base.
    pub fn undefine_base(&mut self) {
        (self.base_defined, self.base, self.base_locked) = (false, None, false);
    }

    /// TPM2_Clear: every owner index goes, the counters' high mark stays.
    pub fn clear(&mut self) {
        self.undefine();
        self.undefine_base();
    }

    pub fn answer(&mut self, cmd: &[u8], out: &mut [u8]) -> Option<usize> {
        let cc = u32::from_be_bytes(cmd[6..10].try_into().ok()?);
        self.log.push(cc);
        let index = if cc == CC_DEFINE { *cmd.get(34)? } else { *cmd.get(17)? };
        let rc = match (cc, index) {
            (CC_DEFINE, COUNTER) if self.defined => 0x14C,
            (CC_DEFINE, COUNTER) => {
                self.defined = true;
                0
            }
            (CC_DEFINE, BASE) if self.base_defined => 0x14C,
            (CC_DEFINE, BASE) => {
                self.base_defined = true;
                0
            }
            (_, COUNTER) if !self.defined => 0x18B,
            (_, BASE) if !self.base_defined => 0x18B,
            (CC_INCREMENT, COUNTER) if self.fail_increment => 0x101,
            (CC_INCREMENT, COUNTER) => {
                let v = self.value.unwrap_or(self.max) + 1;
                (self.value, self.max) = (Some(v), self.max.max(v));
                0
            }
            (CC_READ, COUNTER) => match (self.read_rc, self.value) {
                (Some(rc), _) => rc,
                (None, None) => 0x14A,
                (None, Some(v)) => return Some(value(v, out)),
            },
            (CC_WRITE, BASE) if self.base_locked => 0x148,
            (CC_WRITE, BASE) if self.fail_base_write => 0x101,
            (CC_WRITE, BASE) => {
                self.base = Some(u64::from_be_bytes(cmd.get(33..41)?.try_into().ok()?));
                0
            }
            (CC_WRITELOCK, BASE) => {
                self.base_locked = true;
                0
            }
            (CC_READ, BASE) => match self.base {
                None => 0x14A,
                Some(b) => return Some(value(b, out)),
            },
            _ => 0x143,
        };
        Some(header(rc, 10, out))
    }
}
