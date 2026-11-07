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

//! What `nsh` reads before it connects: the fleet's authorities and
//! withdrawn enrolments, its accepted releases, the user's grant, and any
//! escalation grants, with the client's clock.

use super::{args, files};
use crate::{Config, Policy};
use alloc::string::String;
use alloc::vec::Vec;

pub type Loaded = (Config, Policy, nsh_attest::Policy, Vec<Vec<u8>>);

pub fn load(a: &args::Args) -> Result<Loaded, String> {
    let authorities =
        a.authorities.iter().map(|f| files::authority(f)).collect::<Result<_, _>>()?;
    let revoked = a.revoked.iter().map(|f| files::revoked(f)).collect::<Result<Vec<_>, _>>()?;
    let policy = Policy { fleet_id: a.fleet, authorities, revoked: revoked.concat() };
    let releases = files::releases(&a.releases)?;
    let fleet = nsh_attest::Policy::new(
        a.fleet,
        nsh_authorities(&policy),
        releases,
        policy.revoked.clone(),
    )
    .map_err(|e| alloc::format!("{}: {e:?}", a.releases))?;
    let escalate = a.escalate.iter().map(|f| files::read(f)).collect::<Result<Vec<_>, _>>()?;
    let now_ms =
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|_| "clock")?;
    let cfg = Config {
        host: a.host.as_bytes().to_vec(),
        now_ms: now_ms.as_millis() as u64,
        policy: policy.clone(),
        grant: files::read(&a.grant)?,
    };
    Ok((cfg, policy, fleet, escalate))
}

fn nsh_authorities(p: &Policy) -> Vec<nsh_attest::Authority> {
    p.authorities.iter().map(|k| nsh_attest::Authority::new(k.ed, k.ml.clone())).collect()
}
