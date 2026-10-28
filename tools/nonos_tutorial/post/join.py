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

"""The pieces made one film: each chapter's card and clips run on with cuts,
and the chapters, the title and the end cross-fade into each other (xfade).
A cross-fade of F seconds overlaps two pieces by F, so each part after the
first starts F seconds before the one ahead of it ends, and that start is
where its chapter mark goes, in the film's metadata and in a list to paste
under the video."""

from . import brand


def starts(lengths, fade):
    """Where each part begins in the joined film: each overlaps the one
    before it by `fade` seconds."""
    out, at = [], 0.0
    for n, length in enumerate(lengths):
        out.append(at)
        at += length - (fade if n + 1 < len(lengths) else 0)
    return out


def total(lengths, fade):
    return sum(lengths) - fade * (len(lengths) - 1)


def graph(groups, lengths, fade, transition="fade"):
    """The filter graph: `groups` lists, for each part, the input numbers of
    its pieces in order; `lengths` is each part's length. Ends in [film]."""
    parts = []
    for n, inputs in enumerate(groups):
        joined = "".join(f"[{i}:v]" for i in inputs)
        parts.append(f"{joined}concat=n={len(inputs)}:v=1:a=0,settb=AVTB,fps={brand.FPS}[p{n}]")
    if len(groups) == 1:
        parts.append("[p0]null[film]")
        return ";".join(parts)
    at = starts(lengths, fade)
    last = "p0"
    for n in range(1, len(groups)):
        out = "film" if n + 1 == len(groups) else f"x{n}"
        parts.append(f"[{last}][p{n}]xfade=transition={transition}:duration={fade:g}:offset={at[n]:.3f}[{out}]")
        last = out
    return ";".join(parts)


def clock(secs, millis=False):
    """Seconds as H:MM:SS, or M:SS under an hour, as a chapter list writes
    them."""
    whole = int(secs)
    h, rest = divmod(whole, 3600)
    m, s = divmod(rest, 60)
    text = f"{h}:{m:02d}:{s:02d}" if h else f"{m}:{s:02d}"
    return text + (f".{int((secs - whole) * 1000):03d}" if millis else "")


def chapter_list(titles, at):
    """The list a video page shows as chapters: the first at 0:00."""
    return "".join(f"{clock(t)} {title}\n" for title, t in zip(titles, at))


def ffmetadata(titles, at, end):
    """The same chapters as ffmpeg's metadata file, muxed into each cut."""
    lines = [";FFMETADATA1"]
    ends = at[1:] + [end]
    for title, a, b in zip(titles, at, ends):
        lines += ["[CHAPTER]", "TIMEBASE=1/1000", f"START={int(a * 1000)}", f"END={int(b * 1000)}",
                  "title=" + title.replace("=", "\\=").replace(";", "\\;").replace("#", "\\#")]
    return "\n".join(lines) + "\n"
