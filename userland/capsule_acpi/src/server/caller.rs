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

//! Who is asking. The port is held to `power` and the desktop shell
//! (`HELD`), but PREPARE_SLEEP runs _PTS, which puts firmware in the state
//! it leaves just before power goes; only `capsule_power`, about to call
//! AdminShutdown, may ask for that. Its pid is the registry's answer for
//! the `power` service, looked up at each request so a restart is seen.

use nonos_libc::mk_service_lookup;

const POWER: &[u8] = b"power";

pub fn is_power(sender: u32) -> bool {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(POWER.as_ptr(), POWER.len(), &mut port, &mut pid);
    rc >= 0 && pid != 0 && pid == sender
}
