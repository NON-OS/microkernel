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

//! Following one connection's events. The record's header arrives before
//! `Started` names the session, so the bytes are held until then, never more
//! than the largest header (`RECORD_HEADER_MAX`); after
//! it, every `Record` is appended and every `Checkpoint` kept behind the
//! bytes it covers. A host whose keeper fails must stop the session: a
//! record that cannot be kept must not keep growing unseen.

use crate::files::SessionFiles;
use crate::{Store, CKPT_LEN};
use nsh_server::{Event, RECORD_HEADER_MAX};
use std::io::{self, Error, ErrorKind};

pub struct Keeper<'a> {
    store: &'a Store,
    held: Vec<u8>,
    files: Option<SessionFiles>,
    session_id: Option<[u8; 16]>,
}

impl<'a> Keeper<'a> {
    pub fn new(store: &'a Store) -> Keeper<'a> {
        Keeper { store, held: Vec::new(), files: None, session_id: None }
    }

    /// The session this keeper writes, once it has started.
    pub fn session_id(&self) -> Option<[u8; 16]> {
        self.session_id
    }

    pub fn keep(&mut self, event: &Event) -> io::Result<()> {
        match event {
            Event::Record(bytes) => self.record(bytes),
            Event::Started { session_id, .. } => self.start(session_id),
            Event::Checkpoint(ckpt) => self.ckpt(ckpt),
            Event::Closed(_) => self.files.as_mut().map_or(Ok(()), SessionFiles::sync),
            _ => Ok(()),
        }
    }

    fn record(&mut self, bytes: &[u8]) -> io::Result<()> {
        match self.files.as_mut() {
            Some(f) => f.append_record(bytes),
            None => {
                let held = self.held.len().checked_add(bytes.len());
                if held.is_none_or(|n| n > RECORD_HEADER_MAX) {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "more record before the session than a header",
                    ));
                }
                self.held.extend_from_slice(bytes);
                Ok(())
            }
        }
    }

    fn start(&mut self, session_id: &[u8; 16]) -> io::Result<()> {
        if self.files.is_some() {
            return Err(Error::new(ErrorKind::InvalidData, "a second session on one connection"));
        }
        let mut files = self.store.create(session_id)?;
        files.append_record(&self.held)?;
        self.held = Vec::new();
        self.files = Some(files);
        self.session_id = Some(*session_id);
        Ok(())
    }

    fn ckpt(&mut self, ckpt: &[u8]) -> io::Result<()> {
        let files = self
            .files
            .as_mut()
            .ok_or_else(|| Error::new(ErrorKind::InvalidData, "a checkpoint before the session"))?;
        if ckpt.len() != CKPT_LEN {
            return Err(Error::new(ErrorKind::InvalidData, "a checkpoint of the wrong length"));
        }
        files.append_ckpt(ckpt)
    }
}
