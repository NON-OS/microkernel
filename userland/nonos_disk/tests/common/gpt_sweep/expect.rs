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

//! What each lie must come to, by kind, header, and whether the primary was
//! broken too, given the sound table. A lie that breaks one header leaves
//! the other's whole table; a lie that costs one entry costs that entry
//! alone; a holding backup that differs is said to disagree.

use super::lies::Lie;
use super::model::Part;
use super::rule::Named;

/// `got`, when it is `sound` short of exactly one entry (or of none when
/// `may_keep_all`); else a table no reader names, so the check fails.
fn short_one(got: &[Part], sound: &[Part], may_keep_all: bool) -> Vec<Part> {
    let lost = sound.len() - got.len().min(sound.len());
    let fits = got.iter().all(|p| sound.contains(p)) && (lost == 1 || may_keep_all && lost <= 1);
    if fits {
        got.to_vec()
    } else {
        vec![([0; 16], [0; 16], 0, 0)]
    }
}

pub fn expected(lie: Lie, w: usize, broke: bool, sound: &[Part], got: &Named) -> Named {
    use Lie::*;
    let (all, skips) = (sound.to_vec(), matches!(lie, PastLast | EndBeforeStart | BelowFirst));
    let kept = |may_keep_all| match got {
        Named::Primary(p) | Named::Backup(p) | Named::Disagreeing(p) => {
            short_one(p, sound, may_keep_all)
        }
        _ => Vec::new(),
    };
    match (lie, w, broke) {
        (NoMbrSignature | NoProtective | ProtectiveNotAtOne | Block4096 | BothBroken, ..) => {
            Named::Unbelieved
        }
        (Disagree, _, true) => Named::Backup(kept(true)),
        (_, 1, true) if skips => Named::Backup(kept(false)),
        (_, 1, true) => Named::Unbelieved,
        (BackupMyLba | BackupAlternate | BackupArrayOverHeader | BackupArrayInUsable, ..) => {
            Named::Primary(all)
        }
        (Disagree, ..) => Named::Disagreeing(all),
        (_, 0, _) if skips => Named::Disagreeing(kept(false)),
        (Overlap, 1, _) | (_, 1, _) if skips || lie == Overlap => Named::Disagreeing(all),
        (_, 0, _) => Named::Backup(all),
        _ => Named::Primary(all),
    }
}
