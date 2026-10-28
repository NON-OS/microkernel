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

"""The edit: which stretches of the recording make the film, in what order,
under which chapter, and what is drawn over each (film.json is the tour's).

A time is seconds, "H:MM:SS.s", or {"cue": TEXT, "shift": S}: the moment the
first caption starting with TEXT came up, from the drive's cues.txt, moved
by S seconds. So an edit names the captions it cuts at, and holds for a new
recording of the same steps."""

import json
from pathlib import Path


def cues(path):
    """(seconds, caption) for each line of a drive's cues.txt."""
    out = []
    for line in Path(path).read_text(encoding="utf-8").splitlines():
        stamp, _, text = line.partition("\t")
        if stamp:
            out.append((clock(stamp), text))
    return out


def clock(stamp):
    """"H:MM:SS.s", "M:SS" or "SS.s" as seconds."""
    secs = 0.0
    for part in str(stamp).split(":"):
        secs = secs * 60 + float(part)
    return secs


def seconds(spec, said):
    """A time of the edit as seconds into the recording."""
    if isinstance(spec, (int, float)):
        return float(spec)
    if isinstance(spec, str):
        return clock(spec)
    hits = [at for at, text in said if text.startswith(spec["cue"])]
    if not hits:
        raise ValueError(f"no caption starts with {spec['cue']!r} in cues.txt")
    return hits[0] + float(spec.get("shift", 0))


def within(effect, start, speed, said):
    """A zoom or lower-third with its `at` as seconds into the clip as
    played: a number already is; a cue is placed from the clip's start."""
    at = effect["at"]
    if isinstance(at, dict):
        at = (seconds(at, said) - start) / speed
    return dict(effect, at=float(at))


def resolve(clip, said):
    """A clip with its from and to as seconds, its length once played at
    its speed, and its effects' times in the clip."""
    start, end = seconds(clip["from"], said), seconds(clip["to"], said)
    if end <= start:
        raise ValueError(f"a clip ends before it starts: {clip['from']} to {clip['to']}")
    speed = float(clip.get("speed", 1))
    out = dict(clip, start=start, end=end, speed=speed, length=(end - start) / speed)
    for kind in ("zoom", "lower"):
        out[kind] = [within(e, start, speed, said) for e in clip.get(kind, [])]
    return out


def clips(chapter, said):
    """The chapter's clips resolved; in an optional chapter, a clip whose
    captions the recording lacks (its section was cut) is left out."""
    out = []
    for c in chapter["clips"]:
        try:
            out.append(resolve(c, said))
        except ValueError as e:
            if not chapter.get("optional"):
                raise
            print(f"post: {chapter['title']!r} goes without a clip: {e}", flush=True)
    return out


def chapters(film, said):
    """Each chapter with its clips resolved, and an optional one that has
    none left out."""
    kept = []
    for chapter in film["chapters"]:
        chapter["clips"] = clips(chapter, said)
        if chapter["clips"]:
            kept.append(chapter)
        else:
            print(f"post: leaving out {chapter['title']!r}: none of its sections was recorded", flush=True)
    return kept


def load(path, source=None, cue_file=None):
    """The edit at `path`, every clip's times resolved. Its source video and
    cues are the ones named (a recording's), or the edit's own, beside it."""
    path = Path(path)
    film = json.loads(path.read_text(encoding="utf-8"))
    here = path.parent
    film["source"] = str(Path(source).resolve() if source else (here / film["source"]).resolve())
    film["chapters"] = chapters(film, cues(cue_file or here / film.get("cues", "cues.txt")))
    return film
