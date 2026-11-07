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

//! The accent keys of the German, French and Spanish layouts are dead:
//! each is held for the next key, which is posted with the accent on it.

use nonos_keymap::{accent_of, compose, dead, resolve, Accent, HeldKeys, KeyPosts, Layout};

const SHIFT: u32 = 0x1003;
const ENTER: u32 = 0x0D;
const LEFT: u32 = 0x1203;

/// One key's press and release on a board in `layout`, as a driver posts
/// them: the codes posted down, then up.
struct Board {
    held: HeldKeys,
    layout: Layout,
    shift: bool,
    out: Vec<(bool, u32)>,
}

impl Board {
    fn new(layout: Layout) -> Board {
        Board { held: HeldKeys::new(), layout, shift: false, out: Vec::new() }
    }

    fn post(&mut self, p: KeyPosts, down: bool) {
        if let Some(c) = p.release_first {
            self.out.push((false, c));
        }
        if let Some(c) = p.code {
            self.out.push((down, c));
        }
    }

    /// The physical key named by its US base character.
    fn event(&mut self, base: char, release: bool) {
        let code = resolve(base as u32, self.shift, false, false, self.layout);
        let p = self.held.event(base as u32, release, code);
        self.post(p, !release);
    }

    fn tap(&mut self, base: char) {
        self.event(base, false);
        self.event(base, true);
    }

    fn shifted(&mut self, base: char) {
        let p = self.held.event(SHIFT, false, SHIFT);
        self.post(p, true);
        self.shift = true;
        self.tap(base);
        self.shift = false;
        let p = self.held.event(SHIFT, true, SHIFT);
        self.post(p, false);
    }

    fn other(&mut self, code: u32) {
        let p = self.held.event(code, false, code);
        self.post(p, true);
        let p = self.held.event(code, true, code);
        self.post(p, false);
    }

    /// The characters posted down, in order, modifiers left out.
    fn typed(&self) -> String {
        let chars = self.out.iter().filter(|(down, c)| *down && *c < 0x1000);
        chars.filter_map(|(_, c)| char::from_u32(*c)).collect()
    }

    /// Every press is let go with the code it went down with, as many times.
    fn paired(&self) -> bool {
        let mut downs: Vec<u32> = self.out.iter().filter(|(d, _)| *d).map(|(_, c)| *c).collect();
        let mut ups: Vec<u32> = self.out.iter().filter(|(d, _)| !*d).map(|(_, c)| *c).collect();
        downs.sort_unstable();
        ups.sort_unstable();
        downs == ups
    }
}

#[test]
fn french_circumflex_and_diaeresis_go_on_the_next_letter() {
    let mut b = Board::new(Layout::Fr);
    b.tap('[');
    b.tap('e');
    b.shifted('[');
    b.tap('i');
    b.tap('[');
    b.shifted('e');
    assert_eq!(b.typed(), "êïÊ");
    assert!(b.paired());
}

#[test]
fn german_accents_go_on_the_next_letter() {
    let mut b = Board::new(Layout::De);
    b.tap('=');
    b.tap('e');
    b.shifted('=');
    b.tap('a');
    b.tap('`');
    b.tap('o');
    assert_eq!(b.typed(), "éàô");
    assert!(b.paired());
}

#[test]
fn spanish_accents_go_on_the_next_letter() {
    let mut b = Board::new(Layout::Es);
    b.tap('\'');
    b.tap('a');
    b.shifted('\'');
    b.tap('u');
    b.tap('[');
    b.tap('e');
    b.shifted('[');
    b.tap('o');
    assert_eq!(b.typed(), "áüèô");
    assert!(b.paired());
}

#[test]
fn space_or_the_key_twice_gives_the_accent_itself() {
    let mut b = Board::new(Layout::Fr);
    b.tap('[');
    b.tap(' ');
    b.tap('[');
    b.tap('[');
    b.shifted('[');
    b.tap(' ');
    assert_eq!(b.typed(), "^^¨");
    assert!(b.paired());
}

#[test]
fn a_key_the_accent_does_not_go_on_drops_it() {
    let mut b = Board::new(Layout::Fr);
    b.tap('[');
    b.tap('x');
    b.tap('x');
    assert_eq!(b.typed(), "xx");
    let mut b = Board::new(Layout::De);
    b.tap('`');
    b.tap('1');
    assert_eq!(b.typed(), "1");
}

#[test]
fn enter_or_an_arrow_drops_the_accent_and_is_posted() {
    let mut b = Board::new(Layout::Fr);
    b.tap('[');
    b.other(ENTER);
    b.tap('e');
    b.tap('[');
    b.other(LEFT);
    b.tap('e');
    assert_eq!(b.typed(), "\ree");
    assert!(b.out.contains(&(true, LEFT)) && b.out.contains(&(false, LEFT)));
    assert!(b.paired());
}

#[test]
fn a_held_dead_key_repeats_nothing_and_a_composed_letter_repeats_itself() {
    let mut b = Board::new(Layout::Fr);
    b.event('[', false);
    b.event('[', false);
    b.event('[', false);
    b.event('[', true);
    assert!(b.out.is_empty(), "a dead key posts nothing, repeated or let go");
    b.event('e', false);
    b.event('e', false);
    b.event('e', false);
    b.event('e', true);
    assert_eq!(b.out, [(true, 0xEA), (true, 0xEA), (true, 0xEA), (false, 0xEA)]);
}

#[test]
fn layouts_with_no_dead_keys_are_as_before() {
    for layout in [Layout::Us, Layout::Uk, Layout::It] {
        for base in 0x20u32..=0x7E {
            for shift in [false, true] {
                let c = resolve(base, shift, false, false, layout);
                assert_eq!(accent_of(c), None, "{layout:?} {base:#x} shift {shift}");
            }
        }
    }
    let mut b = Board::new(Layout::Us);
    b.shifted('6');
    b.tap('e');
    assert_eq!(b.typed(), "^e");
}

#[test]
fn every_composed_letter_is_latin1_and_capitals_match() {
    let accents = [Accent::Grave, Accent::Acute, Accent::Circumflex, Accent::Diaeresis];
    for a in accents {
        assert_eq!(accent_of(dead(a)), Some(a));
        assert!(char::from_u32(dead(a)).is_none(), "a dead code is no character");
        for c in b'a'..=b'z' {
            let Some(small) = compose(a, c as u32) else { continue };
            assert!((0xA0..=0xFF).contains(&small), "{a:?} {}", c as char);
            let lower = char::from_u32(small).unwrap();
            if let Some(big) = compose(a, c.to_ascii_uppercase() as u32) {
                let upper = char::from_u32(big).unwrap();
                assert_eq!(upper.to_lowercase().next(), Some(lower), "{a:?} {}", c as char);
            }
        }
    }
    assert_eq!(compose(Accent::Diaeresis, 'Y' as u32), None);
    assert_eq!(accent_of(0x10FFFF), None);
    assert_eq!(accent_of(dead(Accent::Diaeresis) + 1), None);
}
