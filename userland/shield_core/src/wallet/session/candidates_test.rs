//! A restore reads only the candidate note logs that are on the disk, and replays each of
//! those as before: a log left by the same words is never taken for an empty one.

use super::Session;
use crate::custody::{phrase_to_seed, HardwareGuard, Phrase};
use crate::error::CustodyError;
use crate::store::{NoteLog, Row};
use crate::wallet::paths::Paths;

struct Open;

impl HardwareGuard for Open {
    fn wrap(&self, key: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        Ok(key)
    }

    fn unwrap_key(&self, blob: Vec<u8>) -> Result<Vec<u8>, CustodyError> {
        Ok(blob)
    }
}

fn words() -> Vec<String> {
    let mut words = vec![String::from("abandon"); 11];
    words.push(String::from("about"));
    words
}

fn folder(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("nox-candidates-test").join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch folder");
    dir
}

#[test]
fn a_restore_into_a_new_folder_writes_no_candidate_log_and_derives_every_candidate() {
    let dir = folder("new");
    let paths = Paths::under(&dir);
    let session = Session::restore(&paths, &Open, &words()).expect("the standard phrase");
    assert_eq!(session.candidates().len(), 31, "accounts 1 to 31 are candidates");
    for (i, c) in session.candidates().iter().enumerate() {
        let index = i as u32 + 1;
        let seed = phrase_to_seed(&Phrase::parse(&words()).expect("words")).expect("seed");
        let want = crate::evm::EvmAccount::at(seed.bytes(), index).expect("account").address();
        assert_eq!(c.evm().address(), want, "candidate {index}");
        assert_eq!(c.state().cursor(), 0);
        assert!(!paths.notes_of(index).exists(), "candidate {index} wrote nothing");
    }
}

#[test]
fn a_candidate_log_already_on_the_disk_is_replayed() {
    let dir = folder("held");
    let paths = Paths::under(&dir);
    let seed = phrase_to_seed(&Phrase::parse(&words()).expect("words")).expect("seed");
    let (mut log, _) = NoteLog::open_account(paths.notes_of(2), &seed, 2).expect("log 2");
    log.append(&Row::Cursor(42)).expect("a row");
    drop(log);
    let session = Session::restore(&paths, &Open, &words()).expect("the standard phrase");
    assert_eq!(session.candidates()[1].state().cursor(), 42, "account 2's own log is replayed");
    assert_eq!(session.candidates()[0].state().cursor(), 0, "account 1 has none");
}
