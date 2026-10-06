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

//! One request held for a job to carry, and its answer held for the call
//! that asked. nonos_git asks its transport and waits for the answer; a job
//! cannot wait. So a clone runs each git step twice: the first run asks,
//! is told nothing came, and leaves its request here; the job carries it a
//! tick at a time; the second run asks again and is handed the answer,
//! once. Each step (the ref advertisement, the pack) asks one request, so
//! one slot each way is enough, and an answer is never handed to a request
//! it was not for.

use alloc::vec::Vec;

#[derive(Default)]
pub struct Ask {
    asked: Option<Vec<u8>>,
    answer: Option<(Vec<u8>, Vec<u8>)>,
}

impl Ask {
    pub const fn new() -> Self {
        Ask { asked: None, answer: None }
    }

    /// The answer to `request` if one is held, taken; otherwise `request`
    /// is kept for the job to carry and None says to come back.
    pub fn request(&mut self, request: Vec<u8>) -> Option<Vec<u8>> {
        if let Some((asked, _)) = &self.answer {
            if *asked == request {
                return self.answer.take().map(|(_, body)| body);
            }
        }
        self.answer = None;
        self.asked = Some(request);
        None
    }

    /// The request the last run left, for the job to carry.
    pub fn take_asked(&mut self) -> Option<Vec<u8>> {
        self.asked.take()
    }

    /// What came back for `request`.
    pub fn answered(&mut self, request: Vec<u8>, body: Vec<u8>) {
        self.answer = Some((request, body));
    }
}
