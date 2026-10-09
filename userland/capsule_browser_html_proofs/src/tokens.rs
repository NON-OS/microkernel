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

//! Tokens as short strings for the tokenizer proofs: text quoted, a comment
//! as "#", a doctype as "!name", tags with their attributes and slash.

use crate::browser::html::tokenizer::{TextMode, Token, Tokenizer};

/// Every token of `src`, read from `mode` as if `last` were the last start
/// tag, the way the tree builder leaves the tokenizer between tokens.
pub fn tokens(src: &str, mode: TextMode, last: &str) -> Vec<String> {
    let mut t = Tokenizer::with_state(src, mode, last);
    let mut out = Vec::new();
    loop {
        let item = match t.next_token() {
            Token::Eof => return out,
            Token::Chars(s) => format!("'{s}'"),
            Token::Null => "NUL".into(),
            Token::Comment => "#".into(),
            Token::Doctype(d) => format!("!{}", d.name.unwrap_or_default()),
            Token::End(tag) => format!("/{}", tag.name),
            Token::Start(tag) => {
                let attrs: String = tag.attrs.iter().map(|(k, v)| format!(" {k}={v}")).collect();
                let slash = if tag.self_closing { "/" } else { "" };
                format!("<{}{attrs}{slash}", tag.name)
            }
        };
        out.push(item);
    }
}
