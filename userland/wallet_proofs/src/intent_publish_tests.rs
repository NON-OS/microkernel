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

use nonos_nox_config::Contract;
use sha3::{Digest, Keccak256};

use crate::intent::{encode, Publisher};

fn n(v: u128) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[16..].copy_from_slice(&v.to_be_bytes());
    w
}

fn built(p: Publisher) -> Vec<u8> {
    encode(&p, |c: Contract| Some([c as u8; 20])).unwrap()
}

#[test]
fn publish_carries_its_uri_as_an_abi_string() {
    let uri = b"https://example.org/releases/linux.qwen-small/qwen-small@1.bin".to_vec();
    let p = Publisher::Publish {
        publisher_key: [1; 32],
        app_id_hash: [4; 32],
        release_id: [5; 32],
        release_hash: [6; 32],
        bond_id: [2; 32],
        uri: uri.clone(),
    };
    let mut want =
        Keccak256::digest(b"publish(bytes32,bytes32,bytes32,bytes32,bytes32,string)")[..4].to_vec();
    for w in [[1; 32], [4; 32], [5; 32], [6; 32], [2; 32], n(6 * 32), n(uri.len() as u128)] {
        want.extend_from_slice(&w);
    }
    want.extend_from_slice(&uri);
    want.resize(4 + (want.len() - 4).div_ceil(32) * 32, 0);
    assert_eq!(want.len(), 4 + 7 * 32 + 64);
    assert_eq!(built(p), want);
}
