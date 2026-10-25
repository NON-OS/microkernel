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
//! The shape of an RSN element body as the sweeps build it: each field's
//! kind and span, and the reading the building implies.

use super::rng::Rng;
use nonos_wifi_core::rsn::suite::CIPHER_CCMP;
use nonos_wifi_core::rsn::Rsne;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Version,
    Fixed,
    /// A list's count, then its `n` entries of `size` octets.
    Count {
        size: usize,
    },
    Entry,
    GroupMgmt,
}

pub struct Sound {
    pub body: Vec<u8>,
    pub want: Rsne,
    /// Each field's kind and span.
    pub fields: Vec<(Field, usize, usize)>,
}

impl Sound {
    pub fn put(&mut self, f: Field, b: &[u8]) {
        let at = self.body.len();
        self.body.extend_from_slice(b);
        self.fields.push((f, at, self.body.len()));
    }
}

pub fn suite(rng: &mut Rng, types: &[u8]) -> [u8; 4] {
    let oui = if rng.below(6) == 0 { [rng.byte(), rng.byte(), rng.byte()] } else { [0, 15, 172] };
    let t = if rng.below(6) == 0 { rng.byte() } else { rng.pick(types) };
    [oui[0], oui[1], oui[2], t]
}

/// What an element that ends after its version reads as (9.4.2.24.1).
pub fn defaults() -> Rsne {
    Rsne {
        group: CIPHER_CCMP,
        pairwise_ccmp: true,
        pairwise_gcmp256: false,
        akm_psk: false,
        akm_psk_sha256: false,
        akm_sae: false,
        caps: 0,
        group_mgmt: None,
    }
}
