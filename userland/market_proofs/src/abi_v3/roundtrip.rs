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

use nonos_marketplace_abi::{SettleMode, SCHEMA_V3};

use super::blobs::{blob, decode};
use super::fixtures::sold;

#[test]
fn every_schema_3_field_reads_back_as_written() {
    let written = sold();
    let read = decode(&blob(written.clone())).expect("decodes");
    assert_eq!(read.schema_version, SCHEMA_V3);
    let (w, r) = (&written.entries[0], &read.entries[0]);
    assert_eq!(r.commerce, w.commerce);
    assert_eq!(r.commerce.as_ref().map(|c| c.settle_mode), Some(SettleMode::Escrow));
    for (wr, rr) in w.releases.iter().zip(&r.releases) {
        assert_eq!(rr.ext, wr.ext);
        assert_eq!(rr.release_id, wr.release_id);
    }
}

#[test]
fn the_encoder_writes_schema_3_whatever_the_index_claims() {
    let mut index = sold();
    index.schema_version = 2;
    assert_eq!(decode(&blob(index)).expect("decodes").schema_version, SCHEMA_V3);
}
