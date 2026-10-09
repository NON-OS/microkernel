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

//! One list at a time, in the order a route needs them.

use super::budget_roles::MIX_BUDGET;
use super::keep::keep;
use super::live::fetch_mixnodes;
use super::roles::layers_present;
use super::step::{Step, PARTIAL};

/// The mix layers, which are what a route is built from.
pub(super) fn first(tcp_port: u32) -> Step {
    match fetch_mixnodes(tcp_port) {
        Ok(nodes) if layers_present(&nodes) => {
            let nodes = keep(nodes, MIX_BUDGET, b"mix");
            *PARTIAL.lock() = Some((nodes, 1));
            Step::Progressed
        }
        /*
         * A list missing a layer cannot carry a route, and keeping it would
         * let a later step install something unroutable.
         */
        Ok(_) => Step::Failed(13),
        Err(code) => Step::Failed(code),
    }
}
