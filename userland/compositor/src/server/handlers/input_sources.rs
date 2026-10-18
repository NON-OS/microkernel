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

//! NCMP 0x0017 INPUT_SOURCES: the inputs each display's capabilities list
//! and the one it shows now, for the shell's input menu, from the shell
//! alone (a monitor's inputs are a small fingerprint). One reading per
//! output, each bounded by the driver call's wait; it is asked only when
//! a person opens the menu. Every answer carries the fixed payload.

use crate::display::brightness_rule::identity;
use crate::display::input_list::{encode, offered, request, Offered, INPUT_SOURCES_LEN};
use crate::protocol::{Request, E_INVAL, E_NOENT, E_PERM};
use crate::server::respond;
use crate::state::Context;

pub fn handle(
    ctx: &mut Context,
    sender: u32,
    req: &Request,
    body: &[u8],
    tx: &mut [u8],
) -> Result<(), &'static str> {
    let refuse = |status, tx: &mut [u8]| {
        respond::status_payload(sender, req, status, &[0; INPUT_SOURCES_LEN], tx)
    };
    if !super::shell_only::from_shell(sender) {
        return refuse(E_PERM, tx);
    }
    if !body.is_empty() {
        return refuse(E_INVAL, tx);
    }
    let Some(port) = super::driver_call::brightness_port(ctx) else { return refuse(E_NOENT, tx) };
    let (outputs, n) = super::driver_call::outputs(ctx);
    let mut found: alloc::vec::Vec<Offered> = alloc::vec::Vec::new();
    for &output in &outputs[..n] {
        let (id, mut reply) = (identity(output), [0u8; 64]);
        let got = super::driver_call::call(port, &request(id), &mut reply);
        found.extend(got.and_then(|k| offered(id, &reply[..k])));
    }
    respond::status_payload(sender, req, 0, &encode(&found), tx)
}
