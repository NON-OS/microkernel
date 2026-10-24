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

//! What the tool holds a record to when told: `--session`, `--subject`,
//! and `--revoked`, which works offline since it needs no clock.

use super::cli_kit::{dir, file, hex, tool};
use super::own::{session_record, Other};
use crate::kit::enrol::EnrolSpec;
use crate::kit::keys;
use nonos_hash::sha512;

#[test]
fn the_tool_requires_the_session_and_subject_it_is_given() {
    let d = dir("bound");
    let (good, other) =
        (session_record([0x7A; 64], Other::Nothing), session_record([0x7A; 64], Other::Session));
    let session = hex(&[0x5E; 16]);
    assert_eq!(tool(&d, &good, &["--session", &session]).0, 0);
    let (code, lines) = tool(&d, &other, &[]);
    assert_eq!(code, 0, "with no --session, another session's record is shown as such");
    assert!(lines[0].starts_with(&format!("session 5f{}", &session[2..])));
    let (code, lines) = tool(&d, &other, &["--session", &session]);
    assert_eq!((code, lines[0].as_str()), (1, "record refused: Unbound(SessionId)"));
    let subject = hex(&keys::user().digest());
    assert_eq!(tool(&d, &good, &["--subject", &subject]).0, 0);
    let stranger = hex(&keys::stranger().digest());
    let (code, lines) = tool(&d, &good, &["--subject", &stranger]);
    assert_eq!((code, lines[0].as_str()), (1, "record refused: Unbound(Subject)"));
    assert_eq!(tool(&d, &good, &["--session", "5e"]).0, 2);
}

#[test]
fn the_tool_refuses_a_withdrawn_enrolment_offline() {
    let d = dir("revoked");
    let good = session_record([0x7A; 64], Other::Nothing);
    let withdrawn = file(&d, "revoked", &[[0u8; 64], sha512(&EnrolSpec::good().bytes())].concat());
    let (code, lines) = tool(&d, &good, &["--revoked", &withdrawn]);
    assert_eq!((code, lines[0].as_str()), (1, "record refused: Host(Enrol(Revoked))"));
    let other = file(&d, "other", &[0x33u8; 64]);
    assert_eq!(tool(&d, &good, &["--revoked", &other]).0, 0);
    let ragged = file(&d, "ragged", &[0u8; 63]);
    assert_eq!(tool(&d, &good, &["--revoked", &ragged]).0, 2);
}
