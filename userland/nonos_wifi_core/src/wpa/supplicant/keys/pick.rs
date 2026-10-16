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
    let mut gtk = [0u8; GROUP_KEY_LEN];
    gtk.copy_from_slice(g.key);
    let igtk = match kd.igtk {
        Some(i) => {
            if i.key.len() != GROUP_KEY_LEN || !(4..=5).contains(&i.key_id) {
                return None;
            }
            let mut key = [0u8; GROUP_KEY_LEN];
            key.copy_from_slice(i.key);
            Some((key, i.key_id, i.ipn))
        }
        None => None,
    };
    Some(GroupKeys { gtk, gtk_id: g.key_id, igtk, rsc: 0 })
}
