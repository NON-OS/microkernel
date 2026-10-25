// NONOS Operating System (AGPL-3.0-or-later)

//! Only the access point's own EAPOL to this station drives the handshake.

use super::beacons::{AP, OUR};
use crate::assoc::eapol::eapol_from_bss;

#[test]
fn only_the_access_points_own_eapol_to_this_station_drives_the_handshake() {
    let mut f = vec![0x08, 0x02, 0, 0];
    f.extend_from_slice(&OUR);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&[0, 0]);
    assert!(eapol_from_bss(&f, &OUR, &AP), "FromDS, from the BSSID, to us, in the clear");
    let mut to_ds = f.clone();
    to_ds[1] = 0x01;
    assert!(!eapol_from_bss(&to_ds, &OUR, &AP), "ToDS: another station's");
    let mut other = f.clone();
    other[4] ^= 1;
    assert!(!eapol_from_bss(&other, &OUR, &AP), "to another station");
    let mut bss = f.clone();
    bss[10] ^= 1;
    assert!(!eapol_from_bss(&bss, &OUR, &AP), "from another BSS");
    let mut prot = f.clone();
    prot[1] |= 0x40;
    assert!(!eapol_from_bss(&prot, &OUR, &AP), "protected: not the handshake in the clear");
    assert!(!eapol_from_bss(&f[..20], &OUR, &AP), "a runt");
}
