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

//! A clone's git step holds one request and gets its own answer back once.

use crate::git_ask::Ask;

#[test]
fn a_step_asks_then_gets_its_answer_once() {
    let mut ask = Ask::new();
    assert_eq!(ask.request(b"GET /info/refs".to_vec()), None);
    let carried = ask.take_asked().expect("the request is left for the job");
    assert_eq!(carried, b"GET /info/refs".to_vec());
    assert_eq!(ask.take_asked(), None);
    ask.answered(carried, b"refs".to_vec());
    assert_eq!(ask.request(b"GET /info/refs".to_vec()), Some(b"refs".to_vec()));
    // Taken: the same request asks again rather than reading a stale answer.
    assert_eq!(ask.request(b"GET /info/refs".to_vec()), None);
}

#[test]
fn an_answer_is_never_handed_to_another_request() {
    let mut ask = Ask::new();
    ask.answered(b"GET /info/refs".to_vec(), b"refs".to_vec());
    assert_eq!(ask.request(b"POST /git-upload-pack".to_vec()), None);
    assert_eq!(ask.take_asked(), Some(b"POST /git-upload-pack".to_vec()));
    // The mismatched answer was dropped, not kept for later.
    assert_eq!(ask.request(b"GET /info/refs".to_vec()), None);
}
