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
"""The receipt is taken from the log only whole, verified, and a flipped copy
must be refused; with the tool stood in, so this runs with no image."""

import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from . import receipt
from .receipt_take import take

R, HEAD = bytes(range(200)), "[AGENTPROOF-RECEIPT] v=1 bytes=200 seal=0000000000000009"
END, KEPT = "[AGENTPROOF-RECEIPT] v=1 end", "[AGENTPROOF-RECEIPT] v=1 status=kept seal=9"
RULE = "[AGENTPROOF-RECEIPT] v=1 rule again=same other=-16 after-ack=-2"


def rows(b: bytes) -> list:
    return [f"[AGENTPROOF-RECEIPT] {b[i:i + 96].hex()}" for i in range(0, len(b), 96)]


def done(code_of):
    return lambda *a: subprocess.CompletedProcess(a, code_of(a), "nonos-receipt-trust 1\n", "x")


class Take(unittest.TestCase):
    def test_a_whole_receipt_is_taken(self):
        log = "\n".join(["noise", HEAD, *rows(R), "[AGENTPROOF-OUT] x", END, RULE, KEPT])
        self.assertEqual(take(log), (R, "kept seal=9", receipt.RULE))

    def test_a_cut_or_padded_or_unended_receipt_is_not(self):
        for bad in (rows(R)[:-1], rows(R) + rows(b"\0"), rows(R)[:1] + ["[AGENTPROOF-RECEIPT] 0"]):
            self.assertIsNone(take("\n".join([HEAD, *bad, END]))[0])
        for cut in ([HEAD, *rows(R)], [*rows(R), END]):
            self.assertIsNone(take("\n".join(cut))[0])


def checked(code_of, log=None) -> dict:
    """receipt.check over `log`, with the tool exiting code_of(its args)."""
    with tempfile.TemporaryDirectory() as d, mock.patch.object(receipt, "TOOL", Path(__file__)):
        with mock.patch.object(receipt, "tool", done(code_of)):
            return receipt.check(log or "\n".join([HEAD, *rows(R), END, RULE, KEPT]), Path(d))


class Check(unittest.TestCase):
    def test_verified_pinned_and_a_flip_refused(self):
        r = checked(lambda a: 1 if str(a[-2]).endswith("flipped") else 0)
        self.assertEqual((r["verified"], r["flip_refused"], r["why"]), (True, True, []))
        self.assertIn("no one vouched", r["trust"])

    def test_a_refused_receipt_or_an_accepted_flip_fails(self):
        self.assertTrue(checked(lambda a: 0 if a[0] == "pin" else 1)["why"])
        self.assertIn("one byte flipped", " ".join(checked(lambda a: 0)["why"]))
        self.assertTrue(checked(lambda a: 1)["why"], "pin refused")

    def test_no_receipt_fails_with_what_the_terminal_said(self):
        log = "[AGENTPROOF-RECEIPT] v=1 status=none why=no receipt job (-16)"
        self.assertIn("no whole receipt in the log: none why=no receipt job (-16)",
                      checked(lambda a: 0, log)["why"])
