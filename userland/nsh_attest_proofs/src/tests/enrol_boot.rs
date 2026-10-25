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

//! The boot applications are the machine's own, carried in its enrolment
//! and signed by the fleet's authority with the rest of it: another
//! machine's boot manager admits nothing here, and one added or changed
//! after signing breaks the certificate.

use nsh_attest::Refusal;

use super::{passes, refused};
use crate::build::tcg::EV_APP;
use crate::build::tcg_event::digest;
use crate::build::Spec;

#[test]
fn another_machine_s_boot_manager_is_refused_here() {
    let s = Spec { boot_apps: vec![digest(b"another boot manager")], ..Spec::default() };
    assert_eq!(refused(s), Refusal::BootApplication);
}

#[test]
fn every_application_the_enrolment_lists_may_run_and_no_other() {
    let apps = vec![digest(b"shim"), digest(b"boot manager"), digest(b"vendor tool")];
    let mut s = Spec { boot_apps: apps, ..Spec::default() };
    s.log.ext(4, EV_APP, digest(b"shim")).ext(4, EV_APP, digest(b"vendor tool"));
    passes(s);
    let mut s = Spec::default();
    s.log.ext(4, EV_APP, digest(b"shim"));
    assert_eq!(refused(s), Refusal::BootApplication);
}

#[test]
fn a_boot_application_changed_after_signing_breaks_the_certificate() {
    let s = Spec {
        cert_bytes: |b| {
            let want = digest(b"boot manager");
            let at = b.windows(32).position(|w| w == want).expect("listed");
            b[at] ^= 1;
        },
        ..Spec::default()
    };
    assert_eq!(refused(s), Refusal::EnrolSignature);
}
