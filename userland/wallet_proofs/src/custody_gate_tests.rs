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

/*
 * A source gate for the wallet's secrets (#131, carried into Custody): each
 * sits in a Zeroizing buffer; neither State nor Custody can be copied or
 * printed; a new wallet's words are never drawn while locked, a lock keeps
 * them, and unlocking goes straight back to the backup screen.
 */

fn wallet(path: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(root.join("../capsule_wallet_nonos/src/wallet").join(path)).unwrap()
}

#[test]
fn every_secret_is_zeroizing_and_nothing_copies_or_prints_it() {
    let custody = wallet("state/custody.rs");
    for field in ["import_hex", "recover_buf", "export_hex", "backup_words"] {
        let decl = custody.lines().find(|l| l.contains(&format!("pub {field}:"))).unwrap();
        assert!(decl.contains("Zeroizing<"), "{decl}");
    }
    for file in ["state/custody.rs", "state/types.rs"] {
        assert!(!wallet(file).contains("derive("), "{file} derives nothing");
        assert!(!wallet(file).contains("impl Debug"), "{file} prints nothing");
    }
    assert!(wallet("state/types.rs").contains("pub custody: super::custody::Custody,"));
}

#[test]
fn the_words_are_never_drawn_while_locked_and_unlock_returns_to_them() {
    let paint = wallet("paint/paint.rs");
    let locked = paint.find("if state.locked").unwrap();
    let backup = paint.find("if state.keys.backup_active").unwrap();
    assert!(locked < backup, "the locked screen is drawn first");
    let screen = wallet("screen/locked.rs");
    assert!(!screen.contains("custody") && !screen.contains("backup"));
    /* Neither lock nor unlock ends the backup: it stays for the person. */
    let lock = ["event/locking/lock.rs", "event/locking/unlock.rs"].map(wallet).concat();
    assert!(!lock.contains("backup_active") && !lock.contains("wipe_backup"));
}
