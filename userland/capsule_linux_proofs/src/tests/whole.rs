// NONOS Operating System (AGPL-3.0-or-later)
//! A file is taken whole or not at all: a read asks for one byte past the
//! most taken, and a file that fills it is refused, never cut. The keymap,
//! a package's manifest and a program's recorded path are read this way.

use crate::linux::whole::{ask, whole};

#[test]
fn a_file_up_to_the_most_is_taken_and_one_byte_more_is_refused() {
    for max in [256u32, 1 << 20] {
        assert_eq!(ask(max), max + 1, "the read asks one byte past");
        let at_most = vec![7u8; max as usize];
        assert_eq!(whole(at_most.clone(), max), Some(at_most), "MAX taken");
        assert_eq!(whole(vec![7u8; max as usize + 1], max), None, "MAX + 1 refused");
        assert_eq!(whole(Vec::new(), max), Some(Vec::new()), "empty is whole");
    }
    assert_eq!(ask(u32::MAX), u32::MAX, "never wraps to a smaller read");
}

const STORED: &str = include_str!("../../../capsule_linux/src/linux/file/held/cache/stored.rs");
const TAKE: &str = include_str!("../../../capsule_linux/src/linux/file/held/cache/take.rs");

#[test]
fn a_file_too_long_to_hold_is_refused_never_held_cut_short() {
    // The first write to a stored file holds it, and a flush puts the copy
    // over the whole file: a copy cut at MAX_FILE would lose the file's tail.
    assert!(
        STORED.contains("store::read(key, ask(MAX_FILE as u32))"),
        "the read asks one byte past"
    );
    assert!(
        STORED.contains("whole(data, MAX_FILE as u32).ok_or(errno::EFBIG)"),
        "a full read is refused"
    );
    assert!(
        TAKE.contains("true => stored(&key)?,"),
        "hold takes a stored file only through stored.rs"
    );
    assert!(!TAKE.contains("store::read"), "hold reads the store some other way");
}
