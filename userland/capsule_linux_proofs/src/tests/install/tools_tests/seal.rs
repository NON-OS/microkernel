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

//! The seal's content, read back whole and carrying no proof.

use crate::install::tar::{entries, walk, Kind};

/// A tool's content as the seal writes it (tools/nonos_market_catalogue/
/// packages.py `bundle`), for a program, two library files and a link.
const CONTENT: &[u8] = include_bytes!("../../../../vectors/tool-content.tar");

#[test]
fn the_seals_content_reads_back_whole_with_the_personalitys_tar() {
    let w = walk(CONTENT);
    assert_eq!(w.dropped, 0);
    let names: alloc::vec::Vec<&[u8]> = w.entries.iter().map(|e| e.name.as_slice()).collect();
    assert_eq!(
        names,
        [&b"usr/bin/perl"[..], b"usr/lib/perl5/Data/Dumper.pm", b"usr/lib/perl5/strict.pm", b"usr/bin/perl5.44.0"]
    );
    assert!(w.entries[..3].iter().all(|e| matches!(e.kind, Kind::File)));
    /* The other name the tool answers to, for the link table, as a full path. */
    assert!(matches!(&w.entries[3].kind, Kind::Symlink(to) if to == b"/usr/bin/perl"));
    let files = entries(CONTENT);
    assert!(files[0].body.starts_with(b"\x7fELF"), "the program is the bytes it was");
    assert_eq!(files[2].body, b"package strict;\n1;\n");
}

#[test]
fn the_seals_content_carries_no_proof() {
    for e in walk(CONTENT).entries {
        for proof in [&b".nonos_id_cert.bin"[..], b".manifest.bin", b".zk_trailer.bin"] {
            assert!(!e.name.ends_with(proof), "{:?}", core::str::from_utf8(&e.name));
        }
    }
}
