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

//! What a driver may read off its station.

use super::station::SoftStation;
use crate::softmac::radio::{Env, Radio};
use crate::softmac::scanner::Scanner;
use crate::softmac::session::Session;
use crate::softmac::stats::LinkStats;

impl<R: Radio, E: Env> SoftStation<R, E> {
    pub fn radio(&self) -> Option<&R> {
        self.radio.as_ref()
    }

    pub fn radio_mut(&mut self) -> Option<&mut R> {
        self.radio.as_mut()
    }

    pub fn session(&self) -> Option<&Session> {
        self.session.as_ref().filter(|s| s.is_up())
    }

    pub fn stats(&self) -> LinkStats {
        self.stats
    }

    pub fn scanner(&self) -> &Scanner {
        &self.scanner
    }
}
