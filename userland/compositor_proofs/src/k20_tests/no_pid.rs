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

//! The compositor names itself by the endpoint Capsule.mk declares, the one
//! prime_gop_handle.rs looks up; the GOP path, which took its own pid for
//! the self-share before, asks the kernel for none.

#[test]
fn the_gop_path_names_the_compositor_by_its_endpoint_and_asks_no_pid() {
    // cap_audit reads MkGetPid, gated on CoreExec, from any call to it.
    let mk = include_str!("../../../compositor/Capsule.mk");
    assert!(mk.contains("CAPSULE_SERVICE_ENDPOINT := service:4310:compositor\n"));
    let named = include_str!("../../../compositor/src/setup/prime_gop_handle.rs");
    assert!(named.contains("pub const COMPOSITOR_SERVICE: &[u8] = b\"compositor\";"));
    let gop = include_str!("../../../compositor/src/setup/prime_gop.rs");
    assert!(gop.contains("own_handle(sid as u64, registered_pid, share)?"));
    assert!(!gop.contains("mk_getpid"), "the GOP path asks for its pid");
}
