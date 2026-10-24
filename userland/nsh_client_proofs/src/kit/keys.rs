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

//! The fleet's hybrid keys: the host, the enrolment authority, the role
//! authority, the user and a stranger. Each is made once from a fixed seed.

pub use super::keys_use::user_key;
use super::rng::Det;
use nonos_ed25519::KeyPair;
use nonos_mldsa65::SecretKey;
use std::sync::OnceLock;

pub struct Hybrid {
    pub ed: KeyPair,
    pub ml: SecretKey,
    pub ml_public: [u8; 1952],
}

fn make(seed: u8) -> Hybrid {
    let (ml_public, ml) = nonos_mldsa65::keypair(&mut |b| Det::new(seed).fill(b)).unwrap();
    Hybrid { ed: KeyPair::from_seed([seed; 32]), ml, ml_public }
}

macro_rules! key {
    ($name:ident, $seed:expr) => {
        pub fn $name() -> &'static Hybrid {
            static K: OnceLock<Hybrid> = OnceLock::new();
            K.get_or_init(|| make($seed))
        }
    };
}

key!(host, 0x40);
key!(authority, 0x41);
key!(role, 0x42);
key!(user, 0x43);
key!(stranger, 0x44);
