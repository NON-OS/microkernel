// NONOS Operating System (AGPL-3.0-or-later)
//! A station that leaves, or associates again, keeps no key of the session
//! before.

use super::{Ccmp, LinkStation};

#[test]
fn leaving_or_associating_again_forgets_every_key() {
    let (ap, other) = ([2, 0, 0, 0, 0, 1], [2, 0, 0, 0, 0, 9]);
    let mut st = LinkStation::new([2, 0, 0, 0, 0, 2]);
    st.associate(ap, Ccmp::Software { tk: [0x33; 16] });
    st.set_group_key(1, [0x44; 16], 0);
    assert_eq!((st.pairwise_key(), st.group_key(1)), (Some([0x33; 16]), Some([0x44; 16])));
    st.deassociate();
    assert_eq!((st.pairwise_key(), st.group_key(1)), (None, None), "gone on leaving");
    st.associate(ap, Ccmp::SoftwareTxHwRx { tk: [0x55; 16] });
    st.set_group_key(2, [0x66; 16], 0);
    st.associate(other, Ccmp::Hardware);
    assert_eq!((st.pairwise_key(), st.group_key(2)), (None, None), "gone on a new one");
}
