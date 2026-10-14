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

//! The Wi-Fi control protocol setup, Settings and net_core speak through
//! `nonos_wifi_client`, distinct from net_core's link protocol and tagged with
//! its own magic so the two never collide. Status, link and scan are answered
//! instantly; connect runs the whole join.

mod dispatch;
mod join;
mod leave;
mod link_op;
mod ops;
mod scan;
mod status_op;

pub(super) use dispatch::control;
use ops::WIFI_HDR;
