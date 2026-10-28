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

//! What an update carries: what an install carries from the running boot,
//! but setup's answers from the disk being updated, so the system goes on
//! with the answers it was set up with and not the ones given to the stick.
//! The disk's are taken only when they check as setup's own reader checks
//! them; with none such there, the running boot's go, as an install's do.

use alloc::string::String;
use alloc::vec::Vec;

use nonos_policy_proto::setup_record::{check_record, ANSWERS_PATH};

use super::answers::kept_file;
use crate::carry::CarrySource;
use crate::sink::BlockSink;

pub struct DiskAnswers<'a> {
    boot: &'a mut dyn CarrySource,
    answers: Option<Vec<u8>>,
}

impl<'a> DiskAnswers<'a> {
    /// The disk's answers are read here, before an update writes anything.
    pub fn new(disk: &mut dyn BlockSink, boot: &'a mut dyn CarrySource) -> DiskAnswers<'a> {
        let path = core::str::from_utf8(ANSWERS_PATH).ok();
        let raw = path.and_then(|p| kept_file(disk, p));
        DiskAnswers { boot, answers: raw.filter(|r| check_record(r).is_ok()) }
    }

    /// Whether the answers carried are the disk's own.
    pub fn from_disk(&self) -> bool {
        self.answers.is_some()
    }
}

impl CarrySource for DiskAnswers<'_> {
    fn list(&mut self, prefix: &str) -> Vec<String> {
        self.boot.list(prefix)
    }

    fn read(&mut self, path: &str) -> Option<Vec<u8>> {
        match &self.answers {
            Some(raw) if path.as_bytes() == ANSWERS_PATH => Some(raw.clone()),
            _ => self.boot.read(path),
        }
    }
}
