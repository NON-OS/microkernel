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

//! Each append handed to the host: the new record bytes, a checkpoint signed
//! when one is due, and a record with no room left stopping the shell.

use super::Server;
use crate::record::Unfit;
use crate::{EndReason, Event, HostSigner, Rand, Why};
use alloc::vec::Vec;

impl<A, S: HostSigner, R> Server<A, S, R> {
    pub(super) fn logged(
        &mut self,
        ckpt: Result<Option<Vec<u8>>, Unfit>,
        rand: Rand<'_>,
    ) -> Result<(), Unfit> {
        let Some(live) = self.live.as_mut() else { return Ok(()) };
        let new = live.record.take_new();
        if !new.is_empty() {
            self.events.push(Event::Record(new));
        }
        match ckpt {
            Ok(Some(mut fields)) => {
                let digest = crate::record::ckpt_digest(&fields);
                match self.signer.sign(&digest, rand) {
                    Some(sig) => {
                        fields.extend_from_slice(&sig.ed);
                        fields.extend_from_slice(&sig.ml[..]);
                        self.events.push(Event::Checkpoint(fields));
                    }
                    None => self.events.push(Event::Refused(Why::Sign)),
                }
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(Unfit::Full) => {
                self.stop(EndReason::Host);
                Err(Unfit::Full)
            }
            Err(e) => Err(e),
        }
    }
}
