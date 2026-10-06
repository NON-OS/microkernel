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

//! The device secret on a real TPM 2.0, under approvals the release tool's own
//! functions computed: what the chain decides, and what it does not.

use super::release::{approve, release, Release};
use super::swtpm::Swtpm;
use super::tools::{hex, tool};
use crate::security::tpm::device_secret::{device_secret, Approval};

const P: u64 = 0xFFFF_FFFF_0000_0001;

/// The loader admitting `r`'s kernel: one extend of PCR 9, which then reads
/// what the release tool says it will.
fn boot(t: &Swtpm, r: &Release) {
    tool(Some(t), "tpm2_pcrextend", &[&format!("9:sha256={}", hex(&r.extend))]);
    let file = t.dir.file("pcr9");
    tool(Some(t), "tpm2_pcrread", &["sha256:9", "-o", &file]);
    assert_eq!(std::fs::read(&file).expect("pcr9"), r.pcr9, "PCR 9 as the release tool computes it");
}

fn secret(a: &Approval) -> Option<[u64; 4]> {
    device_secret(a).ok()
}

#[test]
fn an_approved_kernel_gets_one_secret_and_a_signed_update_keeps_it() {
    let test = "an_approved_kernel_gets_one_secret_and_a_signed_update_keeps_it";
    let Some(t) = Swtpm::start(test) else { return };
    let (a, b) = (release(&[0xA1; 32], &[0x0F; 32]), release(&[0xB2; 32], &[0x0F; 32]));
    boot(&t, &a);
    let s = secret(&approve(7, &a)).expect("the approved kernel");
    assert_eq!(secret(&approve(7, &a)), Some(s), "the same secret each time");
    assert!(s.iter().all(|&w| w < P), "every word below p");
    let t = t.reboot(test);
    boot(&t, &b);
    assert_eq!(secret(&approve(7, &a)), None, "the old approval does not cover the new kernel");
    assert_eq!(secret(&approve(7, &b)), Some(s), "the new kernel's approval keeps the secret");
}

#[test]
fn a_bad_signature_is_refused_and_another_key_or_machine_is_another_secret() {
    let Some(t) = Swtpm::start("a_bad_signature_is_refused_and_another_key_or_machine") else {
        return;
    };
    let a = release(&[0xA1; 32], &[0x0F; 32]);
    boot(&t, &a);
    let good = approve(7, &a);
    let s = secret(&good).expect("approved");
    let mut forged = good;
    forged.sig_s[31] ^= 1;
    assert_eq!(secret(&forged), None, "the TPM checks the signature");
    let other = secret(&approve(8, &a)).expect("another key's approval");
    assert_ne!(other, s, "the policy binds the key's name");
    tool(Some(&t), "tpm2_pcrextend", &[&format!("7:sha256={}", hex(&[0x5B; 32]))]);
    let moved = secret(&good).expect("PCR 9 still approved");
    assert_ne!(moved, s, "a Secure Boot change is another secret");
}
