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
//! A whole receipt verifies, and says what it carries.

use agentproof_receipt::verify;

use crate::fixture::Fixture;

#[test]
fn a_whole_receipt_verifies() {
    let mut f = Fixture::new(b"strict");
    let (bytes, trust) = (f.bytes(), f.trust());
    let v = verify(&bytes, &trust).unwrap();
    assert_eq!((v.release.as_str(), v.box_word.as_str()), ("0.9.3-test", "strict"));
    assert!(v.enforced && !v.built_in);
    assert_eq!((v.program, v.supervisor, v.seal_no), (Some([7; 32]), [5; 32], 9));
}
