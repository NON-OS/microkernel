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

//! Whether a directed advertisement is aimed at this host. A controller
//! scanning with the extended filter policy passes on one whose TargetA is
//! a resolvable private address it could not resolve (Vol 4, Part E,
//! 7.7.65.11); only the host holds the IRK that makes its own addresses,
//! so only the host can tell whether the address was one of them, whether
//! the current one or one it gave up since (Vol 6, Part B, 1.3.2.3).

use super::env::Env;
use super::find::Peer;
use super::us::Us;
use nonos_bt_hci::HciError;
use nonos_bt_smp::privacy::resolves;
use nonos_bt_wire::addr::BdAddr;
use nonos_bt_wire::hci::directed::directed_reports;

/// True only when this host has a key and `target` resolves under it. An
/// address that is not resolvable private resolves under no key, so a
/// directed advertisement to anyone's public or static address is never
/// taken as ours here.
pub fn aimed_here(target: &BdAddr, irk: Option<&[u8; 16]>) -> bool {
    irk.is_some_and(|k| resolves(k, target))
}

/// The first bonded peer in one LE Directed Advertising Report subevent
/// aimed here. Only a bond makes a directed advertisement worth taking, as
/// for ADV_DIRECT_IND in an Advertising Report.
pub(super) fn directed<E: Env>(sub: &[u8], env: &mut E, us: &Us) -> Result<Option<Peer>, HciError> {
    let irk = us.identity.as_ref().map(|i| &i.irk);
    for r in directed_reports(sub)? {
        if aimed_here(&r.direct_addr, irk) && env.bonded(r.addr_type, &r.addr).is_some() {
            return Ok(Some(Peer { kind: r.addr_type, addr: r.addr }));
        }
    }
    Ok(None)
}
