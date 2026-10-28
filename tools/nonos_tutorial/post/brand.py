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

"""What the film is set in: the desktop's own colours, from
userland/capsule_desktop_shell/src/render/palette.rs, and its two faces,
Geist and JetBrains Mono (userland/assets/fonts; docs/using/desktop.md,
Type). The frame is the recording's, 1920x1080, and the cut runs at 60
frames a second so a zoom or a fade moves smoothly."""

from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
FONTS = ROOT / "userland/assets/fonts"
TITLE = FONTS / "Geist-SemiBold.ttf"
MEDIUM = FONTS / "Geist-Medium.ttf"
BODY = FONTS / "Geist-Regular.ttf"
MONO = FONTS / "JetBrainsMono-Regular.ttf"

# palette.rs, as ffmpeg writes a colour.
BACK = "0x04080C"      # BAR: the menu bar's near-black
CARD = "0x0A1218"      # between BAR and PANEL, under a card's vignette
PANEL = "0x101A22"     # PANEL_SOLID
ACCENT = "0x22C3F0"    # ACCENT
TEXT = "0xE6F1F6"      # TEXT
DIM = "0x7A8D99"       # TEXT_DIM

WIDTH, HEIGHT, FPS = 1920, 1080, 60
