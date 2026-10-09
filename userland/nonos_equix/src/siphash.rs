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


//! The SipHash pieces HashX uses: the round itself, a SipHash-1-3 over a
//! counter that drives program generation, and a SipHash-2-4 over the input
//! that fills the eight registers before a program runs. Neither is the
//! SipHash of a byte string; both take one 64-bit word and a full 256-bit
//! state as the key, exactly as the reference does.

/// Four 64-bit words: a key when it seeds a hash, a state while one runs.
#[derive(Clone, Copy, Default)]
pub struct SipState {
    pub v0: u64,
    pub v1: u64,
    pub v2: u64,
    pub v3: u64,
}

pub fn round(v0: &mut u64, v1: &mut u64, v2: &mut u64, v3: &mut u64) {
    *v0 = v0.wrapping_add(*v1);
    *v2 = v2.wrapping_add(*v3);
    *v1 = v1.rotate_left(13);
    *v3 = v3.rotate_left(16);
    *v1 ^= *v0;
    *v3 ^= *v2;
    *v0 = v0.rotate_left(32);
    *v2 = v2.wrapping_add(*v1);
    *v0 = v0.wrapping_add(*v3);
    *v1 = v1.rotate_left(17);
    *v3 = v3.rotate_left(21);
    *v1 ^= *v2;
    *v3 ^= *v0;
    *v2 = v2.rotate_left(32);
}

/// One word of the generator's stream: SipHash-1-3 of `input` under `key`.
pub fn siphash13_ctr(input: u64, key: &SipState) -> u64 {
    let SipState { mut v0, mut v1, mut v2, mut v3 } = *key;
    v3 ^= input;
    round(&mut v0, &mut v1, &mut v2, &mut v3);
    v0 ^= input;
    v2 ^= 0xff;
    for _ in 0..3 {
        round(&mut v0, &mut v1, &mut v2, &mut v3);
    }
    (v0 ^ v1) ^ (v2 ^ v3)
}

/// The eight starting registers for one input: SipHash-2-4 of `input`, with
/// its state read out after finalization and again after four more rounds.
pub fn siphash24_ctr_state512(key: &SipState, input: u64) -> [u64; 8] {
    let SipState { mut v0, mut v1, mut v2, mut v3 } = *key;
    v1 ^= 0xee;
    v3 ^= input;
    round(&mut v0, &mut v1, &mut v2, &mut v3);
    round(&mut v0, &mut v1, &mut v2, &mut v3);
    v0 ^= input;
    v2 ^= 0xee;
    for _ in 0..4 {
        round(&mut v0, &mut v1, &mut v2, &mut v3);
    }
    let first = [v0, v1, v2, v3];
    v1 ^= 0xdd;
    for _ in 0..4 {
        round(&mut v0, &mut v1, &mut v2, &mut v3);
    }
    [first[0], first[1], first[2], first[3], v0, v1, v2, v3]
}

/// The byte and word stream program generation draws from. Bytes and words
/// are served from two separate buffers, each refilled from the one shared
/// counter, most significant part first; the order matters because it is
/// what decides every instruction.
pub struct SipRng {
    key: SipState,
    counter: u64,
    buffer8: u64,
    buffer32: u64,
    count8: u32,
    count32: u32,
}

impl SipRng {
    pub fn new(key: SipState) -> Self {
        Self { key, counter: 0, buffer8: 0, buffer32: 0, count8: 0, count32: 0 }
    }

    pub fn u8(&mut self) -> u8 {
        if self.count8 == 0 {
            self.buffer8 = siphash13_ctr(self.counter, &self.key);
            self.counter = self.counter.wrapping_add(1);
            self.count8 = 8;
        }
        self.count8 -= 1;
        (self.buffer8 >> (self.count8 * 8)) as u8
    }

    pub fn u32(&mut self) -> u32 {
        if self.count32 == 0 {
            self.buffer32 = siphash13_ctr(self.counter, &self.key);
            self.counter = self.counter.wrapping_add(1);
            self.count32 = 2;
        }
        self.count32 -= 1;
        (self.buffer32 >> (self.count32 * 32)) as u32
    }
}
