// NONOS Operating System (AGPL-3.0-or-later)
//! A sidecar file is taken whole or refused: the read asks one byte past
//! BLOB_MAX, and a file that fills it is refused, so a cut record is never
//! loaded and saved back cut.

use super::whole::{ask, whole, TOO_LONG};

const BLOB_MAX: u32 = 256 * 1024; // capsule_file_manager fm/store_meta.rs

#[test]
fn a_sidecar_up_to_blob_max_is_taken_and_one_byte_more_is_refused() {
    assert_eq!(ask(BLOB_MAX), BLOB_MAX + 1);
    let at_most = vec![b't'; BLOB_MAX as usize];
    assert_eq!(whole(Ok(at_most.clone()), BLOB_MAX), Ok(at_most), "BLOB_MAX taken");
    let over = vec![b't'; BLOB_MAX as usize + 1];
    assert_eq!(whole(Ok(over), BLOB_MAX), Err(TOO_LONG), "BLOB_MAX + 1 refused");
    assert_eq!(
        whole(Err("vfs ipc failed"), BLOB_MAX),
        Err("vfs ipc failed"),
        "a failed read stays"
    );
}
