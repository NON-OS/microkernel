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

//! DDC/CI: the messages and their checks, the allowlist, the timing, both
//! carriers, and the controls on a claimed output through the wire.

mod aux_tests;
pub(crate) mod fake_bus;
mod frame_tests;
mod gmbus_tests;
mod pace_tests;
mod refuse_tests;
mod reply_tests;
mod rig;
mod vcp_tests;
mod wire_tests;
