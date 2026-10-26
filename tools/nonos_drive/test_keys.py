# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

"""What a type: step sends: every printable US character as the keys that
type it, so a boot's typed line reaches the guest whole, and a character with
no key refused before anything is sent, never dropped."""

import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "userland/capsule_install/e2e"))

from qmp import Qmp, keys_for  # noqa: E402


class Keys(unittest.TestCase):
    def test_letters_digits_and_capitals(self):
        self.assertEqual(keys_for("a"), ("a",))
        self.assertEqual(keys_for("7"), ("7",))
        self.assertEqual(keys_for("Q"), ("shift", "q"))

    def test_the_shell_symbols_are_shifted_keys(self):
        want = {"<": ("shift", "comma"), ">": ("shift", "dot"), "?": ("shift", "slash"),
                '"': ("shift", "apostrophe"), "(": ("shift", "9"), ")": ("shift", "0"),
                "|": ("shift", "backslash"), "_": ("shift", "minus"), "+": ("shift", "equal")}
        for ch, keys in want.items():
            self.assertEqual(keys_for(ch), keys, ch)

    def test_the_unshifted_symbols(self):
        want = {" ": "spc", ",": "comma", ";": "semicolon", "'": "apostrophe",
                "\\": "backslash", "=": "equal", "[": "bracket_left", "\n": "ret"}
        for ch, key in want.items():
            self.assertEqual(keys_for(ch), (key,), repr(ch))

    def test_every_printable_character_has_keys(self):
        for code in range(0x20, 0x7f):
            self.assertTrue(keys_for(chr(code)), chr(code))

    def test_a_character_with_no_key_is_refused(self):
        for ch in ["é", "\x00", "€"]:
            with self.assertRaises(ValueError):
                keys_for(ch)


class Sent:
    """Takes input-send-event as QEMU does: a qcode it knows, or an error;
    `events` is every key event in order, as (key, down)."""

    def __init__(self, known=None):
        self.known, self.events = known, []

    def cmd(self, name, **args):
        assert name == "input-send-event", name
        for e in args["events"]:
            key = e["data"]["key"]["data"]
            if self.known is not None and key not in self.known:
                return {"error": {"desc": "invalid parameter"}}
            self.events.append((key, e["data"]["down"]))
        return {"return": {}}


def typed(text, known=None, sent=None):
    q = Qmp.__new__(Qmp)
    sent = sent or Sent(known)
    q.cmd = sent.cmd
    q.type_text(text, gap=0)
    return sent.events


def tap(*keys):
    return [(k, True) for k in keys] + [(k, False) for k in reversed(keys)]


class Typing(unittest.TestCase):
    def test_a_redirect_is_typed_whole(self):
        want = tap("a") + tap("spc") + tap("shift", "comma") + tap("spc") + tap("b")
        self.assertEqual(typed("a < b"), want)

    def test_shift_goes_down_first_and_up_last_for_every_capital(self):
        want = tap("shift", "p") + tap("shift", "n") + tap("shift", "g")
        self.assertEqual(typed("PNG"), want)

    def test_every_key_pressed_is_released(self):
        events = typed('import zlib;print("A<B>C?") | x')
        downs = sorted(k for k, d in events if d)
        ups = sorted(k for k, d in events if not d)
        self.assertEqual(downs, ups)

    def test_a_line_with_no_key_sends_nothing(self):
        sent = Sent()
        with self.assertRaises(ValueError):
            typed("ok é", sent=sent)
        self.assertEqual(sent.events, [], "a line is checked whole before a key is sent")

    def test_a_key_qemu_refuses_stops_the_run(self):
        sent = Sent(known={"a"})
        with self.assertRaises(RuntimeError):
            typed("ab", sent=sent)
        self.assertEqual(sent.events, tap("a"))


if __name__ == "__main__":
    unittest.main()
