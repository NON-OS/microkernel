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
"""Only licences that permit redistribution, however the expression joins
them. Run from the repository root:

    python3 -m unittest discover -s tools/nonos_market_catalogue -t tools
"""

import unittest

from .licences import names, redistributable


class Licences(unittest.TestCase):
    def test_the_index_s_spellings_are_read(self):
        for expr in ("MIT", "MIT Apache-2.0", "MIT OR Unlicense", "BSD-3-Clause AND GPL-2.0-or-later",
                     "GPL-3.0-or-later WITH OpenSSL-Exception", "(MIT OR Apache-2.0) AND Zlib",
                     "GPL-2.0-only WITH GCC-exception-2.0", "Vim", "blessing"):
            self.assertTrue(redistributable(expr), expr)

    def test_one_unknown_name_refuses_the_whole_expression(self):
        for expr in ("MIT OR Proprietary", "custom", "BUSL-1.1", "SSPL-1.0", "MIT AND custom:foo",
                     "Elastic-2.0", "CC-BY-NC-4.0", "Commons-Clause"):
            self.assertFalse(redistributable(expr), expr)

    def test_an_empty_or_joins_only_expression_is_refused(self):
        for expr in ("", "   ", "AND OR", "()"):
            self.assertFalse(redistributable(expr), repr(expr))

    def test_an_exception_alone_is_no_licence(self):
        self.assertFalse(redistributable("OpenSSL-Exception"))
        self.assertFalse(redistributable("GCC-exception-2.0 AND Classpath-exception-2.0"))
        self.assertEqual(names("GPL-2.0-only WITH OpenSSL-Exception"),
                         ["GPL-2.0-only", "OpenSSL-Exception"])


if __name__ == "__main__":
    unittest.main()
