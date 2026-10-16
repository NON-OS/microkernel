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

//! Reading the handshake flight and the response, and judging the silences.

use alloc::vec::Vec;

use nonos_libc::mk_time_millis;

use super::super::types::Exchange;
use super::super::wait::{settle_handshake, settle_response, Settle};

impl Exchange {
    pub(super) fn flight(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let got = self.take()?;
        let now = mk_time_millis();
        if got > 0 {
            self.wait.heard(now);
            return self.advance();
        }
        let ended = self.stream.as_ref().is_some_and(|s| s.ended());
        match settle_handshake(self.wait.check(now), ended, self.anonymous) {
            Settle::More | Settle::Finish => Ok(None),
            Settle::Fail(why) => Err(why),
        }
    }

    pub(super) fn response(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        let got = self.take()?;
        let now = mk_time_millis();
        if got > 0 {
            self.wait.heard(now);
            self.open_records();
        }
        let ended = self.stream.as_ref().is_some_and(|s| s.ended());
        let heard = if got > 0 { super::super::wait::Heard::Waiting } else { self.wait.check(now) };
        match settle_response(heard, ended, self.anonymous) {
            Settle::More => Ok(None),
            Settle::Finish => Ok(Some(self.body())),
            Settle::Fail(why) => Err(why),
        }
    }

    /// The response as the caller reads it: decrypted for TLS, as it came
    /// for plain HTTP.
    fn body(&mut self) -> Vec<u8> {
        match self.tls.take() {
            Some(tls) => tls.reader.into_plaintext(),
            None => core::mem::take(&mut self.raw),
        }
    }
}
