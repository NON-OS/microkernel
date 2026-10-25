// NONOS Operating System (AGPL-3.0-or-later)

//! The MAC and link contexts added at bring-up under the boot's address.

/// `FW_CTXT_ACTION_ADD`, `FW_MAC_TYPE_BSS_STA`, `FW_CTXT_INVALID`.
const ACTION_ADD: u32 = 1;
const MAC_TYPE_BSS_STA: u32 = 5;
const CTXT_INVALID: u32 = 0xFFFF_FFFF;

/// `iwl_mac_config_cmd` adding station MAC 0 at `addr`, unassociated: accept
/// group frames and beacons; HE off until association; the NIC does not
/// advertise ACK-enabled aggregation (its HE capabilities do not set it).
pub fn mac_config_add(addr: [u8; 6]) -> [u8; 52] {
    const ACCEPT_GRP: u32 = 1 << 2;
    const ACCEPT_BEACON: u32 = 1 << 3;
    let mut c = [0u8; 52];
    c[4..8].copy_from_slice(&ACTION_ADD.to_le_bytes());
    c[8..12].copy_from_slice(&MAC_TYPE_BSS_STA.to_le_bytes());
    c[12..18].copy_from_slice(&addr);
    c[20..24].copy_from_slice(&(ACCEPT_GRP | ACCEPT_BEACON).to_le_bytes());
    c[32..36].copy_from_slice(&1u32.to_le_bytes());
    c
}

/// `iwl_link_config_cmd` adding link 0 of MAC 0 with no PHY context yet.
pub fn link_config_add(addr: [u8; 6]) -> alloc::vec::Vec<u8> {
    let mut c = alloc::vec![0u8; 208];
    c[0..4].copy_from_slice(&ACTION_ADD.to_le_bytes());
    c[12..16].copy_from_slice(&CTXT_INVALID.to_le_bytes());
    c[16..22].copy_from_slice(&addr);
    c
}
