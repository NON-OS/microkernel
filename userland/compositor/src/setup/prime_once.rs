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

//! The first announced driver, driven on the display wire. At start-up a
//! driver is announced as it is spawned, before its device is up, so its
//! first answer is waited for up to START_MS rather than SETUP_MS.

use super::discover;
use crate::display::call::START_MS;
use crate::state::Context;

pub fn run_display_once() -> Result<Context, &'static str> {
    let ep = discover::lookup_gfx_endpoint()?;
    super::prime_wire::prime_within(ep, START_MS).map_err(|u| u.what)
}
