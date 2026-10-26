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
"""A small QMP client: screenshots and keystrokes for the e2e runs."""

import json
import socket
import time
# A US keyboard: each printable character as the qcode that types it, and the
# characters typed with shift. A character in neither is refused, never sent
# as a qcode QEMU does not know and so dropped without a word.
NAMES = {" ": "spc", "-": "minus", ".": "dot", "/": "slash", ",": "comma", ";": "semicolon",
         "'": "apostrophe", "[": "bracket_left", "]": "bracket_right", "\\": "backslash",
         "=": "equal", "`": "grave_accent", "\t": "tab", "\n": "ret"}
SHIFTED = {"!": "1", "@": "2", "#": "3", "$": "4", "%": "5", "^": "6", "&": "7", "*": "8",
           "(": "9", ")": "0", "_": "minus", "+": "equal", "{": "bracket_left",
           "}": "bracket_right", "|": "backslash", ":": "semicolon", '"': "apostrophe",
           "<": "comma", ">": "dot", "?": "slash", "~": "grave_accent"}


def keys_for(ch):
    """The qcodes, pressed together, that type `ch` on a US keyboard."""
    if ch.isascii() and ch.isalnum():
        return ("shift", ch.lower()) if ch.isupper() else (ch,)
    if ch in NAMES:
        return (NAMES[ch],)
    if ch in SHIFTED:
        return ("shift", SHIFTED[ch])
    raise ValueError(f"cannot type {ch!r}: no key for it on a US keyboard")


class Qmp:
    def __init__(self, path, wait=30.0):
        end = time.time() + wait
        while True:
            try:
                self.sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
                self.sock.connect(path)
                break
            except OSError:
                if time.time() > end:
                    raise
                time.sleep(0.2)
        self.buf = b""
        self._read()
        self.cmd("qmp_capabilities")

    def _read(self):
        while b"\n" not in self.buf:
            chunk = self.sock.recv(65536)
            if not chunk:
                raise ConnectionError("QMP closed")
            self.buf += chunk
        line, self.buf = self.buf.split(b"\n", 1)
        return json.loads(line)

    def cmd(self, name, **args):
        self.sock.sendall(json.dumps({"execute": name, "arguments": args}).encode() + b"\n")
        while True:
            msg = self._read()
            if "return" in msg or "error" in msg:
                return msg

    def shot(self, path):
        return self.cmd("screendump", filename=path, format="png")

    def key(self, *names, hold_ms=80):
        keys = [{"type": "qcode", "data": n} for n in names]
        return self.cmd("send-key", keys=keys, **{"hold-time": hold_ms})

    def _event(self, name, down):
        event = {"type": "key", "data": {"down": down, "key": {"type": "qcode", "data": name}}}
        reply = self.cmd("input-send-event", events=[event])
        if "error" in reply:
            raise RuntimeError(f"QEMU refused {name}: {reply['error']}")

    def type_text(self, text, gap=0.04):
        """Type `text`: each character's keys pressed in order and released in
        reverse, one event at a time with a gap between, so a guest busy under
        TCG sees shift go down before the key and up after it, every time
        (send-key's chords left shift held in the guest after a run of
        capitals)."""
        keys = [keys_for(ch) for ch in text]
        for chord in keys:
            for name in chord:
                self._event(name, True)
                time.sleep(gap)
            for name in reversed(chord):
                self._event(name, False)
                time.sleep(gap)
            time.sleep(2 * gap)

    def quit(self):
        try:
            self.cmd("quit")
        except (OSError, ConnectionError):
            return
