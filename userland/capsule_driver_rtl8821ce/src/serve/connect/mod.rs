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

//! Joining and leaving a network. A connect finds the target's beacon on the air
//! (sending a directed probe only for a network the person marked hidden), runs
//! the whole join through the association driver under the request's policy
//! (WPA3-SAE whenever offered, WPA2 otherwise, never WPA2 for a network saved
//! as WPA3), then installs the negotiated keys into the hardware CAM, hands
//! the data path the supplicant for the AP's later group key handshakes, and
//! records the association. A disconnect tells the AP, drops the association
//! and clears the keys.

mod consts;
mod ends;
mod entropy;
mod hunt;
mod install;
mod join;
mod leave;
mod media;
mod probe;
mod progress;
mod request;
pub(crate) mod result;
mod run;
mod session;

pub(super) use join::connect;
pub(super) use leave::disconnect;
pub(super) use media::set_media_no_link;
pub(super) use result::ConnectResult;
pub(super) use session::Session;
