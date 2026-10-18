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

//! Where a run goes once its policy is bound: the run role, without Network,
//! asks again for a box with NET_ANON; the network role runs nothing else.

use crate::app::rules::{net_step, NetStep};
use nonos_linux_policy::mask::{KNOWN, MODELS_RO, NET_ANON, NET_LISTEN_LO, TTY, WAYLAND};

#[test]
fn a_box_without_net_anon_runs_in_the_plain_role() {
    for mask in [0, TTY, WAYLAND | MODELS_RO | TTY, NET_LISTEN_LO] {
        assert_eq!(net_step(mask, false), NetStep::Run, "{mask:#x}");
    }
}

#[test]
fn net_anon_in_the_plain_role_asks_to_start_again() {
    for mask in [NET_ANON, NET_ANON | WAYLAND, KNOWN] {
        assert_eq!(net_step(mask, false), NetStep::AskAgain, "{mask:#x}");
    }
}

#[test]
fn the_network_role_runs_only_net_anon() {
    for mask in [NET_ANON, NET_ANON | TTY, KNOWN] {
        assert_eq!(net_step(mask, true), NetStep::Run, "{mask:#x}");
    }
    // Listening at home is not reaching out, so it is no reason to hold Network.
    for mask in [0, NET_LISTEN_LO, WAYLAND | MODELS_RO | TTY, KNOWN & !NET_ANON] {
        assert_eq!(net_step(mask, true), NetStep::Refuse, "{mask:#x}");
    }
}
