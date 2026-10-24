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

//! `ESCALATE_RESULT`. A granted escalation names when it ends, and a
//! refused one names nothing.

use crate::{Reader, Reason, WireError, Writer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EscalateResult {
    pub reason: Reason,
    pub expires_ms: u64,
}

impl EscalateResult {
    pub fn parse(body: &[u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(body);
        let res = EscalateResult { reason: Reason::from_code(r.u16()?)?, expires_ms: r.u64()? };
        r.finish()?;
        res.check().map(|_| res)
    }

    fn check(&self) -> Result<(), WireError> {
        let ok = match self.reason {
            Reason::Ok => self.expires_ms != 0,
            r => r.refuses_escalation() && self.expires_ms == 0,
        };
        ok.then_some(()).ok_or(WireError::Value)
    }

    pub fn encode(&self, out: &mut [u8]) -> Result<usize, WireError> {
        self.check()?;
        let mut w = Writer::new(out);
        w.u16(self.reason.code())?;
        w.u64(self.expires_ms)?;
        Ok(w.done())
    }
}
