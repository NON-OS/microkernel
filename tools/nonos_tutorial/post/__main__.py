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

"""Cut the film from a recording.

    python3 tools/nonos_tutorial/post EDIT.json --out DIR \\
        [--source REC.mp4] [--cues cues.txt] [--preset slow] [--uhd]

EDIT.json is the edit (film.json beside this file is the tour's); --source
and --cues name the recording and its cues.txt when they are not where the
edit says. It writes, in DIR:

    pieces/              each card and clip, drawn and encoded once
    master-1080p60.mp4   the film, every chapter marked
    web-1080p30.mp4      the same at 30 frames a second, smaller, for the web
    master-2160p60.mp4   with --uhd: the master scaled to 3840x2160
    chapters.txt         the chapter list to paste under the video
    chapters.ffmeta      the same, as ffmpeg's metadata file
"""

import argparse
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))

from nonos_tutorial.post import brand, cards, edit, effects, join  # noqa: E402

PIECE = ["-r", str(brand.FPS), "-c:v", "libx264", "-preset", "veryfast", "-crf", "12",
         "-pix_fmt", "yuv420p", "-an"]
QUIET = ["ffmpeg", "-hide_banner", "-loglevel", "error", "-y"]


def run(cmd, what):
    print(f"post: {what}", flush=True)
    subprocess.run(cmd, check=True)


def length(path):
    out = subprocess.run(["ffprobe", "-v", "error", "-show_entries", "format=duration",
                          "-of", "csv=p=0", str(path)], check=True, capture_output=True, text=True)
    return float(out.stdout.strip())


class Pieces:
    """Where pieces and the files of their words go, numbered in order."""

    def __init__(self, out):
        self.dir = Path(out) / "pieces"
        self.dir.mkdir(parents=True, exist_ok=True)
        self.n = 0

    def next(self):
        self.n += 1
        return self.dir / f"{self.n:03d}.mp4"

    def words(self, name, text):
        path = (self.dir / f"{self.n:03d}-{name}.txt").resolve()
        path.write_text(text, encoding="utf-8")
        return path


def card(p, spec, label, seconds, layout=cards.CHAPTER_CARD):
    out = p.next()
    texts = {k: p.words(k, v) for k, v in (("label", label), ("title", spec.get("title")),
                                            ("line", spec.get("line"))) if v}
    run(cards.command(texts, float(seconds), out, layout, PIECE), f"card {out.name}: {spec.get('title')}")
    return out


def clip(p, source, c):
    out = p.next()
    texts = {}
    if c.get("tag"):
        texts["tag"] = p.words("tag", c["tag"])
    for n, lt in enumerate(c.get("lower", [])):
        texts[f"title{n}"] = p.words(f"title{n}", lt["title"])
        texts[f"line{n}"] = p.words(f"line{n}", lt.get("text", ""))
    cmd = [*QUIET, "-ss", f"{c['start']:.3f}", "-t", f"{c['end'] - c['start']:.3f}", "-i", source,
           "-filter_complex", effects.graph(c, texts), "-map", "[v]", *PIECE, str(out)]
    run(cmd, f"clip {out.name}: {c['start']:.1f}s to {c['end']:.1f}s at x{c['speed']:g}")
    return out


def parts(film, out):
    """Every piece drawn: [(chapter title or None, [piece, ...]), ...], the
    title first and the end last."""
    p = Pieces(out)
    title, end = film["title"], film.get("end")
    made = [(title["title"], [card(p, title, title.get("label"), title.get("seconds", 4),
                                   cards.TITLE_CARD)])]
    for n, chapter in enumerate(film["chapters"], 1):
        seconds = chapter.get("card", 3)
        pieces = [card(p, chapter, f"CHAPTER {n:02d}", seconds)] if seconds else []
        pieces += [clip(p, film["source"], c) for c in chapter["clips"]]
        made.append((chapter["title"], pieces))
    if end:
        made.append((None, [card(p, end, end.get("label"), end.get("seconds", 4), cards.TITLE_CARD)]))
    return made


def master(made, lengths, out, fade, transition, preset, meta):
    """The command that joins every part into the 1080p60 master, and its
    path."""
    inputs, groups, n = [], [], 0
    for _, pieces in made:
        groups.append(list(range(n, n + len(pieces))))
        for x in pieces:
            inputs += ["-i", str(x)]
        n += len(pieces)
    path = out / "master-1080p60.mp4"
    cmd = [*QUIET, *inputs, "-i", str(meta), "-filter_complex",
           join.graph(groups, lengths, fade, transition), "-map", "[film]",
           "-map_metadata", str(n), "-map_chapters", str(n), "-r", str(brand.FPS),
           "-c:v", "libx264", "-preset", preset, "-crf", "16", "-pix_fmt", "yuv420p",
           "-movflags", "+faststart", str(path)]
    return cmd, path


def marks(made, lengths, fade, out):
    """chapters.txt and chapters.ffmeta: the end card's time is its last
    chapter's."""
    at = join.starts(lengths, fade)
    titles = [t for t, _ in made if t]
    starts = [a for (t, _), a in zip(made, at) if t]
    (out / "chapters.txt").write_text(join.chapter_list(titles, starts), encoding="utf-8")
    meta = out / "chapters.ffmeta"
    meta.write_text(join.ffmetadata(titles, starts, join.total(lengths, fade)), encoding="utf-8")
    return meta


def derived(master_path, out, uhd):
    web = out / "web-1080p30.mp4"
    run([*QUIET, "-i", str(master_path), "-map", "0:v", "-map_chapters", "0", "-vf", "fps=30",
         "-c:v", "libx264", "-preset", "medium", "-crf", "21", "-pix_fmt", "yuv420p",
         "-movflags", "+faststart", str(web)], f"web cut {web.name}")
    if uhd:
        big = out / "master-2160p60.mp4"
        run([*QUIET, "-i", str(master_path), "-map", "0:v", "-map_chapters", "0",
             "-vf", "scale=3840:2160:flags=lanczos", "-c:v", "libx264", "-preset", "slow",
             "-crf", "16", "-pix_fmt", "yuv420p", "-movflags", "+faststart", str(big)],
            f"UHD master {big.name}")


def main():
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0],
                                 formatter_class=argparse.RawDescriptionHelpFormatter, epilog=__doc__)
    ap.add_argument("edit", type=Path)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--source", type=Path, help="the recording, when not where the edit says")
    ap.add_argument("--cues", type=Path, help="its cues.txt, when not where the edit says")
    ap.add_argument("--preset", default="slow", help="x264 preset of the master (default slow)")
    ap.add_argument("--uhd", action="store_true", help="also a 3840x2160 master, scaled up")
    a = ap.parse_args()
    film = edit.load(a.edit, a.source, a.cues)
    a.out.mkdir(parents=True, exist_ok=True)
    fade, transition = float(film.get("fade", 0.8)), film.get("transition", "fade")
    made = parts(film, a.out)
    lengths = [sum(length(x) for x in pieces) for _, pieces in made]
    meta = marks(made, lengths, fade, a.out)
    cmd, path = master(made, lengths, a.out, fade, transition, a.preset, meta)
    run(cmd, f"master {path.name}, {join.clock(join.total(lengths, fade))} long")
    derived(path, a.out, a.uhd)
    print(f"post: done, {a.out}", flush=True)


if __name__ == "__main__":
    main()
