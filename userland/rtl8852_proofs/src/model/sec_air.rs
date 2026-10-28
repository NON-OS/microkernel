// NONOS Operating System (AGPL-3.0-or-later)

//! CCMP-128 in the modelled chip, under its keys (`sec`). A frame sent with
//! `SEC_HW_ENC` is sealed under its WD's security CAM entry at the packet
//! number its CCMP header holds; one heard Protected is opened under the
//! address CAM slot whose index it names, unicast slots 0 and 1, group
//! slots 2 to 4, and comes up with `HW_DEC`, its header and MIC left (rtw89
//! strips neither); a failed MIC sets `ICV_ERR` over the plaintext all the
//! same. With no slot to open it, it is dropped: what the driver must not
//! depend on (`Faults::pass_unkeyed` passes it up as it came).

use nonos_wifi_core::dot11::ccmp::{decrypt, encrypt, pn_and_key};

use super::state::State;

/// The frame a WD's info word 2 asks the chip to seal, or `None`.
pub fn seal(s: &State, info2: u32, f: &[u8]) -> Option<Vec<u8>> {
    let (kind, key) = s.sec.entries.get(&(info2 as u8)).copied()?;
    let (pn, _) = pn_and_key(&[f, &[0; 8]].concat(), 24)?;
    let plain = [&f[..24], &f[32..]].concat();
    (kind == 6 && info2 >> 9 & 0xf == 6).then(|| encrypt(&plain, pn, &key)).flatten()
}

/// What the chip makes of `f` heard: its RX descriptor word 3 and frame.
pub fn open(s: &State, f: &[u8]) -> Option<(u32, Vec<u8>)> {
    if f.len() < 40 || f[1] & 0x40 == 0 {
        return Some((0, f.to_vec()));
    }
    let (_, id) = pn_and_key(f, 24)?;
    let group = f[4] & 1 != 0;
    let key = s.sec.key(if group { 2..5 } else { 0..2 }, id);
    let Some(key) = key else { return s.faults.pass_unkeyed.then(|| (0, f.to_vec())) };
    let (body, icv) = decrypt(f, &key).map_or_else(|| (unchecked(f, &key), 1 << 10), |b| (b, 0));
    let mut out = f[..32].to_vec();
    out.extend_from_slice(&body);
    out.extend_from_slice(&f[f.len() - 8..]);
    Some((1 << 2 | icv, out))
}

/// The plaintext with the MIC unchecked: the keystream sealing zeros gives.
fn unchecked(f: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let (pn, _) = pn_and_key(f, 24).unwrap();
    let ct = &f[32..f.len() - 8];
    let ks = encrypt(&[&f[..24], &vec![0; ct.len()][..]].concat(), pn, key).unwrap();
    ct.iter().zip(&ks[32..]).map(|(c, k)| c ^ k).collect()
}
