// NONOS Operating System (AGPL-3.0-or-later)

//! Keys a handshake delivered on the link, held until the firmware has them.

use nonos_wifi_core::key::HeldKey;

use super::state::Link;

impl Link {
    /// Hold a new group key (index, key) and a rekeyed pairwise key; each
    /// replaces one still waiting, wiped where it lies.
    pub(super) fn hold_keys(&mut self, group: Option<(u8, [u8; 16])>, pairwise: Option<[u8; 16]>) {
        if let Some((id, key)) = group {
            self.new_group_key = Some((id, HeldKey(key)));
            self.stats.rekeys = self.stats.rekeys.wrapping_add(1);
        }
        if let Some(tk) = pairwise {
            self.new_pairwise_key = Some(HeldKey(tk));
            self.stats.rekeys = self.stats.rekeys.wrapping_add(1);
        }
    }
}
