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
//! The horizontal limits Vol 2c sets, written out here from its pages.

use crate::model::edge::edge;
use crate::model::mode::timing;
use nonos_intel_xelp_display::timing::check::{check, Sink};

#[test]
fn horizontal_limits() {
    let dp = Sink::DisplayPort;
    // Active at least 64 (p. 1365); even (PIPE_SRCSZ, p. 705).
    edge(dp, |t, p| t.h_active = if p { 62 } else { 64 });
    edge(dp, |t, p| t.h_active = if p { 641 } else { 640 });
    // Blank at least 32 (p. 1360); front and sync inside it (p. 1363).
    edge(dp, |t, p| (t.h_blank, t.h_front, t.h_sync) = (if p { 31 } else { 32 }, 1, 1));
    edge(dp, |t, p| t.h_front = if p { 0 } else { 1 });
    edge(dp, |t, p| t.h_sync = if p { 0 } else { 1 });
    edge(dp, |t, p| t.h_sync = if p { 144 } else { 143 });
    // Total within 14 bits (p. 1364).
    edge(dp, |t, p| (t.h_active, t.h_blank) = (8192, if p { 8193 } else { 8192 }));
}

#[test]
fn hdmi_limits() {
    // Active at least 256 on HDMI (p. 1365); 16 from blank to sync (p. 1363).
    edge(Sink::Hdmi, |t, p| t.h_active = if p { 254 } else { 256 });
    edge(Sink::Hdmi, |t, p| t.h_front = if p { 15 } else { 16 });
}

#[test]
fn source_width_13_bits() {
    // PIPE_SRCSZ 28:16 (p. 704): 8192 fits as 8191, 8194 does not.
    assert!(check(&timing(1, [8192, 32, 1, 1], [1, 5, 1, 1]), Sink::DisplayPort).is_ok());
    assert!(check(&timing(1, [8194, 32, 1, 1], [1, 5, 1, 1]), Sink::DisplayPort).is_err());
}
