// NONOS Operating System (AGPL-3.0-or-later)

//! The group keys a message delivered, and storing them.

use super::super::state::Supplicant;
use crate::ccmp::keywrap::{aes_unwrap, MAX_BLOCKS};

/// Bytes of key data the unwrap can hold.
pub(in super::super) const UNWRAPPED_MAX: usize = MAX_BLOCKS * 8;

/// The group cipher (CCMP-128) and group management cipher (BIP-CMAC-128) key
/// lengths.
pub(super) const GROUP_KEY_LEN: usize = 16;

/// The group keys one message delivered.
pub(in super::super) struct GroupKeys {
    pub(in super::super) gtk: [u8; GROUP_KEY_LEN],
    pub(in super::super) gtk_id: u8,
    pub(in super::super) igtk: Option<([u8; GROUP_KEY_LEN], u16, [u8; 6])>,
    /// The Key RSC of the message that delivered them.
    pub(in super::super) rsc: u64,
}

impl Supplicant {
    /// Unwrap `wrapped` under the KEK into `plain`, returning its length.
    pub(in super::super) fn unwrap_key_data(&self, wrapped: &[u8], plain: &mut [u8; UNWRAPPED_MAX]) -> Option<usize> {
        if wrapped.len() < 16 || !wrapped.len().is_multiple_of(8) {
            return None;
        }
        aes_unwrap(&self.kek(), wrapped, plain)
    }

    /// Store the group keys a message delivered. Returns whether the GTK is a
    /// different key from the one already held (a repeat is not reinstalled).
    pub(in super::super) fn store_group_keys(&mut self, k: &GroupKeys) -> bool {
        let changed = self.gtk_len != GROUP_KEY_LEN
            || self.gtk[..GROUP_KEY_LEN] != k.gtk
            || self.gtk_id != k.gtk_id;
        self.gtk[..GROUP_KEY_LEN].copy_from_slice(&k.gtk);
        self.gtk_len = GROUP_KEY_LEN;
        self.gtk_id = k.gtk_id;
        self.gtk_rsc = k.rsc;
        if let Some((key, id, ipn)) = k.igtk {
            self.igtk = key;
            self.igtk_id = id;
            self.igtk_ipn = ipn;
            self.igtk_set = true;
        }
        changed
    }
}
