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
"""SimpleX's terminal client is listed with the guest's SOCKS endpoint and
public relay names, as the SimpleX window tells its core: started without
them it resolves each relay's name itself, which no guest can, and sends
nothing. The endpoint is capsule_linux's (net/routed/endpoint.rs, AT), and
the host mode the window's (capsule_simplex_ui proto/cmd.rs, NETWORK). Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import re
import unittest
from pathlib import Path

from .listed_args import block
from .recipes_file import load

ROOT = Path(__file__).resolve().parents[2]
CATALOGUE = ROOT / "userland/capsule_market/linux-catalogue.json"
ENDPOINT = ROOT / "userland/capsule_linux/src/linux/net/routed/endpoint.rs"
WINDOW = ROOT / "userland/capsule_simplex_ui/src/proto/cmd.rs"


def endpoint() -> str:
    """ip:port of the endpoint every NET_ANON guest has."""
    m = re.search(r"pub const AT: Addr = Addr \{ ip: \[(\d+), (\d+), (\d+), (\d+)\], port: (\d+) \};",
                  ENDPOINT.read_text())
    if m is None:
        raise AssertionError(f"{ENDPOINT} names no endpoint")
    return ".".join(m.groups()[:4]) + ":" + m.group(5)


def window_says(word: str) -> str:
    """The value the SimpleX window's /network command gives `word`."""
    m = re.search(r'pub const NETWORK: &str = "/network ([^"]*)";', WINDOW.read_text())
    if m is None:
        raise AssertionError(f"{WINDOW} names no /network command")
    return dict(w.split("=", 1) for w in m.group(1).split())[word]


class TerminalClient(unittest.TestCase):
    def setUp(self):
        (self.recipe,) = [r for r in json.loads(CATALOGUE.read_text())["apps"] if r["package"] == "simplex-chat"]

    def test_it_reaches_relays_through_the_guest_endpoint_by_their_public_names(self):
        self.assertIn("NET_ANON", self.recipe["mask"].split())
        self.assertEqual(window_says("socks"), "on")
        args = dict(w.split("=", 1) for w in self.recipe["args"])
        self.assertEqual(args, {"--socks-proxy": endpoint(), "--host-mode": window_says("host-mode")})

    def test_the_catalogue_signs_them_as_one_block_in_its_policy(self):
        (checked,) = [r for r in load(CATALOGUE, ROOT) if r["package"] == "simplex-chat"]
        self.assertEqual(checked["args"], block(self.recipe["args"]))
        self.assertTrue(checked["args"].startswith(b"NLXARG01\x02"))


if __name__ == "__main__":
    unittest.main()
