/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The keys an address CAM entry points at, as Linux v6.12 rtw89 keeps them
//! for a station interface (`struct rtw89_addr_cam_entry`, normal security
//! mode, `rtw89_cam_get_addr_cam_key_idx`): slots 0 and 1 for the pairwise
//! key, 2 to 4 for group keys, 5 and 6 for the IGTK; each slot names a
//! security CAM entry and the key's index. Here a key's slot is fixed by
//! its index (pairwise 0, a group or integrity key its index plus one) and
//! the security CAM entry is the slot's own number, where rtw89 takes the
//! first free of each. In the CAM command (`rtw89_cam_fill_addr_cam_info`):
//! word 9 the two-bit key indices from bit 18, word 10 the valid bits and
//! slots 0 to 2's entries, word 11 slots 3 to 6's.

/// `RTW89_SEC_CAM_IN_ADDR_CAM`.
pub const SLOTS: usize = 7;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Keys {
    pub valid: u8,
    pub keyid: [u8; SLOTS],
}

impl Keys {
    /// The slot a key of index `id` takes, `None` past the IGTK's.
    pub fn slot(pairwise: bool, id: u8) -> Option<usize> {
        match (pairwise, id) {
            (true, 0) => Some(0),
            (false, 1..=5) => Some(usize::from(id) + 1),
            _ => None,
        }
    }

    pub fn set(&mut self, slot: usize, id: u8) {
        self.valid |= 1 << slot;
        self.keyid[slot] = id & 3;
    }

    pub fn has(&self, slot: usize) -> bool {
        self.valid & 1 << slot != 0
    }

    /// Words 9 (to OR in), 10 and 11 of the CAM command.
    pub fn words(&self) -> (u32, u32, u32) {
        let mut w9 = 0;
        let (mut w10, mut w11) = (u32::from(self.valid), 0u32);
        for s in (0..SLOTS).filter(|&s| self.has(s)) {
            w9 |= u32::from(self.keyid[s]) << (18 + 2 * s);
            match s {
                0..=2 => w10 |= (s as u32) << (8 + 8 * s),
                _ => w11 |= (s as u32) << (8 * (s - 3)),
            }
        }
        (w9, w10, w11)
    }
}
