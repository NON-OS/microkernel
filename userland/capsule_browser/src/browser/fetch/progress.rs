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

use alloc::string::String;

use crate::browser::fetch::types::{Fetch, Phase};

/// A short human label for the current fetch phase, shown on the loading
/// screen so the reader sees real progress instead of a static "loading".
pub fn phase_label(phase: Phase) -> &'static str {
    match phase {
        Phase::Connecting | Phase::SocksHello | Phase::SocksMethod | Phase::SocksConnect => {
            "Connecting to"
        }
        Phase::TlsHello | Phase::TlsFlight => "Securing",
        Phase::SendReq => "Requesting",
        Phase::ReadBody => "Downloading",
        Phase::Decrypt | Phase::Done => "Rendering",
        Phase::Error => "Error",
    }
}

/// The status line for a navigation in flight: the phase and the host. It
/// changes only when the phase does, so a tick that moved nothing has
/// nothing new to draw.
pub fn status(f: &Fetch) -> String {
    alloc::format!("{} {}", phase_label(f.phase), f.url.host)
}
