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

//! A login's second part, what the grant allows: the fleet, the time, a
//! fresh revocation list that does not name it, the role, and capabilities
//! the host knows within the role's ceiling. Each has its own code.

use crate::build::grant::GrantB;
use crate::build::run::Setup;
use crate::build::NOW;
use nsh_roles::caps::FORBIDDEN;
use nsh_roles::Refusal;

pub fn at(now: u64, g: GrantB) -> Result<(), Refusal> {
    Setup { now, ..Setup::default() }.login(&g).map(|_| ())
}

pub fn login(g: GrantB) -> Result<(), Refusal> {
    at(NOW, g)
}

#[test]
fn a_grant_for_another_fleet_is_refused() {
    assert_eq!(login(GrantB { fleet: [0xF2; 16], ..GrantB::default() }), Err(Refusal::Fleet));
}

#[test]
fn a_grant_holds_from_not_before_until_just_before_not_after() {
    let g = GrantB::default;
    let (nb, na) = (g().not_before_ms, g().not_after_ms);
    assert_eq!(at(nb - 1, g()), Err(Refusal::NotYet));
    assert_eq!(at(nb, g()), Ok(()));
    assert_eq!(at(na - 1, g()), Ok(()));
    assert_eq!(at(na, g()), Err(Refusal::Expired));
}

#[test]
fn a_role_the_table_lacks_is_refused() {
    assert_eq!(
        login(GrantB { role: b"root".to_vec(), ..GrantB::default() }),
        Err(Refusal::RoleUnknown)
    );
}

#[test]
fn every_capability_is_known_and_within_the_ceiling() {
    let caps = |c: Vec<&'static str>| login(GrantB { caps: c, ..GrantB::default() });
    assert_eq!(caps(vec!["CoreExec", "Root"]), Err(Refusal::CapUnknown));
    assert_eq!(caps(vec!["CoreExec", "Network"]), Err(Refusal::CapsCeiling));
    for name in FORBIDDEN {
        assert_eq!(caps(vec!["CoreExec", name]), Err(Refusal::CapsCeiling), "{name}");
    }
    let ceiling = vec!["AttestRead", "CoreExec", "FileSystem", "IPC", "Memory", "ProcessControl"];
    assert_eq!(caps(ceiling), Ok(()));
}
