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

//! Who a function's last holder was, by the driver service it served (L78).
//! A restarted driver is a new process with the same service, so a function
//! no reset could be trusted on goes back to that service only. Names under
//! `driver.` are the system's to give (L52), so no other capsule takes one.

pub(crate) const NAME_MAX: usize = 32;
const ROOT: &str = "driver.";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Name {
    len: u8,
    bytes: [u8; NAME_MAX],
}

/// The least `driver.` name among those served, one that fits; none when
/// the claimant serves no driver name.
pub(crate) fn name_of<'a>(served: impl IntoIterator<Item = &'a str>) -> Option<Name> {
    let ours = |n: &&str| n.starts_with(ROOT) && n.len() <= NAME_MAX;
    let least = served.into_iter().filter(ours).min()?;
    let mut bytes = [0; NAME_MAX];
    bytes[..least.len()].copy_from_slice(least.as_bytes());
    Some(Name { len: least.len() as u8, bytes })
}

/// A record and a claimant of the same service: none never matches.
pub(crate) fn same(record: Option<Name>, claimant: Option<Name>) -> bool {
    matches!((record, claimant), (Some(a), Some(b)) if a == b)
}
