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

//! PCR 4 forgeries. The TPM extends only digests, so an event's type is a
//! claim: only no-action, separator, action and application events are
//! admitted, and a separator or action must carry its own body's hash.

use nsh_attest::{LogFault, Refusal};

use super::{passes, refused};
use crate::build::tcg::{EV_ACTION, EV_APP, EV_SEPARATOR};
use crate::build::tcg_event::digest;
use crate::build::tcg_log::Log;
use crate::build::Spec;

fn with(f: fn(&mut Log)) -> Refusal {
    let mut s = Spec::default();
    f(&mut s.log);
    refused(s)
}

#[test]
fn an_application_relabelled_as_an_action_is_refused() {
    let r = with(|l| {
        l.ext(4, EV_ACTION, digest(b"a shell"));
    });
    assert_eq!(r, Refusal::Log(LogFault::Pcr4Digest));
    let mut s = Spec::default();
    s.log.ext(4, EV_SEPARATOR, digest(b"a shell"));
    assert_eq!(refused(s), Refusal::Log(LogFault::Pcr4Digest), "as a separator");
}

#[test]
fn an_application_relabelled_as_any_other_type_is_refused() {
    for kind in [0x0000_000D, 0x8000_0001, 0x8000_0004, 0x0000_0008] {
        let mut s = Spec::default();
        s.log.ext(4, kind, digest(b"a shell"));
        assert_eq!(refused(s), Refusal::Log(LogFault::Pcr4Event), "type {kind:#x}");
    }
}

#[test]
fn an_action_whose_body_hashes_to_its_digest_passes() {
    let mut s = Spec::default();
    s.log.note(4, EV_ACTION, b"Returning from EFI Application from Boot Option");
    passes(s);
    let mut s = Spec::default();
    s.log.note(4, EV_SEPARATOR, &[0xFF; 4]);
    s.log.ext(4, EV_APP, digest(b"boot manager"));
    passes(s);
}

#[test]
fn other_pcrs_keep_their_own_event_types() {
    let mut s = Spec::default();
    s.log.ext(5, 0x8000_0006, digest(b"gpt")).ext(7, 0x8000_00E0, digest(b"db"));
    passes(s);
}
