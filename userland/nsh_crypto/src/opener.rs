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

//! Opening sealed frames. Once a frame fails to open, every later frame
//! fails too: the session is over, and nothing after a forged, dropped or
//! reordered frame is trusted.

use crate::count::Count;
use crate::{CryptoError, Secret};
use nonos_seal::{open, TAG_LEN};

pub struct Opener {
    count: Count,
    dead: bool,
}

impl Opener {
    pub fn new(key: Secret<32>) -> Self {
        Opener::from_count(Count::new(key))
    }

    pub(crate) fn from_count(count: Count) -> Self {
        Opener { count, dead: false }
    }

    /// Open the `sealed` bytes that followed `len` into `out`, giving the
    /// type byte and body.
    pub fn open(
        &mut self,
        len: [u8; 4],
        sealed: &[u8],
        out: &mut [u8],
    ) -> Result<usize, CryptoError> {
        let fits = u32::from_le_bytes(len) as usize == sealed.len() && sealed.len() > TAG_LEN;
        if self.dead || !fits {
            self.dead = true;
            return Err(CryptoError::Open);
        }
        let inner = sealed.len() - TAG_LEN;
        let nonce = self.count.nonce(inner)?;
        match open(self.count.key.expose(), &nonce, &len, sealed, out) {
            Ok(n) => {
                self.count.advance(inner);
                Ok(n)
            }
            Err(_) => {
                self.dead = true;
                Err(CryptoError::Open)
            }
        }
    }
}
