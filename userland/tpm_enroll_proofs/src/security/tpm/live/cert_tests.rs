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

//! The RSA EK certificate swtpm_setup provisions reads back through the
//! kernel's NV reader byte for byte, in more than one chunk, and certifies the
//! very key `ek_public` derives. A TPM with no certificate says so.

use super::cert_setup::provisioned;
use super::swtpm::Swtpm;
use super::tools::{hex, tool};
use crate::security::tpm::enroll::ek_calls::derive_ek_public as ek_public;
use crate::security::tpm::enroll::nv_public::{build_nv_read_public, parse_nv_read_public};
use crate::security::tpm::enroll::{ek_certificate, EkKind, EnrollError};
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::KeyError;

#[test]
fn the_rsa_ek_certificate_reads_back_and_certifies_the_kernels_ek() {
    let test = "the_rsa_ek_certificate_reads_back_and_certifies_the_kernels_ek";
    let Some((state, work)) = provisioned(test) else { return };
    let Some(_t) = Swtpm::start_in(state, test) else { return };
    let index = EkKind::Rsa2048.cert_index();
    let slot =
        parse_nv_read_public(&run(&build_nv_read_public(index)).expect("read public"), index)
            .expect("certificate index");
    let cert = ek_certificate(EkKind::Rsa2048).expect("certificate");
    let file = work.file("ek-rsa2048.crt");
    assert_eq!(cert, std::fs::read(&file).expect("swtpm_setup's copy"), "byte-identical");
    assert!(cert.len() > 512, "read in more than one chunk");
    let ek = ek_public(EkKind::Rsa2048).expect("ek public");
    let modulus = &ek.area[ek.area.len() - 256..];
    let out =
        tool(None, "openssl", &["x509", "-inform", "DER", "-in", &file, "-noout", "-modulus"]);
    let certified = String::from_utf8(out).expect("openssl output");
    assert_eq!(certified.trim(), format!("Modulus={}", hex(modulus).to_uppercase()));
    eprintln!(
        "{test}: {} byte index read as {:#010x}, certificate {} bytes",
        slot.size,
        slot.auth,
        cert.len()
    );
}

#[test]
fn a_tpm_without_a_certificate_answers_tpm_rc_handle() {
    let Some(_t) = Swtpm::start("a_tpm_without_a_certificate_answers_tpm_rc_handle") else {
        return;
    };
    for kind in [EkKind::Rsa2048, EkKind::EccP256] {
        let r = ek_certificate(kind);
        assert_eq!(r, Err(EnrollError::Key(KeyError::Refused(0x18B))), "{kind:?}");
    }
}
