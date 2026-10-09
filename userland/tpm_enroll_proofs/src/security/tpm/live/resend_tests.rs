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

//! Why every command resends. The AK has no `noDA`, so the first command of a
//! TPM startup that authorizes it with its password is answered with
//! `TPM_RC_RETRY` and not run; sent again, it runs. `machine_key::run` and the
//! quote go through `transact_resending`, so neither meets it.

use super::steps::load_test_ak;
use super::swtpm::Swtpm;
use crate::security::tpm::enroll::hash::{build_hash, parse_hash};
use crate::security::tpm::enroll::sign::{build_sign, parse_sign};
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::transact;

/// The response code of `cmd` sent once, with no resend.
pub fn sent_once(cmd: &[u8]) -> u32 {
    let mut out = [0u8; 4096];
    // SAFETY: the test's own swtpm; the signature is the kernel's.
    let n = unsafe { transact(cmd, &mut out) }.expect("sent");
    assert!(n >= 10, "a whole header");
    u32::from_be_bytes([out[6], out[7], out[8], out[9]])
}

#[test]
fn the_first_ak_authorization_of_a_startup_is_retry() {
    let Some(_t) = Swtpm::start("the_first_ak_authorization_of_a_startup_is_retry") else {
        return;
    };
    let (ak, _) = load_test_ak();
    let (digest, ticket) = parse_hash(&run(&build_hash(&[0x44; 32])).expect("hash")).expect("t");
    let cmd = build_sign(ak, &digest, &ticket);
    assert_eq!(sent_once(&cmd), 0x922, "TPM_RC_RETRY, and the command did not run");
    assert_eq!(sent_once(&cmd), 0, "the same bytes run the second time");
}

#[test]
fn run_resends_so_the_first_sign_of_a_startup_runs() {
    let Some(_t) = Swtpm::start("run_resends_so_the_first_sign_of_a_startup_runs") else {
        return;
    };
    let (ak, _) = load_test_ak();
    let (digest, ticket) = parse_hash(&run(&build_hash(&[0x45; 32])).expect("hash")).expect("t");
    let first = parse_sign(&run(&build_sign(ak, &digest, &ticket)).expect("sent"));
    assert!(first.is_ok(), "the first sign of this startup: {first:?}");
}
