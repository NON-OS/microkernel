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

use nonos_settle_proto::{approve_spender, kind_of};

#[test]
fn a_selector_belongs_to_its_kind_only() {
    let close = [&0x39c79e0cu32.to_be_bytes()[..], &[8; 32]].concat();
    assert_eq!(kind_of(15, &close).map(|k| k.id), Some(15));
    assert_eq!(kind_of(14, &close), None, "close is no topUp");
    assert_eq!(kind_of(15, &close[..35]), None, "a cut word");
    assert_eq!(kind_of(20, &close), None, "no kind 20");
    assert_eq!(kind_of(0, &close), None);
    let fund_from = [&0xaba72c43u32.to_be_bytes()[..], &[0; 480]].concat();
    assert_eq!(kind_of(9, &fund_from).map(|k| k.id), Some(9), "fundFrom is a Fund");
    let mut approve = [&0x095ea7b3u32.to_be_bytes()[..], &[0; 64]].concat();
    approve[16..36].copy_from_slice(&[5; 20]);
    assert_eq!(kind_of(1, &approve).map(|k| k.id), Some(1));
    assert_eq!(approve_spender(&approve), Some([5; 20]));
    approve[15] = 1;
    assert_eq!(approve_spender(&approve), None, "a dirty spender word");
}
