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


//! How a Linux run the Terminal started says it must start again with the
//! network: an exit status no guest can give, as a guest's is one byte.

/// The terminal role's exit once the kernel holds its ticket
/// (`mk_linux_run_net`): its Terminal starts the same argv once as
/// `tool.linux.net`, and any other status ends the run as ever.
pub const LINUX_ASK_NET: i32 = 0x004E_4554;
