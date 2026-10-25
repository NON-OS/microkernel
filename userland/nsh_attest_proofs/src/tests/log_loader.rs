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

//! Step 5, the applications in PCR 4: each is the release's loader or a
//! boot application this machine's enrolment lists, and the loader is
//! among them.

use nsh_attest::Refusal;

use super::{passes, refused};
use crate::build::tcg::EV_APP;
use crate::build::tcg_event::digest;
use crate::build::tcg_log::{loader, Log};
use crate::build::Spec;

fn log(f: fn(&mut Log)) -> Refusal {
    let mut s = Spec::default();
    f(&mut s.log);
    refused(s)
}

#[test]
fn an_unlisted_application_after_the_loader_is_refused() {
    let r = log(|l| {
        l.ext(4, EV_APP, digest(b"a shell"));
    });
    assert_eq!(r, Refusal::BootApplication);
}

#[test]
fn an_unlisted_application_before_the_loader_is_refused() {
    let mut l = Log::empty();
    l.ext(4, EV_APP, digest(b"a shell")).ext(4, EV_APP, loader());
    assert_eq!(refused(Spec { log: l, ..Spec::default() }), Refusal::BootApplication);
}

#[test]
fn a_boot_manager_the_enrolment_does_not_list_is_refused() {
    let s = Spec { boot_apps: vec![], ..Spec::default() };
    assert_eq!(refused(s), Refusal::BootApplication);
}

#[test]
fn a_log_without_the_loader_is_refused() {
    let mut l = Log::empty();
    l.ext(0, 1, digest(b"crtm")).ext(4, EV_APP, digest(b"boot manager"));
    assert_eq!(refused(Spec { log: l, ..Spec::default() }), Refusal::Loader);
    let mut l = Log::empty();
    l.ext(0, 1, digest(b"crtm")).ext(5, EV_APP, loader());
    assert_eq!(refused(Spec { log: l, ..Spec::default() }), Refusal::Loader);
}

#[test]
fn listed_applications_in_any_order_pass() {
    let mut s = Spec::default();
    s.log.ext(4, EV_APP, digest(b"boot manager"));
    passes(s);
    let mut l = Log::empty();
    l.ext(4, EV_APP, loader());
    passes(Spec { log: l, boot_apps: vec![], ..Spec::default() });
}
