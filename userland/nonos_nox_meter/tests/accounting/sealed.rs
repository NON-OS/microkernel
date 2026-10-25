// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::fixture::funded_state;
use super::host::{keccak, Host};
use nonos_nox_meter::persist::{mac_key, open, seal, FILE_LEN, STATE_LEN};
use nonos_nox_meter::MeterError;

#[test]
fn a_sealed_file_opens_only_under_its_key_and_unchanged() {
    let key = mac_key(&Host, &[7; 32]);
    assert_ne!(key, mac_key(&Host, &[8; 32]));
    let mut pre = b"NONOS.nox.meter.mac.v1".to_vec();
    pre.extend_from_slice(&[7; 32]);
    assert_eq!(key, keccak(&pre));
    let s = funded_state();
    let file = seal(&Host, &key, &s).unwrap();
    assert_eq!(file.len(), FILE_LEN);
    assert_eq!(open(&Host, &key, &file), Ok(s.clone()));
    assert_eq!(open(&Host, &mac_key(&Host, &[8; 32]), &file), Err(MeterError::Corrupt));
    for at in [0, STATE_LEN / 2, STATE_LEN - 1, STATE_LEN, FILE_LEN - 1] {
        let mut bad = file.clone();
        bad[at] ^= 1;
        assert_eq!(open(&Host, &key, &bad), Err(MeterError::Corrupt), "byte {at}");
    }
    assert_eq!(open(&Host, &key, &file[..STATE_LEN]), Err(MeterError::Corrupt));
}

#[test]
fn the_tag_is_keccak_over_its_domain_key_and_body() {
    let key = [3u8; 32];
    let file = seal(&Host, &key, &funded_state()).unwrap();
    let mut pre = b"NONOS.nox.meter.state.v1".to_vec();
    pre.extend_from_slice(&key);
    pre.extend_from_slice(&file[..STATE_LEN]);
    assert_eq!(&file[STATE_LEN..], &keccak(&pre));
}
