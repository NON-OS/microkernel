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
"""A server on this machine for the mirror tests: it answers a byte range as
a mirror or Hugging Face does, so a resumed download is tested for real."""

import contextlib
import http.server
import io
import threading


class Ranged(http.server.BaseHTTPRequestHandler):
    files, starts = {}, []

    def do_GET(self):
        body = self.files.get(self.path.lstrip("/"))
        if body is None:
            return self.send_error(404)
        asked = self.headers.get("Range")
        start = int(asked.split("=")[1].split("-")[0]) if asked else 0
        self.starts.append(start)
        self.send_response(206 if asked else 200)
        self.send_header("Content-Length", str(len(body) - start))
        self.end_headers()
        self.wfile.write(body[start:])

    def log_message(self, *args):
        pass


def serve(files):
    """A running server for `files` (name -> bytes); its base URL, and it."""
    Ranged.files, Ranged.starts = files, []
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Ranged)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    return f"http://127.0.0.1:{server.server_address[1]}", server


def quietly(f, *args):
    """`f(*args)` with its progress lines kept off the test's output."""
    with contextlib.redirect_stdout(io.StringIO()):
        f(*args)
