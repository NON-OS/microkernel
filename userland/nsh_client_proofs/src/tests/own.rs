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

//! A record whose header is this session's, or differs from it in one
//! field: the session id, the handshake, the subject or the document.

use crate::kit::enrol::EnrolSpec;
use crate::kit::keys;
use crate::kit::record::{header_with, record_doc, Rec, SESSION};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Other {
    Nothing,
    Session,
    Transcript,
    Subject,
    Document,
}

/// A signed record of five entries for the session whose
/// `TH(SERVER_HELLO)` is `th`, with `other` changed.
pub fn session_record(th: [u8; 64], other: Other) -> Vec<u8> {
    let mut named = th;
    let (mut session, mut subject, mut doc) = (SESSION, keys::user().digest(), record_doc(&th));
    match other {
        Other::Nothing => {}
        Other::Session => session[0] ^= 1,
        Other::Transcript => named[0] ^= 1,
        Other::Subject => subject = keys::stranger().digest(),
        Other::Document => doc = record_doc(&[0x11; 64]),
    }
    if other == Other::Transcript {
        doc = record_doc(&named);
    }
    let header = header_with(session, subject, &named, &EnrolSpec::good().bytes(), &doc);
    Rec::for_handshake(named, header, 5).bytes()
}
