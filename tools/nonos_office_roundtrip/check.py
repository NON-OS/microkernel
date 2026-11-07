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

"""What a round trip must show, apart from LibreOffice: the text that went
in is the text that comes back, the edit is there after a reopen, and the
fonts the document names are the ones the PDF embeds. Pure, so the suite
holds it without LibreOffice."""

import re

ORIGINAL = "NONOS round trip: the quick brown fox jumps over the lazy dog."
EDIT = "Edited after reopening: 0123456789 äöü ß € 中文."
FONTS = ("Liberation Serif", "Noto Sans")
# A PDF names an embedded font as /FontName /ABCDEF+Liberation-Serif or the
# like: a subset tag, then the name with its spaces dropped or dashed.
FONT_NAME = re.compile(rb"/FontName\s*/(?:[A-Z]{6}\+)?([A-Za-z0-9-]+)")


def squash(name: str) -> str:
    return re.sub(r"[^a-z0-9]", "", name.lower())


def embedded(pdf: bytes) -> set:
    """The fonts a PDF embeds, squashed: no subset tag, case, space or dash."""
    return {squash(m.decode("ascii")) for m in FONT_NAME.findall(pdf)}


def fonts_embedded(pdf: bytes, fonts=FONTS) -> list:
    """The named fonts the PDF does not embed; empty when every one is."""
    have = embedded(pdf)
    return [f for f in fonts if not any(h.startswith(squash(f)) for h in have)]


def text_kept(paragraphs: list, want: list) -> bool:
    """Every wanted paragraph is there, in order, whole."""
    got = [p for p in paragraphs if p.strip()]
    return got[:len(want)] == want
