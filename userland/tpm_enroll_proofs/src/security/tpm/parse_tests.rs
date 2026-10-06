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

//! Against bytes written from the specification, with no TPM: the profile's
//! templates, and every bound and refusal of the activation and its answer.

use super::parse_util::{resp, unhex};
use crate::security::tpm::enroll::activate::{build_activate, check_challenge, parse_activate};
use crate::security::tpm::enroll::ek::build_create_ek;
use crate::security::tpm::enroll::{EkKind, EnrollError};
use crate::security::tpm::machine_key::KeyError;

const POLICY_A: &str = "837197674484b3f81a90cc8d46a5d724fd52d76e06520b64f2a1da1b331469aa";

#[test]
fn the_ek_commands_carry_the_profiles_templates_l1_and_l2() {
    let (k, z) = ("0006 0080 0043 0010", "00".repeat(32));
    let l1 =
        format!("0001 000b 000300b2 0020 {POLICY_A} {k} 0800 00000000 0100 {}", "00".repeat(256));
    let l2 = format!("0023 000b 000300b2 0020 {POLICY_A} {k} 0003 0010 0020 {z} 0020 {z}");
    for (kind, template) in [(EkKind::Rsa2048, unhex(&l1)), (EkKind::EccP256, unhex(&l2))] {
        let mut want =
            unhex("80020000 0000 00000131 4000000b 00000009 400000090000000000 0004 00000000");
        want.extend_from_slice(&(template.len() as u16).to_be_bytes());
        want.extend_from_slice(&template);
        want.extend_from_slice(&[0; 6]);
        let size = want.len() as u32;
        want[2..6].copy_from_slice(&size.to_be_bytes());
        assert_eq!(build_create_ek(kind), want, "{kind:?}");
    }
}

#[test]
fn challenge_inputs_are_bounded_before_any_command() {
    assert!(check_challenge(&[1; 132], &[1; 512]).is_ok());
    for (b, s) in [(0, 256), (133, 256), (68, 0), (68, 513)] {
        let (blob, secret) = (vec![1; b], vec![1; s]);
        assert_eq!(check_challenge(&blob, &secret), Err(EnrollError::OutOfBounds), "{b} {s}");
        assert_eq!(build_activate(1, 2, 3, &blob, &secret).err(), Some(EnrollError::OutOfBounds));
    }
    let cmd = build_activate(0x8000_0000, 0x8000_0001, 0x0300_0000, &[1; 68], &[2; 256]);
    let cmd = cmd.expect("in bounds");
    let auth = "00000012 400000090000000000 030000000000000000";
    assert_eq!(cmd[10..40], unhex(&format!("80000000 80000001 {auth}"))[..]);
    assert_eq!(cmd[40..42], [0, 68]);
    assert_eq!(cmd.len(), 40 + 2 + 68 + 2 + 256);
}

#[test]
fn activation_answers_are_bounded() {
    let ok = resp(0, &[&[0, 0, 0, 34][..], &[0, 32], &[9; 32]].concat());
    assert_eq!(parse_activate(&ok).expect("a digest").as_bytes(), &[9; 32]);
    for cut in 0..ok.len() {
        assert!(parse_activate(&ok[..cut]).is_err(), "cut at {cut}");
    }
    for n in [0usize, 65] {
        let r = resp(0, &[&[0, 0, 0, 0][..], &(n as u16).to_be_bytes(), &vec![9; n]].concat());
        assert!(parse_activate(&r).is_err(), "{n} bytes");
    }
    let refused = parse_activate(&resp(0x1DF, &[])).err();
    assert_eq!(refused, Some(EnrollError::Key(KeyError::Refused(0x1DF))));
}
