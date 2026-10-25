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

//! The source reader the glue checks share takes out what the compiler
//! ignores and nothing else: a call in any kind of comment is gone, one in a
//! literal or in code stays, and the layout of what stays is kept.

use super::code::{code, flat};

#[test]
fn a_commented_call_goes_and_a_literal_stays() {
    let src = "a(); /* b(); /* c(); */ d(); */ e(); // f();\n/// g();\n//! h();\ni(); /**/ j();";
    assert_eq!(flat(src), "a(); e(); i(); j();", "a call in a comment kept");
    let lits = r#""// k();" "\"/* l(); */\"" '"' '\'' m::<'a>(); // n();"#;
    assert_eq!(flat(lits), r#""// k();" "\"/* l(); */\"" '"' '\'' m::<'a>();"#, "a literal cut");
    assert_eq!(code("    o(); // p\n}\n"), "    o();  \n}\n", "the layout not kept");
}
