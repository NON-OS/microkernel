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

use crate::browser::html::tokenizer::{TextMode, Token, Tokenizer};

use super::super::super::limits::TRUNC_ATTRS;
use super::super::super::node::Ns;
use super::super::ops::state::Builder;

impl Builder {
    /// Tokenize `src`, starting in `state`, into this builder until the input
    /// ends or parsing stops. The tokenizer is told after every token what
    /// the tree builder asked of it: a raw text state after a start tag, and
    /// whether a CDATA section may open.
    pub fn run(&mut self, src: &str, state: TextMode) {
        let mut tok = Tokenizer::with_state(src, state, "");
        loop {
            tok.foreign = !self.open.is_empty() && self.ns(self.adjusted()) != Ns::Html;
            let token = tok.next_token();
            let eof = matches!(token, Token::Eof);
            self.process(token);
            if let Some(next) = self.switch_to.take() {
                tok.mode = next;
            }
            if eof || self.stopped {
                break;
            }
        }
        if tok.dropped_attrs {
            self.dom.truncated |= TRUNC_ATTRS;
        }
    }

    pub(in super::super) fn ns(&self, id: usize) -> Ns {
        self.dom.nodes[id].ns
    }
}
