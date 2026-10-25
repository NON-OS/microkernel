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

//! The order of a take-back's steps in the takeover model (L79): as built,
//! or with the release moved before the repoint (ReleaseFirst); after the
//! last step the takeover reads bootfb's claim again.

use super::knob::Knob;
use super::taker::PEEK;

pub(super) const RETIRE: u8 = 4;
pub(super) const INFLIGHT: u8 = 5;
pub(super) const DRAIN: u8 = 6;
pub(super) const REPOINT: u8 = 7;
pub(super) const FLUSH: u8 = 8;
pub(super) const RELEASE: u8 = 9;
pub(super) const SETTLE: u8 = 10;

pub(super) fn then(k: Knob, pc: u8) -> u8 {
    let built = [RETIRE, INFLIGHT, DRAIN, REPOINT, FLUSH, RELEASE, SETTLE];
    let first = [RETIRE, INFLIGHT, DRAIN, RELEASE, SETTLE, REPOINT, FLUSH];
    let order = if k == Knob::ReleaseFirst { first } else { built };
    let at = order.iter().position(|&p| p == pc).map_or(order.len(), |i| i + 1);
    order.get(at).copied().unwrap_or(PEEK)
}
