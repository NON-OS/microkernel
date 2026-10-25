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

use crate::protocol::Request;
use crate::server::respond;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender_pid: u32,
    req: &Request,
    tx: &mut [u8],
) -> Result<(), &'static str> {
    // Input is the input router's to deliver, to the window the window
    // manager focused; a client subscribing here must not make itself the
    // focused one. Answered 0 for the clients that still send it.
    let _ = ctx;
    respond::status(sender_pid, req, 0, tx)
}
