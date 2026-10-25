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

//! The files the image itself carries in the Linux tree, which no package
//! may replace (install/place/check.rs). They are the "linux" group of
//! tools/nix/store.json, make's own catalogue of the store, so the list
//! is the one the image was built from. Pure, so the host proofs read the
//! committed catalogue through it too.

use std::fs;
use std::path::Path;

/// Make's catalogue, from this crate's directory, where a build script runs.
const STORE: &str = "../../tools/nix/store.json";

/// The list the catalogue names, written to `to` as Rust.
pub fn write(to: &Path) -> Result<(), String> {
    println!("cargo:rerun-if-changed={STORE}");
    let text = fs::read_to_string(STORE).map_err(|e| format!("{STORE}: {e}"))?;
    let files = image_files(&text).map_err(|e| format!("{STORE}: {e}"))?;
    fs::write(to, source(&files)).map_err(|e| format!("{}: {e}", to.display()))
}

/// Each path in the "linux" group, without its `/linux`, sorted.
pub fn image_files(store: &str) -> Result<Vec<String>, String> {
    let mut keys = store.match_indices("\"linux\"").map(|(i, _)| i);
    let (Some(key), None) = (keys.next(), keys.next()) else {
        return Err("not one \"linux\" group".into());
    };
    let rest = store[key + "\"linux\"".len()..].trim_start();
    let rest = rest.strip_prefix(':').map(str::trim_start).ok_or("\"linux\" with no value")?;
    let group = rest.strip_prefix('[').ok_or("\"linux\" is not a list")?;
    let group = &group[..group.find(']').ok_or("the \"linux\" list never ends")?];
    let mut files = Vec::new();
    for (i, _) in group.match_indices("\"path\"") {
        let value = group[i + "\"path\"".len()..].trim_start();
        let value = value.strip_prefix(':').map(str::trim_start).unwrap_or("");
        let value = value.strip_prefix('"').ok_or("a path that is not a string")?;
        let path = &value[..value.find('"').ok_or("a path never ends")?];
        let file = path.strip_prefix("/linux/").ok_or_else(|| format!("{path}: not in /linux"))?;
        if !file.bytes().all(|b| b.is_ascii_graphic() && b != b'\\') {
            return Err(format!("{path}: not a plain name"));
        }
        files.push(format!("/{file}"));
    }
    if files.is_empty() {
        return Err("the \"linux\" group names no file".into());
    }
    files.sort();
    files.dedup();
    Ok(files)
}

/// The list as the Rust constant install/place/image.rs includes.
pub fn source(files: &[String]) -> String {
    let mut out = String::from("pub(in crate::linux::install) const IMAGE_FILES: &[&[u8]] = &[\n");
    for f in files {
        out.push_str(&format!("    b\"{f}\",\n"));
    }
    out.push_str("];\n");
    out
}
