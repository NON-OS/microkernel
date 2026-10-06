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

//! The ECC index, which swtpm_setup 0.7 does not fill (it certifies a P-384
//! EK at `0x01C00016` instead): a certificate made here over the kernel's
//! L-2 EK is written to `0x01C0000A` with room to spare and only the owner
//! allowed to read it, and comes back trimmed to its DER length.

use super::swtpm::Swtpm;
use super::tools::tool;
use crate::security::tpm::enroll::ek_calls::derive_ek_public as ek_public;
use crate::security::tpm::enroll::nv_public::{build_nv_read_public, parse_nv_read_public};
use crate::security::tpm::enroll::{ek_certificate, EkKind};
use crate::security::tpm::machine_key::run::run;

#[test]
fn an_ecc_ek_certificate_reads_back_trimmed_through_the_owner() {
    let Some(t) = Swtpm::start("an_ecc_ek_certificate_reads_back_trimmed_through_the_owner") else {
        return;
    };
    let f = |n: &str| t.dir.file(n);
    tool(Some(&t), "tpm2_createek", &["-c", "-", "-G", "ecc", "-f", "pem", "-u", &f("ek.pem")]);
    let curve = ["-pkeyopt", "ec_paramgen_curve:P-256"];
    tool(
        None,
        "openssl",
        &[&["genpkey", "-algorithm", "EC", "-out", &f("ca.pem")][..], &curve].concat(),
    );
    let cert = ["x509", "-new", "-subj", "/CN=nonos-test-ek", "-key", &f("ca.pem"), "-days", "1"];
    let der = ["-force_pubkey", &f("ek.pem"), "-outform", "DER", "-out", &f("ek.der")];
    tool(None, "openssl", &[&cert[..], &der].concat());
    let der = std::fs::read(f("ek.der")).expect("certificate");
    let size = (der.len() + 64).to_string();
    let attrs = "ppwrite|writedefine|ppread|ownerread|no_da|platformcreate";
    tool(Some(&t), "tpm2_nvdefine", &["0x01C0000A", "-C", "p", "-s", &size, "-a", attrs]);
    tool(Some(&t), "tpm2_nvwrite", &["0x01C0000A", "-C", "p", "-i", &f("ek.der")]);
    let index = EkKind::EccP256.cert_index();
    let slot = parse_nv_read_public(&run(&build_nv_read_public(index)).expect("read"), index)
        .expect("certificate index");
    assert_eq!((slot.size, slot.auth), (der.len() + 64, 0x4000_0001), "owner reads it");
    assert_eq!(ek_certificate(EkKind::EccP256).expect("certificate"), der, "trimmed to its DER");
    let spki = tool(None, "openssl", &["pkey", "-pubin", "-in", &f("ek.pem"), "-outform", "DER"]);
    let ek = ek_public(EkKind::EccP256).expect("ek public");
    let n = ek.area.len();
    let xy = [&ek.area[n - 66..n - 34], &ek.area[n - 32..]].concat();
    assert_eq!(spki[spki.len() - 64..], xy[..], "the certified key is the kernel's EK");
}
