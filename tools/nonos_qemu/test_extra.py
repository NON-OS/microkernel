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

"""The opt-in extra words on QEMU's command line; nothing here starts QEMU."""

import os
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import extra  # noqa: E402

class Extra(unittest.TestCase):
    def test_unset_adds_nothing(self):
        self.assertEqual(extra.words({}), [])

    def test_empty_adds_nothing(self):
        self.assertEqual(extra.words({extra.VAR: "  "}), [])

    def test_the_words_come_in_order(self):
        env = {extra.VAR: "-msg timestamp=on -trace pci_nvme_rw"}
        self.assertEqual(extra.words(env), ["-msg", "timestamp=on", "-trace", "pci_nvme_rw"])

    def test_a_quoted_word_stays_one_word(self):
        env = {extra.VAR: "-trace 'enable=pci_nvme_*'"}
        self.assertEqual(extra.words(env), ["-trace", "enable=pci_nvme_*"])

    def test_the_environment_is_read_when_none_is_given(self):
        os.environ[extra.VAR] = "-S"
        try:
            self.assertEqual(extra.words(), ["-S"])
        finally:
            del os.environ[extra.VAR]

    def test_the_launcher_puts_the_words_on_the_line(self):
        with open(os.path.join(HERE, "__main__.py")) as f:
            self.assertIn('"-no-reboot", *extra.words()]', f.read())


if __name__ == "__main__":
    unittest.main()
