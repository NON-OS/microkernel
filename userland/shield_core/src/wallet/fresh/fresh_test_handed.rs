//! The handed-out list: kept once each, found by address (v1.4.51).

use super::super::*;
use super::sign::address_at;

#[test]
fn only_an_address_kept_as_handed_out_has_an_index() {
    let dir = std::env::temp_dir().join(format!("nox-fresh-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(handed_index(&dir, &address_at(3)), None);
    keep_handed(&dir, 3, &address_at(3)).unwrap();
    keep_handed(&dir, 3, &address_at(3)).unwrap();
    keep_handed(&dir, 4, &address_at(4)).unwrap();
    assert_eq!(handed_index(&dir, &address_at(3)), Some(3));
    assert_eq!(handed_index(&dir, &address_at(4)), Some(4));
    assert_eq!(handed_index(&dir, &address_at(5)), None);
    let text = std::fs::read_to_string(dir.join("fresh")).unwrap();
    assert_eq!(text.lines().count(), 2, "kept once each");
    let _ = std::fs::remove_dir_all(&dir);
}
