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

"""The caption a recording draws: one file that ffmpeg's drawtext reads again
at every frame (reload=1). A `say:` step writes the new text to a file beside
it and renames that over the old one, so drawtext reads the old caption or
the new one, each whole, and never half a line. An empty text clears it."""

import os
from pathlib import Path

NAME = "caption.txt"


def say(path, text):
    """Make `text` the caption at `path`, in one rename. drawtext reads
    UTF-8, whatever this machine's locale: NØNOS has an Ø in it."""
    path = Path(path)
    tmp = path.with_name(path.name + ".new")
    tmp.write_text(text, encoding="utf-8")
    os.replace(tmp, path)
