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

mod bring;
mod config;
pub mod control;
mod crc;
mod deslip;
mod engine;
mod hci;
pub mod header;
mod heard;
mod inbound;
mod link;
mod out;
mod packet;
mod reliable;
mod resend;
pub mod slip;
mod transport;

pub use bring::SEND_LOOKS;
pub use config::{Agreed, OURS};
pub use crc::crc;
pub use deslip::{Deslip, FRAME_MAX};
pub use engine::H5;
pub use hci::H5Hci;
pub use link::{Link, State, RESEND_MS};
pub use packet::{build, open, Bad};
pub use reliable::{Reliable, LOST_MS};
