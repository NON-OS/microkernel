//! Where the wallet keeps its files, all in the app private storage the shell passes: not a
//! cache, which can be evicted with the notes in it, and not shared storage another app reads.

use std::path::PathBuf;

/// The files of one wallet, in app private storage excluded from cloud backup.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Paths {
    pub vault: PathBuf,
    pub notes: PathBuf,
    pub measurement: PathBuf,
    pub tor: PathBuf,
    /// The account book and the nonce records of each public account.
    pub account: PathBuf,
    pub watching: PathBuf,
}

impl Paths {
    pub fn under(dir: impl Into<PathBuf>) -> Paths {
        let dir = dir.into();
        Paths {
            vault: dir.join("vault.seal"),
            notes: dir.join("notes.log"),
            measurement: dir.join("measurement.txt"),
            tor: dir.join("tor"),
            account: dir.join("account"),
            watching: dir.join("watching.seal"),
        }
    }

    /// The note log of account `index`. Account 0 keeps the file every wallet has today.
    pub fn notes_of(&self, index: u32) -> PathBuf {
        if index == 0 {
            return self.notes.clone();
        }
        self.notes.with_file_name(format!("notes-{index}.log"))
    }
}
