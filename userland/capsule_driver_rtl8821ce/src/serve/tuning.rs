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

//! The radio's channels and frame size, which the scanner, the join's
//! beacon hunt and the radio's bring-up share, and the serving loop's pace.

/// The scan's channels and frame size, kept with the scanner so its receive
/// step is whole without the rest of `serve`.
pub(super) use super::scanner::{SCAN_CHANNELS, SCAN_FRAME_MAX};
/// The channel the radio parks on until a scan retunes it. Channel 1 is always
/// legal and is where a passive scan starts.
pub(super) const DEFAULT_CHANNEL: u8 = 1;

/// The driver's own service inbox, where requests arrive.
pub(super) const SERVICE_INBOX: u64 = 0;
/// How long the serve loop waits for a request before taking an idle scan step.
/// Short enough that a background sweep makes steady progress, long enough that the
/// loop is not busy-waiting.
pub(super) const IDLE_TIMEOUT_MS: u64 = 100;
