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

//! The devkit enrols a live machine from its request: read from the proof
//! spec's own document, the machine is the fixture's and its enrolment is
//! the fixture machine's byte for byte; a machine's own key, PCRs, boot
//! applications and the host keys it made are what its enrolment names; a
//! loader PCR 4 does not record, or a document that does not parse, is
//! refused.

use super::devkit_host::public;
use crate::kit::attest_build::parts::release;
use crate::kit::attest_build::spec::Spec;
use crate::kit::attest_build::tcg_log::loader;
use crate::kit::devkit::make::enrol_for;
use crate::kit::devkit::{enrolment, Machine, Window};
use crate::kit::host::spec;

const AT: u64 = 1_793_000_000_000;

#[test]
fn the_spec_document_is_the_fixture_machine() {
    let doc = spec().build().doc;
    let m = Machine::from_doc(&doc, &loader()).expect("the machine");
    assert_eq!(m, Machine::fixture());
    let w = Window::at(AT).expect("a window");
    let live = enrolment(&w, &m, &public());
    assert_eq!(live.name, "enrol");
    assert_eq!(live.bytes, enrolment(&w, &Machine::fixture(), &public()).bytes);
}

#[test]
fn a_machine_is_what_its_enrolment_names() {
    let m = Machine {
        ak: [0xA5; 64],
        pcrs: [[1; 32], [2; 32], [3; 32], [7; 32]],
        boot_apps: vec![[0xB0; 32]],
    };
    let w = Window::at(AT).expect("a window");
    let mut keys = public();
    keys[0] ^= 1;
    keys[2000 - 17] ^= 0x40;
    let bytes = enrol_for(&w, &m, &keys);
    let e = nsh_wire::Enrol::parse(&bytes).expect("an enrolment");
    assert_eq!(e.ak_public, &[0xA5; 64]);
    assert_eq!(e.not_after_ms, w.enrol_ends_ms);
    let s = Spec::default();
    let mut want = s.cert();
    (want.ak, want.pcrs, want.boot_apps, want.not_after_ms) =
        (m.ak, m.pcrs, m.boot_apps.clone(), w.enrol_ends_ms);
    want.host_keys = nsh_crypto::key_digest(&keys[..32], &keys[32..]);
    assert_eq!(e.host_keys, &want.host_keys, "the keys the machine made");
    assert_eq!(
        bytes,
        want.sign(crate::kit::attest_build::keys::hybrid(
            crate::kit::attest_build::keys::AUTHORITY
        ))
    );
}

#[test]
fn a_loader_pcr4_does_not_record_or_a_broken_document_is_refused() {
    let doc = spec().build().doc;
    assert!(Machine::from_doc(&doc, &release().pcr8).is_err());
    assert!(Machine::from_doc(b"NONOSATT", &loader()).is_err());
}
