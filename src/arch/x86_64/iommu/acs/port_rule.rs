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

//! Which bridges count as switch downstream ports (L76). The type a port
//! reports is not trusted alone: one may report itself upstream or a root
//! port, or carry no type a walk could read. A bridge counts when it says
//! downstream, when the bridge above it says upstream (it sits on a switch's
//! internal bus, whatever it says), when it has no type and a bridge above
//! it, or when the bridge above it is not borne out as a link's end by the
//! whole chain above (L77c). One counted too many only groups more; one left
//! out let its peers reach each other past the IOMMU. Pure, for the host.

use super::chain_rule::{chain, ends, naming};

/// PCIe Device/Port Type values (recalled, unverified): a root port, and a
/// switch's upstream port and a downstream one.
pub(super) const ROOT_PORT: u8 = 4;
pub(super) const UPSTREAM: u8 = 5;
pub(super) const DOWNSTREAM: u8 = 6;

/// `own`: the type the bridge reports, none when unread. `above`: the bridge
/// whose secondary bus is this one's own, by the type it reports; none when
/// no bridge is above it, as for a root port.
pub(super) const fn counts(own: Option<u8>, above: Option<Option<u8>>) -> bool {
    matches!(own, Some(DOWNSTREAM))
        || matches!(above, Some(Some(UPSTREAM)))
        || (own.is_none() && above.is_some())
}

/// A bridge as the pick reads it: its own bus, the secondary bus it names
/// (none when unread), and the type it reports.
pub(super) type Bridge = (u8, Option<u8>, Option<u8>);

/// Which bridges count, by index, every bridge naming a bridge's bus asked,
/// so a false one cannot hide the switch it sits in. A bridge names only a
/// bus below its own: one firmware left unset names bus 0, and would else
/// put every bridge of the root bus in a switch. Below a bridge only a
/// switch's two port types are believed, anything else taken as unread
/// (L77); and one saying upstream is believed only below a link's end that
/// the chain above bears out: a switch whose ports all lie, its upstream
/// port closed, would else group nothing below it (L77c).
pub(super) fn picked(bridges: &[Bridge]) -> impl Fn(usize) -> bool + '_ {
    let ups = chain(bridges);
    move |at| {
        let Some(&(bus, _, said)) = bridges.get(at) else { return false };
        let own = said.filter(|&t| t == UPSTREAM || t == DOWNSTREAM);
        let mut above = naming(bridges, bus).peekable();
        match above.peek() {
            None => counts(own, None),
            Some(_) => above.any(|b| counts(own, Some(b.2)) || !ends(bridges, &ups, b)),
        }
    }
}
