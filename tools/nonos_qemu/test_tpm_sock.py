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

"""The TPM's control socket stays within a unix socket's path limit, however
deep the checkout: beside the state when it fits, else in a short dir."""

import os
import unittest

from .tpm import SOCK_MAX, sock_path


class Sock(unittest.TestCase):
    def test_a_shallow_state_keeps_its_socket(self):
        self.assertEqual(sock_path("/r/target/swtpm"), "/r/target/swtpm/swtpm-sock")

    def test_a_deep_state_gets_a_short_one(self):
        deep = "/" + "d" * 120 + "/target/swtpm"
        sock = sock_path(deep, short=lambda: "/tmp/nonos-tpm-x")
        self.assertEqual(sock, "/tmp/nonos-tpm-x/swtpm-sock")
        self.assertLessEqual(len(os.fsencode(sock)), SOCK_MAX)
