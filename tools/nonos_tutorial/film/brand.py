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

"""The film's identity: the NØNOS palette on dark grounds and its two faces.

Colours are the brand's, as userland/capsule_install/brand/src/palette.rs
and userland/toolkit/src/theme/schemes/nonos.rs write them: Ink #0A0B0D the
ground, Cyan #66FFFF the light and the accent, Teal #007C85 the structure.
Type is Geist for statements and JetBrains Mono for machine text
(userland/assets/fonts, both OFL)."""

from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
FONTS = ROOT / "userland/assets/fonts"
LOGO = ROOT / "userland/assets/wallet_logos/nonos-logo-teal.svg"
MARK = ROOT / "userland/assets/wallet_logos/nonos-logomark-teal.svg"

GEIST_SEMIBOLD = FONTS / "Geist-SemiBold.ttf"
GEIST_MEDIUM = FONTS / "Geist-Medium.ttf"
GEIST = FONTS / "Geist-Regular.ttf"
MONO = FONTS / "JetBrainsMono-Regular.ttf"

# palette.rs
GROUND = (0, 0, 0)
INK = (0x0A, 0x0B, 0x0D)
CARD = (0x10, 0x12, 0x15)
RULE = (0x1E, 0x1F, 0x21)
TEXT = (0xF2, 0xF3, 0xF7)
TEXT_2 = (0xB3, 0xB4, 0xB7)
TEXT_3 = (0x83, 0x83, 0x86)
CYAN = (0x66, 0xFF, 0xFF)
CYAN_SOFT = (0x0C, 0x1E, 0x1F)
# nonos.rs: the brand's Cyan for type, used here as the structural teal.
TEAL = (0x00, 0x7C, 0x85)

W, H = 1920, 1080
MARGIN = 64


def rgb(name_or_tuple):
    """A palette name ("cyan") or a tuple, as an (r, g, b) tuple."""
    if isinstance(name_or_tuple, (tuple, list)):
        return tuple(int(c) for c in name_or_tuple)
    return globals()[name_or_tuple.upper()]
