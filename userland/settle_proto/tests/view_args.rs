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

use nonos_settle_proto::{view, view_calldata};

#[test]
fn view_calldata_holds_every_argument_to_its_form() {
    let v = view(60).unwrap();
    let (mut asset, mut owner) = ([0u8; 32], [0u8; 32]);
    asset[31] = 2;
    owner[12..].copy_from_slice(&[9; 20]);
    let call = view_calldata(v, &[asset, owner]).unwrap();
    assert_eq!((&call[..4], call.len()), (&[0x88, 0xd8, 0xb2, 0xa7][..], 68));
    assert_eq!(view_calldata(v, &[asset]), None, "a word short");
    assert_eq!(view_calldata(v, &[owner, owner]), None, "an address is no uint64");
    let mut wide = owner;
    wide[11] = 1;
    assert_eq!(view_calldata(view(1).unwrap(), &[wide]), None, "a dirty address");
    let mut byte = [0u8; 32];
    byte[30] = 1;
    assert_eq!(view_calldata(view(50).unwrap(), &[[7; 32], byte, [7; 32]]), None, "kind > 255");
    assert_eq!(view_calldata(view(18).unwrap(), &[]).unwrap(), vec![0x84, 0xb0, 0x19, 0x6e]);
}
