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

//! Every parser of untrusted input, fed arbitrary bytes: none panics, and
//! each answers with a value or a reason. The input is the release directory,
//! the tree's ledger, a verifier's report or summary, and a key file.

use crate::release::{kernel, receipt, record};
use crate::sign::key::seed;
use crate::sign::named::{line, signer_named};
use crate::tree::{declared, manifest, report, summary};
use proptest::prelude::*;
use std::path::Path;

proptest! {
    #![proptest_config(ProptestConfig { cases: 2048, failure_persistence: None, ..ProptestConfig::default() })]

    #[test]
    fn no_parser_panics_on_noise(b in prop::collection::vec(any::<u8>(), 0..4096)) {
        let _ = record::parse(&b);
        let _ = receipt::parse(&b);
        let _ = manifest::parse(&b);
        let _ = kernel::parse(&b);
        let _ = report::parse(&b);
        let _ = declared::parse(&b);
        let _ = summary::steps(&b, "nonos-harden");
        let _ = seed(&b, Path::new("/nonexistent/nonos-harden-key"));
    }

    #[test]
    fn no_text_reader_panics(s in "\\PC*") {
        let _ = signer_named(&s);
        let _ = line(&s);
        let _ = record::parse(s.as_bytes());
        let _ = manifest::parse(s.as_bytes());
        let _ = declared::parse(s.as_bytes());
    }

    #[test]
    fn a_footer_shaped_tail_never_panics(
        head in prop::collection::vec(any::<u8>(), 0..4000),
        tail in prop::collection::vec(any::<u8>(), 56),
    ) {
        let b = [head.as_slice(), b"NONOSIMG", &tail].concat();
        let _ = kernel::parse(&b);
    }
}
