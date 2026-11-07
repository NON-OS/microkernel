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

//! The nsh/1 host, in two capsules. nsh.attest (AttestRead, the host keys, no
//! Network) comes up first so nsh.front can resolve it; nsh.front (Network,
//! no AttestRead, no host secret) then opens the listener. Each is behind its
//! own feature, which the server profile (nonos-server) and development twins
//! turn on, and is absent from every other image.

#[cfg(feature = "nonos-capsule-nsh-attest")]
pub(super) fn spawn_nsh_attest() {
    use crate::userspace::capsule_nsh_attest as c;
    super::super::boot::capsule(
        "NSH-ATTEST",
        "nsh_attest",
        c::spawn_nsh_attest_capsule,
        c::shared_state,
    );
}

#[cfg(not(feature = "nonos-capsule-nsh-attest"))]
pub(super) fn spawn_nsh_attest() {}

#[cfg(feature = "nonos-capsule-nsh-front")]
pub(super) fn spawn_nsh_front() {
    use crate::userspace::capsule_nsh_front as c;
    super::super::boot::capsule("NSH-FRONT", "nsh_front", c::spawn_nsh_front_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-nsh-front"))]
pub(super) fn spawn_nsh_front() {}
