// NONOS Operating System (AGPL-3.0-or-later)

//! The group keys picked out of parsed key data.

use super::store::{GroupKeys, GROUP_KEY_LEN};
use crate::eapol::kde::KeyData;

/// Pull the group keys out of parsed key data. `None` unless a GTK of the
/// CCMP-128 length is present, and any IGTK present is BIP-CMAC-128 sized with
/// an index of 4 or 5.
pub(in super::super) fn group_keys(kd: &KeyData<'_>) -> Option<GroupKeys> {
    let g = kd.gtk?;
    if g.key.len() != GROUP_KEY_LEN {
        return None;
    }
    // Built in place: the keys are copied once, into the holder that wipes
    // them when it goes.
    let mut keys = GroupKeys { gtk: [0; GROUP_KEY_LEN], gtk_id: g.key_id, igtk: None, rsc: 0 };
    keys.gtk.copy_from_slice(g.key);
    if let Some(i) = kd.igtk {
        if i.key.len() != GROUP_KEY_LEN || !(4..=5).contains(&i.key_id) {
            return None;
        }
        let (key, _, _) = keys.igtk.insert(([0; GROUP_KEY_LEN], i.key_id, i.ipn));
        key.copy_from_slice(i.key);
    }
    Some(keys)
}
