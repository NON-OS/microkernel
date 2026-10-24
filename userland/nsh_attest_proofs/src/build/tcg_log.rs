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

//! A whole log, and the PCR values replaying it gives. The standard log is
//! shaped like a firmware's: the CRTM, configuration, option ROMs, Secure
//! Boot state, separators, the boot manager and last the loader in PCR 4,
//! and a PCR 5 event the checker does not replay.

use super::tcg::{header, EV_ACTION, EV_APP, EV_SEPARATOR};
use super::tcg_event::{digest, ev, extend, note_ev};

#[derive(Clone)]
pub struct Log {
    pub bytes: Vec<u8>,
    pub pcrs: [[u8; 32]; 24],
}

/// The action firmware logs before it starts an application.
pub const CALLING: &[u8] = b"Calling EFI Application from Boot Option";

/// The release's loader, as its Authenticode digest.
pub fn loader() -> [u8; 32] {
    digest(b"nonos loader")
}

impl Log {
    pub fn empty() -> Self {
        Log { bytes: header(), pcrs: [[0; 32]; 24] }
    }

    pub fn ext(&mut self, pcr: u32, kind: u32, d: [u8; 32]) -> &mut Self {
        self.bytes.extend(ev(pcr, kind, d));
        self.pcrs[pcr as usize] = extend(&self.pcrs[pcr as usize], &d);
        self
    }

    /// A separator or action, its digest its body's.
    pub fn note(&mut self, pcr: u32, kind: u32, body: &[u8]) -> &mut Self {
        self.bytes.extend(note_ev(pcr, kind, body));
        self.pcrs[pcr as usize] = extend(&self.pcrs[pcr as usize], &digest(body));
        self
    }

    pub fn standard() -> Self {
        let mut l = Log::empty();
        l.ext(0, 0x8, digest(b"crtm"));
        l.ext(1, 0x8000_0006, digest(b"boot order"));
        l.ext(2, 0x8000_0003, digest(b"option rom"));
        l.ext(7, 0x8000_0001, digest(b"secure boot"));
        for pcr in 0..8 {
            l.note(pcr, EV_SEPARATOR, &[0; 4]);
        }
        l.note(4, EV_ACTION, CALLING);
        l.ext(4, EV_APP, digest(b"boot manager"));
        l.ext(4, EV_APP, loader());
        l.note(5, EV_ACTION, b"Exit Boot Services Invocation");
        l
    }
}
