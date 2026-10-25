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

//! `power.off`, this capsule's second name: the desktop shell's request for
//! the wiped power-off (G-53's frame, `wire`). The kernel registers the name
//! for this capsule when it spawns it, and its HELD line lets only the shell
//! address it; both names land in one inbox, so the sender is checked here
//! again. A request that passes has firmware prepare soft-off, then asks the
//! kernel for AdminShutdown, which is zerostate's terminate: the leave phase,
//! the wipe, soft-off. Only a refusal is answered: a power-off that works
//! never returns.

pub mod rule;
/// The frame, as capsule_acpi serves it too: one definition of G-53's wire.
#[path = "../../../capsule_acpi/src/power/off/wire.rs"]
pub mod wire;

use nonos_libc::mk_admin_shutdown;

use crate::server::prepare_soft_off;
use rule::{verdict, Verdict};

/// Serve one `power.off` request; the reply's length in `out` (always a
/// refusal: when the power-off runs, the call does not come back).
pub fn requested(req: &[u8], from_shell: bool, out: &mut [u8]) -> usize {
    let status = match verdict(wire::well_formed(req), from_shell) {
        Verdict::Refuse(e) => e,
        Verdict::Off => {
            // Whatever firmware answers, the power-off goes on.
            let _ = prepare_soft_off();
            mk_admin_shutdown() as i32
        }
    };
    let reply = wire::refusal(req, status);
    out[..reply.len()].copy_from_slice(&reply);
    reply.len()
}
