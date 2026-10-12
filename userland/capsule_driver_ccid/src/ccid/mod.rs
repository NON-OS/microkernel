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
mod class_desc;
mod exchange;
mod msg;
mod notice;
mod reader;
mod reply;
mod slots;

pub use exchange::exchange;
pub use msg::{answers, command, GET_SLOT_STATUS, HEADER, ICC_POWER_OFF, ICC_POWER_ON, XFR_BLOCK};
pub use reader::{find_reader, Reader};
pub use reply::{reply, Outcome};
pub use slots::Slots;
