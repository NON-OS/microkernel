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

//! read_doc at 256 KiB: a document of it decoded whole, one byte more refused.

use nonos_marketplace_abi::encode_entry_document;

use crate::clients::vfs;
use crate::document::document_of;
use crate::publish::files::read_doc;

const DOC_MAX: usize = 256 * 1024;
const PATH: &str = "/data/listing.entry";

#[test]
fn read_doc_decodes_a_document_of_256_kib_whole() {
    let doc = document_of(DOC_MAX);
    vfs::put(PATH.as_bytes(), doc.clone());
    let back = read_doc(7, PATH).map(|e| encode_entry_document(&e));
    assert_eq!(back, Ok(doc));
}

#[test]
fn read_doc_refuses_a_document_one_byte_past_256_kib() {
    // Its first 256 KiB are a whole document, so only the length refuses it.
    let mut doc = document_of(DOC_MAX);
    doc.push(0);
    vfs::put(PATH.as_bytes(), doc);
    let err = read_doc(7, PATH).err();
    assert_eq!(err, Some("that file is not one listing's entry document"));
}
