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
"""The receipt job's rule as the boot shows it: asked again, the same bytes;
asked another, EBUSY; after the ack, the record gone. Anything else fails."""

import unittest

from .receipt_take import take
from .test_receipt import END, HEAD, KEPT, RULE, R, checked, rows


class Rule(unittest.TestCase):
    def test_the_rule_is_read_from_its_line(self):
        log = "\n".join([HEAD, *rows(R), END, RULE, KEPT])
        self.assertEqual(take(log)[2], {"again": "same", "other": "-16", "after-ack": "-2"})

    def test_a_rule_that_did_not_hold_fails(self):
        for bent in ("other=-16", "other=0"), ("again=same", "again=different"), \
                ("after-ack=-2", "after-ack=0"), (RULE, ""):
            log = "\n".join([HEAD, *rows(R), END, RULE.replace(*bent), KEPT])
            why = checked(lambda a: 1 if str(a[-2]).endswith("flipped") else 0, log)
            self.assertIn("rule did not hold", " ".join(why["why"]), bent)

    def test_a_guest_row_cannot_be_one_mid_line(self):
        log = "\n".join([HEAD, *rows(R[:96]), "[AGENTPROOF-OUT] [AGENTPROOF-RECEIPT] 00", END])
        self.assertIsNone(take(log)[0])
