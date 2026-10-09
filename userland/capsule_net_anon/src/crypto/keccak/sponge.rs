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


//! The sponge, and the two instances onion services need.

use super::permute::keccak_f;

/// Both instances have capacity 512, so rate 136 bytes.
const RATE: usize = 136;
/// FIPS 202 domain separation, with the first padding bit folded in.
const SHA3_PAD: u8 = 0x06;
const SHAKE_PAD: u8 = 0x1f;

#[derive(Clone)]
struct Sponge {
    state: [u64; 25],
    /// Bytes absorbed into the current block, or squeezed out of it.
    at: usize,
}

impl Sponge {
    fn new() -> Self {
        Self { state: [0u64; 25], at: 0 }
    }

    fn xor_byte(&mut self, index: usize, byte: u8) {
        self.state[index / 8] ^= (byte as u64) << (8 * (index % 8));
    }

    fn absorb(&mut self, data: &[u8]) {
        for byte in data {
            self.xor_byte(self.at, *byte);
            self.at += 1;
            if self.at == RATE {
                keccak_f(&mut self.state);
                self.at = 0;
            }
        }
    }

    /// Pad and permute; afterwards `at` counts bytes squeezed from the block.
    fn pad(&mut self, domain: u8) {
        self.xor_byte(self.at, domain);
        self.xor_byte(RATE - 1, 0x80);
        keccak_f(&mut self.state);
        self.at = 0;
    }

    fn squeeze(&mut self, out: &mut [u8]) {
        for slot in out.iter_mut() {
            if self.at == RATE {
                keccak_f(&mut self.state);
                self.at = 0;
            }
            *slot = (self.state[self.at / 8] >> (8 * (self.at % 8))) as u8;
            self.at += 1;
        }
    }
}

impl Drop for Sponge {
    fn drop(&mut self) {
        for lane in self.state.iter_mut() {
            /*
             * SAFETY: eK@nonos.systems. The state has absorbed key material
             * (descriptor secrets, hs-ntor inputs), so it is wiped with a
             * store the optimiser may not remove.
             */
            unsafe { core::ptr::write_volatile(lane, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}

/// SHA3-256, cloneable and peekable like the SHA-1 a relay hop runs.
#[derive(Clone)]
pub struct Sha3_256 {
    sponge: Sponge,
}

impl Default for Sha3_256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha3_256 {
    pub fn new() -> Self {
        Self { sponge: Sponge::new() }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.sponge.absorb(data);
    }

    pub fn finish(mut self) -> [u8; 32] {
        let mut out = [0u8; 32];
        self.sponge.pad(SHA3_PAD);
        self.sponge.squeeze(&mut out);
        out
    }

    /// The digest of everything so far, leaving the running state as it is.
    pub fn peek(&self) -> [u8; 32] {
        self.clone().finish()
    }
}

/// SHAKE-256: absorb, then squeeze as many bytes as wanted.
pub struct Shake256 {
    sponge: Sponge,
    squeezing: bool,
}

impl Default for Shake256 {
    fn default() -> Self {
        Self::new()
    }
}

impl Shake256 {
    pub fn new() -> Self {
        Self { sponge: Sponge::new(), squeezing: false }
    }

    /// Absorb more input. Nothing may be absorbed once squeezing has begun,
    /// and a call after that is ignored rather than corrupting the output.
    pub fn update(&mut self, data: &[u8]) {
        if !self.squeezing {
            self.sponge.absorb(data);
        }
    }

    /// The next `out.len()` bytes of output.
    pub fn squeeze(&mut self, out: &mut [u8]) {
        if !self.squeezing {
            self.sponge.pad(SHAKE_PAD);
            self.squeezing = true;
        }
        self.sponge.squeeze(out);
    }
}
