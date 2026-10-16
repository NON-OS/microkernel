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

//! The live and sealed tables as the kernel's other seal files reach them:
//! the prover's DELIVER check, open's reservation, end's drop and
//! MkSealRead's lookup by app.

use super::fixture::family;
use crate::foreign::seal::finish::{drop_family, finish};
use crate::foreign::seal::table::{supervises_live, with_family, LIVE, SEALED};

#[test]
fn a_supervisor_is_live_only_while_its_family_is() {
    let sup = 120;
    assert!(!supervises_live(sup));
    assert_eq!(with_family(sup, |f| f.sup), None, "no family, nothing run");
    let mut f = family(sup);
    f.app_id = Some([0x78; 32]);
    LIVE.lock().push(f);
    assert!(supervises_live(sup));
    assert_eq!(
        with_family(sup, |f| {
            f.enrolled = false;
            f.sup
        }),
        Some(sup)
    );
    assert_eq!(with_family(sup, |f| f.enrolled), Some(false), "the change stays");
    let n = finish(sup).expect("it seals");
    assert!(!supervises_live(sup), "sealed is no longer live");
    let sealed = SEALED.lock();
    let kept = sealed.iter().find(|s| s.seal_no == n).map(|s| s.app_id);
    assert_eq!(kept, Some([0x78; 32]), "kept under its app, for MkSealRead");
}

#[test]
fn a_dropped_family_keeps_nothing() {
    let sup = 121;
    let mut f = family(sup);
    f.app_id = Some([0x79; 32]);
    LIVE.lock().push(f);
    drop_family(sup);
    assert!(!supervises_live(sup));
    assert_eq!(finish(sup), None, "nothing left to seal");
    assert!(!SEALED.lock().iter().any(|s| s.app_id == [0x79; 32]));
}
