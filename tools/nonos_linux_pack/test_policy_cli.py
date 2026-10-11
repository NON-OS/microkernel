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
"""The policy command end to end: E39's ids, a list beside NET_ANON only,
and the 256 bytes written where asked."""

import contextlib
import io
import tempfile
import unittest
from pathlib import Path

from .blake3 import blake3
from .policy_cli import main
from .test_policy import CALLS

PUB = "bbeaa6c15c496575134e0c72d8d75d7d6128c9fdf3723761f44026a0adf522f2"


def run(*extra, dests=None):
    with tempfile.TemporaryDirectory() as d:
        program, out = Path(d, "prog"), Path(d, "policy.bin")
        program.write_bytes(b"\x7fELF" + bytes(3000))
        argv = ["--program", str(program), "--calls", "calls=" + CALLS, "--publisher", PUB,
                "--app", "linux.qwen-small", "--release", "qwen-small@1", "--out", str(out), *extra]
        if dests is not None:
            Path(d, "dests").write_text(dests)
            argv += ["--dests", str(Path(d, "dests"))]
        said, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(said), contextlib.redirect_stderr(err):
            code = main(argv)
        written = out.read_bytes() if out.exists() else None
        return code, said.getvalue(), written, blake3(program.read_bytes())


class PolicyCommand(unittest.TestCase):
    def test_a_release_policy_with_e39_ids(self):
        code, said, written, program = run("--mask", "WAYLAND,MODELS_RO,TTY")
        self.assertEqual(code, 0)
        self.assertIn("listing_id=ff4399cc04b31c66714c8be18d9b33e33912a83ccbc7ede50fce0a31d8414cd5", said)
        self.assertIn("release_id=af2e230eabc03a74fb38620601f259a086d9924a715b352c66d09c23694a864f", said)
        self.assertEqual(len(written), 256)
        self.assertEqual(written[16:48], program)
        self.assertIn(f"guest_policy={written.hex()}", said)
        self.assertEqual(written[120:152], bytes(32))

    def test_a_list_needs_net_anon_and_lands_in_the_hash(self):
        self.assertEqual(run("--mask", "TTY", dests="a.example:443\n")[0], 1)
        code, said, written, _ = run("--mask", "NET_ANON,TTY", dests="a.example:443, b.example:80\n")
        self.assertEqual(code, 0)
        self.assertIn("dests=0002", said)
        self.assertNotEqual(written[120:152], bytes(32))

    def test_bad_inputs_are_refused(self):
        self.assertEqual(run("--mask", "WAYLAND,NOPE")[0], 1)
        self.assertEqual(run("--mask", "TTY", dests="Example.org:443")[0], 1)


if __name__ == "__main__":
    unittest.main()
