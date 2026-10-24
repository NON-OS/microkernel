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
"""The installer's chain on a real Alpine package: its control hashes to the
signed index's C: and its data to the control's datahash, and any change
breaks it; an index is read only when a pinned key signed it. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import gzip
import unittest

from . import alpine_auth, alpine_index, apkindex, gz, tarwalk
from .apk_fixture import tar
from .vector_fixture import APK, REC, flip


class Package(unittest.TestCase):
    def test_the_chain_holds_and_yields_only_the_data(self):
        files = alpine_auth.package_files(APK, alpine_auth.checksum(REC["C"]))
        self.assertEqual([e.name for e in tarwalk.files(files)], [b"usr/lib/udev/rules.d/41-nitrokey.rules"])

    def test_any_change_to_control_or_data_breaks_the_chain(self):
        parts, want = gz.members(APK), alpine_auth.checksum(REC["C"])
        for raw in (flip(APK, parts[1].start + 20), flip(APK, len(APK) - 9), APK[:parts[2].start]):
            with self.assertRaises(ValueError):
                alpine_auth.package_files(raw, want)
        swapped = APK[:parts[2].start] + gzip.compress(tar([("usr/bin/evil", b"x")]))
        with self.assertRaises(ValueError):
            alpine_auth.package_files(swapped, want)
        with self.assertRaises(ValueError):
            alpine_auth.package_files(APK, bytes(20))
        for bad in ("", "Q2" + REC["C"][2:], REC["C"][:-4], None):
            with self.assertRaises(ValueError, msg=bad):
                alpine_auth.checksum(bad)


class Index(unittest.TestCase):
    def index(self, sig_name: bytes, members=2, first=("DESCRIPTION", b"v3.20")):
        body = tar([first, ("APKINDEX", b"P:a\nV:1-r0\n\nP:b\nV:2-r0\n")])
        sig = tar([(sig_name.decode(), b"s")])
        return b"".join(gzip.compress(m) for m in [sig, body] + [body] * (members - 2))

    def test_an_index_is_read_only_under_a_pinned_key_and_the_right_digest(self):
        seen = []
        ok = lambda key, sig, h, d: seen.append(h) or True
        name = (".SIGN.RSA256." + alpine_index.NAMES[1]).encode()
        recs = apkindex.records(apkindex.text(alpine_index.signed_index(self.index(name), ok)))
        self.assertEqual([r["P"] for r in recs], ["a", "b"])
        self.assertEqual(seen, ["sha256"])
        for bad in (self.index(b".SIGN.RSA.unpinned.rsa.pub"), self.index(name, members=3)):
            with self.assertRaises(ValueError):
                alpine_index.signed_index(bad, ok)
        with self.assertRaises(ValueError):
            alpine_index.signed_index(self.index(name), lambda *a: False)
        self.assertEqual(apkindex.records(apkindex.text(tar([("DESCRIPTION", b"P:x\nV:1\n")]))), [])


if __name__ == "__main__":
    unittest.main()
