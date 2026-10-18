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

/*
 * The publisher screen's files on the data volume: the entry document the
 * person names, the ReleaseAttest log, and each signature beside the
 * document, persisted so it survives the boot.
 */

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{persist, read_file, stat, write_file};
use nonos_marketplace_abi::{decode_entry_document, MarketplaceEntry};

use super::super::attest::{decode, encode, Record, FILE_LEN};

/// The largest entry document read: an entry with its releases is a few KiB.
const DOC_MAX: u32 = 256 * 1024;
const LOG_PATH: &[u8] = b"/data/wallet.attests";

pub fn read_doc(pid: u32, path: &str) -> Result<MarketplaceEntry, &'static str> {
    // One byte past the cap, so a longer file is refused, never decoded cut.
    let bytes =
        read_file(pid, path.as_bytes(), DOC_MAX + 1).map_err(|_| "that file could not be read")?;
    if bytes.len() > DOC_MAX as usize {
        return Err("that file is not one listing's entry document");
    }
    decode_entry_document(&bytes).map_err(|_| "that file is not one listing's entry document")
}

/// The log; a missing file is an empty log, an unreadable one an error,
/// since signing on a guess could sign a second, different attest.
pub fn read_log(pid: u32) -> Result<Vec<Record>, &'static str> {
    if stat(pid, LOG_PATH).is_err() {
        return Ok(Vec::new());
    }
    // One byte past FILE_LEN, so a longer log fails decode's length check.
    let bytes = read_file(pid, LOG_PATH, FILE_LEN as u32 + 1)
        .map_err(|_| "the attest log could not be read")?;
    decode(&bytes).ok_or("the attest log is damaged; nothing is signed until it is restored")
}

pub fn write_log(pid: u32, log: &[Record]) -> Result<(), &'static str> {
    let bytes = encode(log).ok_or("the attest log is full")?;
    keep(pid, LOG_PATH, &bytes, "the attest log could not be saved")
}

pub fn write_sig(pid: u32, path: &str, sig: &[u8; 65]) -> Result<(), &'static str> {
    keep(pid, path.as_bytes(), sig, "the signature could not be saved beside the document")
}

fn keep(pid: u32, path: &[u8], bytes: &[u8], why: &'static str) -> Result<(), &'static str> {
    write_file(pid, path, bytes).and_then(|_| persist(pid, path)).map_err(|_| why)
}
