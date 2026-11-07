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

"""The checks a round trip is judged by, without LibreOffice."""

import unittest

from . import check


class Checks(unittest.TestCase):
    def test_embedded_fonts_are_read_past_their_subset_tags(self):
        pdf = (b"<< /Type /FontDescriptor /FontName /BAAAAA+LiberationSerif /Flags 6 >>\n"
               b"<< /FontName /CAAAAA+NotoSans-Regular >>")
        self.assertEqual(check.embedded(pdf), {"liberationserif", "notosansregular"})
        self.assertEqual(check.fonts_embedded(pdf), [])

    def test_a_font_the_pdf_replaced_is_named(self):
        pdf = b"<< /FontName /BAAAAA+DejaVuSans >> << /FontName /LiberationSerif >>"
        self.assertEqual(check.fonts_embedded(pdf), ["Noto Sans"])
        self.assertEqual(check.fonts_embedded(b""), list(check.FONTS))

    def test_text_is_kept_only_whole_and_in_order(self):
        want = [check.ORIGINAL, check.EDIT]
        self.assertTrue(check.text_kept([check.ORIGINAL, "", check.EDIT], want))
        self.assertFalse(check.text_kept([check.EDIT, check.ORIGINAL], want))
        self.assertFalse(check.text_kept([check.ORIGINAL], want))
        self.assertFalse(check.text_kept([check.ORIGINAL, check.EDIT[:-1]], want))
