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

//! The session's record: checked offline at once, and kept with the host's
//! document when --records names a directory.

use super::files::write_private;
use super::report::{hex, say, Report};
use super::session::Nsh;
use crate::{verify_record, Bound, Verdict};
use alloc::format;

impl Report<'_> {
    pub(super) fn record(&mut self, c: &Nsh, record: &[u8]) {
        let Some(doc) = c.host_document() else { return say("record: no document to check it by") };
        let expect =
            Bound { session_id: c.session_id(), transcript: c.th_server_hello(), subject: None };
        match verify_record(record, doc, &[], &expect, self.policy, c.attester()) {
            Ok(r) => match r.verdict {
                Verdict::Complete { entries, end_reason, .. } => say(&format!(
                    "record: complete and signed, {entries} entries, end reason {end_reason}"
                )),
                Verdict::CutShort { seq, .. } => {
                    say(&format!("record: cut short after entry {seq}"))
                }
            },
            Err(why) => say(&format!("record refused: {why:?}")),
        }
        if let (Some(dir), Some(id)) = (&self.records, c.session_id()) {
            // A record holds what was typed and printed, so it is kept as
            // the host keeps its own: readable by its owner alone, and
            // never written through a link or over a file already there.
            let base = format!("{dir}/{}", hex(&id));
            let at = |ext: &str| std::path::PathBuf::from(format!("{base}.{ext}"));
            let kept =
                write_private(&at("record"), record).and_then(|_| write_private(&at("doc"), doc));
            match kept {
                Ok(()) => say(&format!("record kept: {base}.record, {base}.doc")),
                Err(e) => say(&format!("record not kept: {e}")),
            }
        }
    }
}
