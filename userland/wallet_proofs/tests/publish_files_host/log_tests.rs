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

//! read_log at FILE_LEN: a log of it read whole, one byte more refused as damaged.

use crate::attest::{encode, Attest, Record, FILE_LEN};
use crate::clients::vfs;
use crate::publish::files::{read_log, write_log, write_sig};

const LOG: &[u8] = b"/data/wallet.attests";

fn record() -> Record {
    let attest = Attest {
        chain_id: 1,
        listing_id: [1; 32],
        release_id: [2; 32],
        bond_id: [3; 32],
        chunk_root: [4; 32],
        content_len: 4096,
        package_hash: [5; 32],
    };
    Record { attest, sig: [6; 65] }
}

#[test]
fn read_log_reads_a_log_of_file_len_whole() {
    assert_eq!(write_log(7, &[record()]), Ok(()));
    assert_eq!(vfs::get(LOG).map(|b| b.len()), Some(FILE_LEN));
    assert_eq!(read_log(7), Ok(vec![record()]));
}

#[test]
fn read_log_refuses_a_log_one_byte_past_file_len() {
    let mut log = encode(&[record()]).unwrap_or_default();
    log.push(0);
    vfs::put(LOG, log);
    let damaged = "the attest log is damaged; nothing is signed until it is restored";
    assert_eq!(read_log(7), Err(damaged));
}

#[test]
fn write_sig_keeps_the_signature_beside_the_document() {
    assert_eq!(write_sig(7, "/data/listing.entry.sig", &[9; 65]), Ok(()));
    assert_eq!(vfs::get(b"/data/listing.entry.sig"), Some(vec![9; 65]));
    assert_eq!(vfs::persisted(), vec![b"/data/listing.entry.sig".to_vec()]);
}
