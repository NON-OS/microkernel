// NONOS Operating System (AGPL-3.0-or-later)

//! The frames the firmware sends on its own while the station sleeps, built
//! as mac80211 builds them for rtw88's reserved pages (`ieee80211_pspoll_get`,
//! `ieee80211_nullfunc_get`): to the access point, from the station.

/// PS-Poll: control frame, the AID with its two top bits set, the BSSID and
/// the station's address.
pub fn ps_poll(aid: u16, bssid: [u8; 6], sta: [u8; 6]) -> [u8; 16] {
    let mut f = [0u8; 16];
    f[0] = 0xA4;
    f[2..4].copy_from_slice(&(aid | 0xC000).to_le_bytes());
    f[4..10].copy_from_slice(&bssid);
    f[10..16].copy_from_slice(&sta);
    f
}

/// Null data to the distribution system: the firmware's way of telling the
/// access point the station sleeps or wakes.
pub fn null(bssid: [u8; 6], sta: [u8; 6]) -> [u8; 24] {
    let mut f = [0u8; 24];
    f[0..2].copy_from_slice(&[0x48, 0x01]);
    f[4..10].copy_from_slice(&bssid);
    f[10..16].copy_from_slice(&sta);
    f[16..22].copy_from_slice(&bssid);
    f
}

/// QoS Null, at TID 7 as mac80211 sends it.
pub fn qos_null(bssid: [u8; 6], sta: [u8; 6]) -> [u8; 26] {
    let mut f = [0u8; 26];
    f[..24].copy_from_slice(&null(bssid, sta));
    f[0] = 0xC8;
    f[24] = 7;
    f
}
