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
use nonos_settle_proto::MAX_URL;

use crate::intent::{encode, Publisher};

const AT: [u8; 20] = [0xAA; 20];

#[test]
fn refused_shapes() {
    use Publisher::*;
    let at = |_| Some(AT);
    for spender in
        [Contract::NoxToken, Contract::ProofAnchor, Contract::ShieldPool, Contract::ListingRegistry]
    {
        assert_eq!(encode(&Approve { spender, amount: 1 }, at), None, "{spender:?}");
    }
    assert_eq!(encode(&Approve { spender: Contract::Escrow, amount: 0 }, at), None);
    assert_eq!(encode(&Bond { publisher_key: [1; 32], amount: 0 }, at), None);
    let mut long = b"https://h/".to_vec();
    long.resize(MAX_URL + 1, b'a');
    for uri in [b"http://h/r".to_vec(), b"".to_vec(), long] {
        let p = Publish {
            publisher_key: [1; 32],
            app_id_hash: [4; 32],
            release_id: [5; 32],
            release_hash: [6; 32],
            bond_id: [2; 32],
            uri,
        };
        assert_eq!(encode(&p, at), None);
    }
}
