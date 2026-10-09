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

//! The EK the kernel derives is the EK tpm2-tools derives, byte for byte, and
//! it is gone from the TPM when the call returns.

use sha2::{Digest, Sha256};

use super::swtpm::Swtpm;
use super::tools::{held, tool};
use crate::security::tpm::enroll::ek::{build_create_ek, parse_ek};
use crate::security::tpm::enroll::ek_calls::derive_ek_public as ek_public;
use crate::security::tpm::enroll::EkKind;
use crate::security::tpm::machine_key::create::parse_create;
use crate::security::tpm::machine_key::flush::build_flush;
use crate::security::tpm::machine_key::run::run;

fn same_as_tpm2_createek(kind: EkKind, alg: &str, test: &str) {
    let Some(t) = Swtpm::start(test) else { return };
    let ek = ek_public(kind).expect("kernel ek");
    assert_eq!(held(&t), (0, 0), "the EK was flushed");
    tool(Some(&t), "tpm2_createek", &["-c", "-", "-G", alg, "-u", &t.dir.file("ek.pub")]);
    let reference = std::fs::read(t.dir.file("ek.pub")).expect("tpm2_createek output");
    assert_eq!(ek.area, reference, "TPM2B_PUBLIC byte-identical to tpm2_createek's");
    assert_eq!(ek.name[..2], [0x00, 0x0B]);
    assert_eq!(ek.name[2..], Sha256::digest(&ek.area[2..])[..], "name hashes TPMT_PUBLIC");
    assert_eq!(ek_public(kind).expect("again"), ek, "reproduced on every call");
    eprintln!("{test}: {} bytes, identical to tpm2_createek -G {alg}", ek.area.len());
}

#[test]
fn rsa_ek_is_the_one_tpm2_createek_derives() {
    same_as_tpm2_createek(EkKind::Rsa2048, "rsa", "rsa_ek_is_the_one_tpm2_createek_derives");
}

#[test]
fn ecc_ek_is_the_one_tpm2_createek_derives() {
    same_as_tpm2_createek(EkKind::EccP256, "ecc", "ecc_ek_is_the_one_tpm2_createek_derives");
}

/// A real CreatePrimary answer cut anywhere inside its parameters is refused,
/// none is read past its end, and the other kind's shape does not pass.
#[test]
fn a_cut_or_mismatched_ek_answer_is_refused() {
    let Some(_t) = Swtpm::start("a_cut_or_mismatched_ek_answer_is_refused") else { return };
    for kind in [EkKind::Rsa2048, EkKind::EccP256] {
        let resp = run(&build_create_ek(kind)).expect("create");
        run(&build_flush(parse_create(&resp).expect("handle"))).expect("flush");
        let other = if kind == EkKind::Rsa2048 { EkKind::EccP256 } else { EkKind::Rsa2048 };
        assert!(parse_ek(&resp, kind).is_ok());
        assert!(parse_ek(&resp, other).is_err());
        let end = 18 + u32::from_be_bytes([resp[14], resp[15], resp[16], resp[17]]) as usize;
        for n in 0..end {
            assert!(parse_ek(&resp[..n], kind).is_err(), "{kind:?} cut at {n}");
        }
        let mut flipped = resp.to_vec();
        flipped[end - 1] ^= 1;
        assert!(parse_ek(&flipped, kind).is_err(), "a name that is not the hash");
    }
}
