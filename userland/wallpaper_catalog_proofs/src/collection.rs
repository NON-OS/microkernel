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

//! The store's wallpaper collection as the catalog sees it through vfs: the
//! files nonos-data/wallpapers/catalog.txt names, in its order, end to end,
//! as tools/nonos-wallpaper-pack packs them. A proof can change a byte of it
//! and see what the catalog does.

use std::sync::Mutex;

const COLLECTION: &[u8] = b"/Wallpapers/collection";

static BYTES: Mutex<Option<Vec<u8>>> = Mutex::new(None);

/// Held by a proof for as long as it uses the collection, since proofs run
/// on threads at once and one changes bytes another reads.
pub static TURN: Mutex<()> = Mutex::new(());

/// Wallpaper `index`'s own bytes, from its file, or `None` past the last.
pub fn wallpaper(index: u32) -> Option<Vec<u8>> {
    files().get(index as usize).map(|(_, b)| b.clone())
}

/// The wallpapers as (slug, bytes), in catalog order, read from the tree
/// once.
pub fn files() -> &'static [(String, Vec<u8>)] {
    static FILES: std::sync::OnceLock<Vec<(String, Vec<u8>)>> = std::sync::OnceLock::new();
    FILES.get_or_init(read_files)
}

fn read_files() -> Vec<(String, Vec<u8>)> {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../nonos-data/wallpapers");
    let list = std::fs::read_to_string(format!("{dir}/catalog.txt")).unwrap();
    list.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(|l| {
            let (slug, file) = l.split_once(' ').unwrap();
            (slug.to_string(), std::fs::read(format!("{dir}/{file}")).unwrap())
        })
        .collect()
}

/// Whether the disk carries the whole collection; else only `ALONE`.
static WHOLE: Mutex<bool> = Mutex::new(true);

/// Kept wallpapers carried each alone, by store path.
static ALONE: Mutex<std::collections::BTreeMap<Vec<u8>, Vec<u8>>> = Mutex::new(std::collections::BTreeMap::new());

/// A disk as an install that kept only `kept` writes it: no collection, and
/// each of those wallpapers as its own file.
pub fn installed(kept: &[usize]) {
    *WHOLE.lock().unwrap() = false;
    let mut alone = ALONE.lock().unwrap();
    alone.clear();
    for &i in kept {
        let (slug, bytes) = &files()[i];
        alone.insert(format!("/Wallpapers/{slug}.jpg").into_bytes(), bytes.clone());
    }
}

/// Put the collection back as the files make it.
pub fn reset() {
    *WHOLE.lock().unwrap() = true;
    ALONE.lock().unwrap().clear();
    *BYTES.lock().unwrap() = Some(files().iter().flat_map(|(_, b)| b.iter().copied()).collect());
}

/// Change the collection's byte at `at`.
pub fn flip(at: usize) {
    let mut bytes = BYTES.lock().unwrap();
    bytes.as_mut().expect("reset first")[at] ^= 0x40;
}

pub struct VfsStream {
    alone: Option<Vec<u8>>,
}

impl VfsStream {
    pub fn open(owner_pid: u32, path: &[u8]) -> Result<VfsStream, &'static str> {
        assert_eq!(owner_pid, nonos_libc::SERVICE_PID, "the catalog opens as itself");
        if path != COLLECTION && !path.starts_with(b"/Wallpapers/") {
            return Err("vfs open failed");
        }
        if path != COLLECTION || !*WHOLE.lock().unwrap() {
            // A disk that carries each kept wallpaper alone, or none.
            let alone = ALONE.lock().unwrap().get(path).cloned();
            return alone.map(|b| VfsStream { alone: Some(b) }).ok_or("vfs open failed");
        }
        Ok(VfsStream { alone: None })
    }

    pub fn read_window(&mut self, offset: u64, len: u32) -> Result<Vec<u8>, &'static str> {
        if let Some(b) = &self.alone {
            let start = (offset as usize).min(b.len());
            return Ok(b[start..(start + len as usize).min(b.len())].to_vec());
        }
        let bytes = BYTES.lock().unwrap();
        let all = bytes.as_ref().ok_or("vfs read failed")?;
        let start = (offset as usize).min(all.len());
        let end = (start + len as usize).min(all.len());
        Ok(all[start..end].to_vec())
    }
}
