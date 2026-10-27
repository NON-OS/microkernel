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

"""A film is shots, and cuts that pick from them.

A timeline file (JSON) names the tempo, the shots and the cuts. A chapter
is a file of shots (`"include"` pulls them in), and the full film and the
60-90 s cut for X are two lists of shot ids over the same shots, so both
come from one edit. Times are seconds (1.5), beats ("2b"), bars ("2bar")
or a timecode ("0:03.75"); a shot's length is cut on the beat it names.

Footage is named by alias ("b434:003599.png"), and `--footage
b434=/some/folder` says where the alias lives, so any folder of frames
works. A folder with times.txt (frame number, UNIX time) gives each frame
its UTC time for the timecode."""

import datetime
import json
import math
import re
import wave
from pathlib import Path

_TC = re.compile(r"^(?:(\d+):)?(\d+):(\d+(?:\.\d+)?)$")
_UNIT = re.compile(r"^(-?\d+(?:\.\d+)?)\s*(b|beat|beats|bar|bars|s)?$")


class TimelineError(ValueError):
    pass


def parse_time(value, bpm):
    """Seconds from 1.5, "1.5s", "2b", "2bar" or "1:02.5"."""
    if isinstance(value, bool):
        raise TimelineError(f"not a time: {value!r}")
    if isinstance(value, (int, float)):
        return float(value)
    text = str(value).strip()
    m = _TC.match(text)
    if m:
        h = int(m.group(1)) if m.group(1) else 0
        return h * 3600 + int(m.group(2)) * 60 + float(m.group(3))
    m = _UNIT.match(text)
    if not m:
        raise TimelineError(f"not a time: {value!r} (seconds, '2b', '2bar' or 'm:ss.s')")
    n, unit = float(m.group(1)), m.group(2) or "s"
    beat = 60.0 / bpm
    if unit in ("b", "beat", "beats"):
        return n * beat
    if unit in ("bar", "bars"):
        return n * 4 * beat
    return n


def frame_of(t, fps):
    """The frame a time falls on: a cut at t is shown from this frame."""
    return int(math.floor(t * fps + 0.5))


class Footage:
    """Aliases for folders of frames."""

    def __init__(self, roots):
        self.roots = {k: Path(v) for k, v in roots.items()}
        self._times = {}

    @staticmethod
    def parse_arg(items):
        roots = {}
        for item in items or []:
            if "=" not in item:
                raise TimelineError(f"--footage wants alias=folder, not {item!r}")
            k, v = item.split("=", 1)
            roots[k.strip()] = v.strip()
        return roots

    def path(self, ref):
        if ":" in ref and not ref.startswith("/"):
            alias, name = ref.split(":", 1)
            if alias not in self.roots:
                raise TimelineError(f"footage alias {alias!r} is not given (--footage {alias}=...)")
            return self.roots[alias] / name
        return Path(ref)

    def frames(self, ref):
        """Every frame in an alias's folder, sorted by name."""
        folder = self.path(ref)
        files = sorted(p for p in folder.iterdir() if p.suffix.lower() in (".png", ".jpg"))
        if not files:
            raise TimelineError(f"no frames in {folder}")
        return files

    def utc(self, ref):
        """The UTC time of a frame, from times.txt beside it, or None."""
        p = self.path(ref)
        folder = p.parent
        if folder not in self._times:
            table = {}
            tf = folder / "times.txt"
            if tf.exists():
                for line in tf.read_text().splitlines():
                    parts = line.split()
                    if len(parts) == 2:
                        table[int(parts[0])] = float(parts[1])
            self._times[folder] = table
        try:
            n = int(p.stem)
        except ValueError:
            return None
        t = self._times[folder].get(n)
        if t is None:
            return None
        return datetime.datetime.fromtimestamp(t, datetime.timezone.utc)


def format_utc(dt):
    return dt.strftime("%Y-%m-%d %H:%M:%S UTC")


class Shot:
    def __init__(self, spec, bpm):
        if "id" not in spec:
            raise TimelineError(f"a shot has no id: {json.dumps(spec)[:80]}")
        self.id = spec["id"]
        self.spec = spec
        self.length = parse_time(spec.get("length", "2bar"), bpm)
        if self.length <= 0:
            raise TimelineError(f"shot {self.id}: length must be above 0")
        self.start = 0.0

    @property
    def end(self):
        return self.start + self.length


class Film:
    def __init__(self, data, base=Path(".")):
        self.title = data.get("title", "")
        self.fps = int(data.get("fps", 30))
        self.bpm = float(data.get("bpm", 128))
        self.size = tuple(data.get("size", (1920, 1080)))
        specs = list(data.get("shots", []))
        for inc in data.get("include", []):
            specs.extend(json.loads((base / inc).read_text()).get("shots", []))
        self.shots = {}
        for s in specs:
            shot = Shot(s, self.bpm)
            if shot.id in self.shots:
                raise TimelineError(f"two shots are called {shot.id!r}")
            self.shots[shot.id] = shot
        self.cuts = data.get("cuts") or {"all": [s["id"] for s in specs]}
        for name, ids in self.cuts.items():
            for i in ids:
                if i not in self.shots:
                    raise TimelineError(f"cut {name!r} names a shot that is not there: {i!r}")

    def beat(self):
        return 60.0 / self.bpm

    def time(self, value):
        return parse_time(value, self.bpm)

    def cut(self, name):
        """The cut's shots, each a fresh Shot placed end to end."""
        if name not in self.cuts:
            raise TimelineError(f"no cut {name!r}; there are {sorted(self.cuts)}")
        out, t = [], 0.0
        for i in self.cuts[name]:
            shot = Shot(self.shots[i].spec, self.bpm)
            shot.start = t
            t += shot.length
            out.append(shot)
        return out

    @staticmethod
    def load(path):
        path = Path(path)
        return Film(json.loads(path.read_text()), base=path.parent)


def shot_at(shots, t):
    """The shot showing at time t, and t within it."""
    for s in shots:
        if s.start <= t < s.end:
            return s, t - s.start
    last = shots[-1]
    return last, min(t - last.start, last.length - 1e-6)


def parse_narration(data, bpm=128.0, folder=None):
    """A narration timeline: {"lines": [{"file": "01.wav", "at": "0:03.75"}]}.
    Returns [(file, start seconds, duration or None)], sorted by start.
    With `folder`, each WAV is opened for its length and lines that would
    overlap are refused."""
    lines = data.get("lines") if isinstance(data, dict) else data
    if not isinstance(lines, list) or not lines:
        raise TimelineError("a narration timeline needs a non-empty list of lines")
    out = []
    for i, line in enumerate(lines):
        if not isinstance(line, dict) or "file" not in line or "at" not in line:
            raise TimelineError(f"narration line {i + 1} needs 'file' and 'at'")
        at = parse_time(line["at"], bpm)
        if at < 0:
            raise TimelineError(f"narration line {i + 1} starts before the film")
        dur = None
        if folder is not None:
            with wave.open(str(Path(folder) / line["file"]), "rb") as w:
                dur = w.getnframes() / float(w.getframerate())
        out.append((line["file"], at, dur))
    out.sort(key=lambda x: x[1])
    for (fa, a, da), (fb, b, _) in zip(out, out[1:]):
        if da is not None and a + da > b + 1e-3:
            raise TimelineError(f"narration {fa} runs into {fb}")
    return out
