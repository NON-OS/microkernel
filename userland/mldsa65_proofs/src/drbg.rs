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

//! AES-256 CTR_DRBG without a derivation function, as the NIST PQC test
//! generator seeds and draws it: the state is a key and a counter block,
//! each draw encrypts successive counters, and each draw ends with an
//! update that replaces both.

use nonos_aes::Aes256;

pub struct Drbg {
    key: [u8; 32],
    v: [u8; 16],
}

impl Drbg {
    pub fn new(entropy: &[u8; 48]) -> Self {
        let mut d = Drbg { key: [0; 32], v: [0; 16] };
        d.update(Some(entropy));
        d
    }

    fn next_block(&mut self) -> [u8; 16] {
        for b in self.v.iter_mut().rev() {
            *b = b.wrapping_add(1);
            if *b != 0 {
                break;
            }
        }
        let mut block = self.v;
        Aes256::new(&self.key).encrypt_block(&mut block);
        block
    }

    fn update(&mut self, provided: Option<&[u8; 48]>) {
        let mut temp = [0u8; 48];
        for i in 0..3 {
            temp[16 * i..16 * i + 16].copy_from_slice(&self.next_block());
        }
        if let Some(p) = provided {
            temp.iter_mut().zip(p).for_each(|(t, p)| *t ^= p);
        }
        self.key.copy_from_slice(&temp[..32]);
        self.v.copy_from_slice(&temp[32..]);
    }

    pub fn fill(&mut self, out: &mut [u8]) {
        for chunk in out.chunks_mut(16) {
            let block = self.next_block();
            chunk.copy_from_slice(&block[..chunk.len()]);
        }
        self.update(None);
    }
}
