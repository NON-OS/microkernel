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

"""--forward HOST:GUEST forwards a TCP port to the guest, bound to the
host's loopback only, so a guest listener (nsh on 2222) is reachable from
this machine and never from another. Nothing here starts QEMU."""

import os
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import machine  # noqa: E402


def netdev_of(argv):
    return argv[argv.index("-netdev") + 1]


class Forward(unittest.TestCase):
    def test_a_forward_is_bound_to_loopback(self):
        argv = machine.devices("nat", None, None, forwards=[(2222, 2222)])
        self.assertEqual(netdev_of(argv), "user,id=net0,hostfwd=tcp:127.0.0.1:2222-:2222")

    def test_several_forwards_and_none(self):
        argv = machine.devices("nat", None, None, forwards=[(2222, 2222), (8022, 22)])
        self.assertEqual(netdev_of(argv).count("hostfwd=tcp:127.0.0.1:"), 2)
        self.assertEqual(netdev_of(machine.devices("nat", None, None)), "user,id=net0")

    def test_no_forward_ever_binds_every_interface(self):
        argv = machine.devices("nat", None, None, forwards=[(1, 65535)])
        self.assertNotIn("hostfwd=tcp::", netdev_of(argv))
        self.assertNotIn("0.0.0.0", netdev_of(argv))

    def test_the_pair_is_two_ports(self):
        self.assertEqual(machine.forward("2222:2222"), (2222, 2222))
        for bad in ["2222", "0:22", "22:65536", "a:22", "22:22:22", "-1:22", ""]:
            self.assertIsNone(machine.forward(bad), bad)

    def test_net_off_has_no_forward(self):
        self.assertNotIn("-netdev", machine.devices("off", None, None, forwards=[(2222, 2222)]))


if __name__ == "__main__":
    unittest.main()
