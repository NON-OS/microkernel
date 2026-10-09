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
NAMES = {" ": "spc", "-": "minus", ".": "dot", "/": "slash"}


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

    def type_text(self, text):
        for ch in text:
            if ch.isupper():
                self.key("shift", ch.lower())
            else:
                self.key(NAMES.get(ch, ch))
            time.sleep(0.05)

    def quit(self):
        try:
            self.cmd("quit")
        except (OSError, ConnectionError):
            return
