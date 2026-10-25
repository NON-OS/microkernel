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

//! A TPM session's caller nonce comes from the seeded RNG only. When it
//! cannot be drawn, the machine key and the device secret are refused with
//! `NoRandomness` before any command goes to the TPM: with no TPM here, a
//! command sent would fail as `Tpm(NotPresent)` instead.

use crate::crypto::rng::refuse_draws;
use crate::security::tpm::device_secret::{device_secret, Approval};
use crate::security::tpm::machine_key::derive::derive;
use crate::security::tpm::machine_key::KeyError;

#[test]
fn no_nonce_no_session() {
    let a = Approval { key_x: [1; 32], key_y: [2; 32], sig_r: [3; 32], sig_s: [4; 32] };
    refuse_draws(true);
    let (key, secret) = (derive(b"volume"), device_secret(&a));
    refuse_draws(false);
    assert_eq!(key, Err(KeyError::NoRandomness));
    assert_eq!(secret, Err(KeyError::NoRandomness));
    assert_eq!(KeyError::NoRandomness.as_str(), "no randomness for the session nonce");
}
