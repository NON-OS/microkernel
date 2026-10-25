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
 * Where the publisher screen reads and writes: the
 * person types the path of one entry document under /data, and each
 * signature goes beside it, <doc>.binding.sig and
 * <doc>.<release_id>.attest.sig, for marketplace-index to take. A path
 * outside /data, one that climbs out of it, or a release id that would
 * name another directory is refused.
 */

use alloc::string::String;

const ROOT: &str = "/data/";
const PATH_MAX: usize = 200;

fn plain(part: &str) -> bool {
    !part.is_empty()
        && part != "."
        && part != ".."
        && part.bytes().all(|b| b.is_ascii_graphic() && b != b'/' && b != b'\\')
}

pub fn doc_path_ok(path: &str) -> bool {
    path.len() <= PATH_MAX && path.strip_prefix(ROOT).is_some_and(|rest| rest.split('/').all(plain))
}

pub fn binding_sig_path(doc: &str) -> Option<String> {
    doc_path_ok(doc).then(|| [doc, ".binding.sig"].concat())
}

pub fn attest_sig_path(doc: &str, release_id: &str) -> Option<String> {
    (doc_path_ok(doc) && plain(release_id)).then(|| [doc, ".", release_id, ".attest.sig"].concat())
}
