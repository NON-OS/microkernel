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

//! A save cut at every write it makes: the next start finds the old
//! document or the new one, whole, and no object nothing needs.

use nonos_user_files::limits::DEFAULT_QUOTA;
use nonos_user_files::wire::flags::{CREATE, TRUNC, WRITE};
use nonos_user_files::Broker;

use super::live::no_strays;
use super::{fresh, get_file, pattern, put_file, settle, APP};
use crate::doubles::{client, Desk, Volume};

const DOC: &[u8] = b"Documents/report.odt";
const TMP: &[u8] = b"Documents/report.odt.tmp";

/// A program's safe save: a temporary name, written, synced, renamed over.
/// Stops at the first refusal, as a program whose disk failed would.
fn save(b: &mut Broker<Volume>, desk: &Desk, body: &[u8], sync: bool) -> Result<(), i32> {
    let mut c = client(b, desk, APP);
    let (h, _) = c.open(TMP, WRITE | CREATE | TRUNC, 0o644)?;
    for (i, piece) in body.chunks(200_000).enumerate() {
        c.write(h, (i * 200_000) as u64, piece)?;
    }
    if sync {
        c.fsync(h)?;
    }
    c.close(h)?;
    c.rename(TMP, DOC, 0)
}

fn cut_everywhere(old: &[u8], new: &[u8], sync: bool, torn: bool) {
    let (mut b, desk) = fresh();
    put_file(&mut b, &desk, DOC, old);
    settle(&mut b, &desk);
    let before = b.objects().reboot();
    let mut whole = Broker::new(before.reboot(), DEFAULT_QUOTA);
    save(&mut whole, &desk, new, sync).expect("an uncut save");
    let writes = whole.objects().spent;
    assert!(writes > 2, "a save that wrote nothing");
    let (mut olds, mut news) = (0, 0);
    for cut in 0..=writes {
        let mut v = before.reboot();
        v.cut = Some(cut);
        v.torn = torn.then_some(cut % 977 + 1);
        let mut b = Broker::new(v, DEFAULT_QUOTA);
        let _ = save(&mut b, &desk, new, sync);
        let mut next = Broker::new(b.objects().reboot(), DEFAULT_QUOTA);
        let found = get_file(&mut next, &desk, DOC).expect("the document is there after the cut");
        match found {
            f if f == old => olds += 1,
            f if f == new => news += 1,
            f => panic!("cut {cut}: a document of {} bytes, neither old nor new", f.len()),
        }
        no_strays(next.objects());
    }
    assert!(olds > 0 && news > 0, "every cut gave the same document: {olds} old, {news} new");
}

#[test]
fn a_synced_save_is_old_or_new_at_every_cut() {
    cut_everywhere(&pattern(1, 2_600_000), &pattern(2, 1_300_000), true, false);
}

#[test]
fn an_unsynced_save_is_old_or_new_at_every_cut() {
    cut_everywhere(&pattern(3, 900_000), &pattern(4, 3_100_000), false, false);
}

#[test]
fn a_write_torn_by_the_cut_never_shows() {
    cut_everywhere(&pattern(5, 1_048_576), &pattern(6, 2_097_153), true, true);
}

#[test]
fn a_save_after_a_cut_lands_and_leaves_nothing_behind() {
    let (mut b, desk) = fresh();
    put_file(&mut b, &desk, DOC, &pattern(7, 3_000_000));
    settle(&mut b, &desk);
    let mut v = b.objects().reboot();
    v.cut = Some(4);
    let mut cut = Broker::new(v, DEFAULT_QUOTA);
    assert!(save(&mut cut, &desk, &pattern(8, 2_000_000), true).is_err());
    let mut next = Broker::new(cut.objects().reboot(), DEFAULT_QUOTA);
    save(&mut next, &desk, &pattern(9, 500_000), true).unwrap();
    assert_eq!(get_file(&mut next, &desk, DOC).unwrap(), pattern(9, 500_000));
    assert_eq!(get_file(&mut next, &desk, TMP), Err(nonos_user_files::errno::ENOENT));
    no_strays(next.objects());
}
