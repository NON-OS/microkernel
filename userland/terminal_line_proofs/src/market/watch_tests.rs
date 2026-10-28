// NONOS Operating System (AGPL-3.0-or-later)
//! An install the Terminal queued is asked about once a second, said once
//! when it ends, however it ends, and the list never grows past its bound.

use super::watch::{Watch, EVERY_MS, MAX};

/// A stage as the kernel answers: 1 queued, 2 installing, 3 installed, and
/// 16 plus the code of a failure (150 is an installer that panicked).
fn done(code: i64) -> bool {
    code == 3 || code == 4 || code >= 16
}

#[test]
fn a_panicked_installer_is_said_once_with_its_code() {
    let mut w: Watch<bool> = Watch::new();
    w.add(b"linux.monero", false);
    assert!(w.due(|| 0, |_| 2i64, done).is_empty(), "still installing");
    assert!(w.due(|| EVERY_MS - 1, |_| 150, done).is_empty(), "not asked again within the second");
    let ended = w.due(|| EVERY_MS, |_| 150, done);
    assert_eq!(ended, vec![(b"linux.monero".to_vec(), false, 150)]);
    assert!(w.due(|| 10 * EVERY_MS, |_| 150, done).is_empty(), "said once, never again");
}

#[test]
fn each_listing_ends_on_its_own() {
    let mut w: Watch<bool> = Watch::new();
    w.add(b"linux.a", false);
    w.add(b"linux.b", true);
    let ended = w.due(|| 0, |id| if id == b"linux.b" { 16 } else { 1 }, done);
    assert_eq!(ended, vec![(b"linux.b".to_vec(), true, 16)]);
    let ended = w.due(|| EVERY_MS, |_| 3, done);
    assert_eq!(ended, vec![(b"linux.a".to_vec(), false, 3)]);
}

#[test]
fn nothing_followed_asks_nothing() {
    let mut w: Watch<bool> = Watch::new();
    let mut asked = 0;
    let ended = w.due(
        || 0,
        |_| {
            asked += 1;
            3
        },
        done,
    );
    assert!(ended.is_empty());
    assert_eq!(asked, 0);
}

#[test]
fn the_list_is_bounded_and_asking_again_does_not_double_it() {
    let mut w: Watch<u8> = Watch::new();
    for i in 0..(MAX as u8 + 3) {
        w.add(&[b'a' + i], i);
    }
    w.add(&[b'a' + 5], 99);
    let ended = w.due(|| 0, |_| 3, done);
    assert_eq!(ended.len(), MAX);
    assert_eq!(ended[0].0, vec![b'a' + 3], "the first asked for gave way");
    assert_eq!(ended.last().unwrap(), &(vec![b'a' + 5], 99, 3), "asked again, followed anew");
}
