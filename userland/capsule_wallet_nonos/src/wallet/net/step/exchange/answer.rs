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

//! The answer gathered, opened, and read only as a whole 200.

use alloc::vec::Vec;

use nonos_tls::AppReader;

use super::super::gather::Gathered;
use super::said::{CUT_RECORD, CUT_SHORT, HTTP_REFUSED, NO_ANSWER, NO_REPLY, RECORD_FAILED};
use super::state::Exchange;

impl Exchange {
    pub(super) fn answer(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        match self.gather_slice(|_| false) {
            Gathered::More => Ok(None),
            Gathered::Nothing => Err(NO_REPLY),
            Gathered::Whole(answer) => {
                self.answer_len = answer.len();
                let app = self.app.as_ref().ok_or(NO_ANSWER)?;
                /* Records that followed the server's Finished in its
                 * flight are numbered before the answer's. */
                let mut wire = self.server.get(self.flight_end..).unwrap_or(&[]).to_vec();
                wire.extend_from_slice(&answer);
                let mut reader = AppReader::new();
                reader.feed(app, &wire);
                /* Why the answer is not one: a record that fails its tag,
                 * or one cut partway. */
                if reader.is_broken() {
                    return Err(RECORD_FAILED);
                }
                if !crate::wallet::tls13::records_whole(&wire) {
                    return Err(CUT_RECORD);
                }
                let plain = reader.into_plaintext();
                /* Only a whole 200 answer is one: an error status or a
                 * body cut short is said, never read as values. */
                match crate::wallet::rpc::http_answer(&plain) {
                    crate::wallet::rpc::Http::Body(body) => Ok(Some(body)),
                    crate::wallet::rpc::Http::Status(_) => Err(HTTP_REFUSED),
                    crate::wallet::rpc::Http::Incomplete => Err(CUT_SHORT),
                }
            }
        }
    }
}
