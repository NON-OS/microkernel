//! Opening, adding and forgetting view-only accounts. Each log is sealed under a key derived from
//! the watching key and the view key, and named by a keyed hash, so a file name shows nothing.

use super::file;
use super::{Watched, Watching, WATCH_CONTEXT};
use crate::custody::Seed;
use crate::error::WalletError;
use crate::keys::derive::byte_key;
use crate::keys::view_key_read::parse_view_key;
use crate::store::NoteLog;
use crate::wallet::paths::Paths;
use std::path::PathBuf;
use zeroize::Zeroizing;

impl Watching {
    pub(crate) fn open(paths: &Paths, seed: &Seed) -> Result<Watching, WalletError> {
        let root = Zeroizing::new(byte_key(seed.bytes(), WATCH_CONTEXT));
        let mut watching = Watching {
            root,
            file: paths.watching.clone(),
            notes: paths.notes.clone(),
            list: vec![],
        };
        for text in file::read(&watching.file, &watching.root)? {
            let entry = watching.entry(text)?;
            watching.list.push(entry);
        }
        Ok(watching)
    }

    pub(super) fn entry(&self, text: Zeroizing<String>) -> Result<Watched, WalletError> {
        let key = parse_view_key(&text)?;
        let store =
            blake3::derive_key("nox-shield 2026 watched store key v1", &self.material(&text));
        let (log, state) = NoteLog::open_keyed(self.log_path(&text), store)?;
        Ok(Watched { key, text, log, state })
    }

    fn material(&self, text: &str) -> Zeroizing<Vec<u8>> {
        let mut m = Zeroizing::new(self.root.to_vec());
        m.extend_from_slice(text.as_bytes());
        m
    }

    pub(super) fn log_path(&self, text: &str) -> PathBuf {
        let name = blake3::derive_key("nox-shield 2026 watched log name v1", &self.material(text));
        let hex: String = name.iter().take(8).map(|b| format!("{b:02x}")).collect();
        self.notes.with_file_name(format!("notes-v{hex}.log"))
    }

    pub(super) fn save(&self) -> Result<(), WalletError> {
        let texts: Vec<&str> = self.list.iter().map(|w| w.text.as_str()).collect();
        Ok(file::write(&self.file, &self.root, &texts)?)
    }
}
