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

//! find pinned whole (L79). Its code, with comments cut and its log lines'
//! words left out, must be exactly the code below, statement by statement:
//! nothing negated or narrowed inside `mine`, no way out before the store,
//! no name shadowed after it is set, and nothing hidden behind a dead
//! attribute or inside a comment. Only the words it logs may change. The
//! model in cover_find.rs restates this code, so the two cannot drift.

use super::cover_glue_tests::COVER;
use super::glue_text::{bare, body};

const HEAD: &str = "pub(super)fnfind(page:u64,len:u64){";

const FIND: &str = concat!(
    "letall=table::list();",
    "letmine=|r:&DeviceRecord|{",
    "letspans:[Span;6]=r.bars.map(|b|(b.base,b.size,b.kind));",
    "holds((page,len),&spans,usize::from(r.bar_count),BarKind::Mmioasu8)",
    "};",
    "letany=all.iter().any(mine);",
    "letsingle=all.iter().filter(|r|display(r)).count()==1;",
    "letby=|r:&&DeviceRecord|rule(mine(r),any,display(r),single);",
    "letids:Vec<u64>=all.iter().filter(by).map(|r|r.device_id).collect();",
    "foridin&ids{",
    "lethow=ifany{\"\"}else{\"\"};",
    "letline=format!(\"\",how,id);",
    "serial::println(line.as_bytes());",
    "}",
    "ifids.is_empty(){",
    "serial::println(b\"\");",
    "}",
    "COVER.call_once(||ids);",
);

#[test]
fn find_is_exactly_the_code_the_model_restates() {
    let src = bare(COVER);
    assert_eq!(src.matches("fnfind(").count(), 1, "a second find");
    assert_eq!(body(&src, HEAD), FIND, "find's code changed");
}

#[test]
fn the_store_is_reached_by_find_alone_and_read_by_covers_alone() {
    let src = bare(COVER);
    // One static, set in find and read in covers, and nothing else naming it.
    assert_eq!(src.matches("staticCOVER:Once<Vec<u64>>=Once::new();").count(), 1);
    assert_eq!(src.matches("COVER").count(), 3, "the store named elsewhere");
    let head = "pub(incrate::hardware::broker)fncovers(device_id:u64)->bool{";
    assert_eq!(body(&src, head), "COVER.get().is_some_and(|v|v.contains(&device_id))");
}
