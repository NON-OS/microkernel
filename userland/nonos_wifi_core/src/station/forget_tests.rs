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

//! A station that leaves, or associates again, keeps no key of the session
//! before.

use super::{Ccmp, LinkStation};
use crate::rsn::TemporalKey;

#[test]
fn leaving_or_associating_again_forgets_every_key() {
    let (ap, other) = ([2, 0, 0, 0, 0, 1], [2, 0, 0, 0, 0, 9]);
    let (tk, gtk) = (TemporalKey::Ccmp128([0x33; 16]), TemporalKey::Ccmp128([0x44; 16]));
    let mut st = LinkStation::new([2, 0, 0, 0, 0, 2]);
    st.associate(ap, Ccmp::Software { tk: [0x33; 16] });
    st.set_group_key(1, TemporalKey::Ccmp128([0x44; 16]), 0);
    assert!(st.pairwise_key() == Some(tk) && st.group_key(1) == Some(gtk));
    st.deassociate();
    assert!(st.pairwise_key().is_none() && st.group_key(1).is_none(), "gone on leaving");
    st.associate(ap, Ccmp::SoftwareTxHwRx { tk: [0x55; 16] });
    st.set_group_key(2, TemporalKey::Ccmp128([0x66; 16]), 0);
    st.associate(other, Ccmp::Hardware);
    assert!(st.pairwise_key().is_none() && st.group_key(2).is_none(), "gone on a new one");
    assert!(!st.holds_keys());
}
