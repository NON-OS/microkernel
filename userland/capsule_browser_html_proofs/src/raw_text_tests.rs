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

//! The text states (13.2.5.2 to 13.2.5.29): RCDATA, RAWTEXT, script data
//! with its escapes, PLAINTEXT, and CDATA where foreign content allows it.

use crate::browser::html::tokenizer::{TextMode, Tokenizer};
use crate::tokens::tokens;

#[test]
fn raw_text_ends_only_at_the_appropriate_end_tag() {
    let rc = tokens("a&amp;</b></TITLE >x", TextMode::Rcdata, "title");
    assert_eq!(rc, ["'a&</b>'", "/title", "'x'"]);
    let raw = tokens("a&amp;</style", TextMode::Rawtext, "style");
    assert_eq!(raw, ["'a&amp;</style'"]);
    let pt = tokens("</plaintext>&amp;", TextMode::Plaintext, "plaintext");
    assert_eq!(pt, ["'</plaintext>&amp;'"]);
}

#[test]
fn script_data_escapes_hide_a_nested_end_tag() {
    let s = tokens("<!--<script></script>x</script>y", TextMode::ScriptData, "script");
    assert_eq!(s, ["'<!--<script></script>x'", "/script", "'y'"]);
    let t = tokens("a<!--b--></script>", TextMode::ScriptData, "script");
    assert_eq!(t, ["'a<!--b-->'", "/script"]);
}

#[test]
fn cdata_opens_only_in_foreign_content() {
    let mut t = Tokenizer::new("<![CDATA[a<b]]>c");
    t.foreign = true;
    let first = t.next_token();
    assert!(matches!(first, crate::browser::html::tokenizer::Token::Chars(ref s) if s == "a<b"));
    assert_eq!(tokens("<![CDATA[a<b]]>c", TextMode::Data, ""), ["#", "'c'"]);
}
