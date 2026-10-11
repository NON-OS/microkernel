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

//! The server's flight, gathered until it can be judged.

use alloc::vec::Vec;

use super::super::gather::Gathered;
use super::io::judged;
use super::said::NO_ANSWER;
use super::state::Exchange;
use super::types::Stage;

impl Exchange {
    pub(super) fn flight(&mut self) -> Result<Option<Vec<u8>>, &'static str> {
        /* Ready once a whole Finished came or the flight cannot
         * finish, however many records the server packed it into.
         * Each record is opened once across the slices. */
        let flight = self.flight.take().ok_or(NO_ANSWER)?;
        let mut handshake = self.handshake.take();
        let done = |bytes: &[u8]| judged(&flight, &mut handshake, bytes);
        let gathered = self.gather_slice(done);
        self.flight = Some(flight);
        self.handshake = handshake;
        match gathered {
            Gathered::More => Ok(None),
            Gathered::Nothing => Err(NO_ANSWER),
            Gathered::Whole(server) => {
                self.server = server;
                self.now = super::super::super::rtc_stamp::rtc_stamp();
                self.stage = Stage::Check;
                Ok(None)
            }
        }
    }
}
