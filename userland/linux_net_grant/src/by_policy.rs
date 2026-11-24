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

//! A grant by the program's own policy. The front never verifies a trailer
//! itself: the personality had the kernel verify the policy's local trailer
//! inside the family (capsule_linux app/bind.rs, after the guest exists), and
//! the kernel records only what it accepted. The policy is vouched for under
//! its own domain (nonos_linux_policy::vouch), so the measurement looked for
//! is `vouched_hash`, which only a policy's trailer gives: a program or any
//! other blob the machine or a publisher signed that happens to parse as a
//! policy is measured under its plain BLAKE3 and is refused here. The kernel
//! also says which verification recorded each measurement, and the policy's
//! must be a local trailer's: a publisher's chain over bytes that are the
//! domain and a policy, which a compromised personality could have the kernel
//! verify, is not taken for a vouched policy. Its program must be among the
//! measurements too, proved either way in this family before it ran.

use nonos_linux_policy::{decode, mask::NET_ANON, vouch::vouched_hash};

use super::{Facts, Grant, Refusal, LOCAL, PROVED};

pub fn by_policy(pid: u32, policy: &[u8], facts: &impl Facts) -> Result<Grant, Refusal> {
    let p = decode(policy).map_err(|_| Refusal::NotAPolicy)?;
    if p.mask & NET_ANON == 0 {
        return Err(Refusal::NoNetAnon);
    }
    if !facts.holds(pid, &vouched_hash(policy), LOCAL) {
        return Err(Refusal::PolicyNotVouched);
    }
    if !facts.holds(pid, &p.program_blake3, PROVED) {
        return Err(Refusal::ProgramNotProved);
    }
    Ok(Grant::Policy { program: p.program_blake3 })
}
