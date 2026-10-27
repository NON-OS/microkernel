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

"""Render a cut of the NØNOS film.

  python3 tools/nonos_tutorial/film timelines/style_test.json --cut style \\
      --footage b434=<frames folder> --footage lanes=<folder> --out test.mp4 \\
      [--stills 1.2,5,9.5] [--music bed.mp3] [--narration wavs/ --narration-timeline n.json]

Frames are rendered in parallel and piped to ffmpeg (H.264, CRF 18,
yuv420p, +faststart), so no frame folder fills the disk; --frames DIR
keeps PNGs as well. --stills writes only the frames at those times."""

import argparse
import json
import os
import subprocess
import sys
import time
from multiprocessing import Pool
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from audio import encode_cmd  # noqa: E402
from timeline import Film, Footage, parse_narration  # noqa: E402

_SRC = None


def _init(timeline, cut, roots):
    global _SRC
    from render import FrameSource
    _SRC = FrameSource(Film.load(timeline), cut, Footage(roots))


def _frame(i):
    return i, _SRC.frame(i).tobytes()


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("timeline")
    ap.add_argument("--cut", default=None)
    ap.add_argument("--footage", action="append", default=[], help="alias=folder")
    ap.add_argument("--out", default="film.mp4")
    ap.add_argument("--stills", default=None, help="comma-separated times (s); write PNGs only")
    ap.add_argument("--frames", default=None, help="also keep every frame as PNG here")
    ap.add_argument("--workers", type=int, default=os.cpu_count() or 2)
    ap.add_argument("--music", default=None)
    ap.add_argument("--narration", default=None, help="folder of WAV lines")
    ap.add_argument("--narration-timeline", default=None, help="JSON: {lines: [{file, at}]}")
    ap.add_argument("--range", default=None, help="first:last frame, for a quick look")
    args = ap.parse_args(argv)

    timeline = str(Path(args.timeline).resolve())
    film = Film.load(timeline)
    cut = args.cut or next(iter(film.cuts))
    roots = Footage.parse_arg(args.footage)

    if args.stills:
        from render import FrameSource
        src = FrameSource(film, cut, Footage(roots))
        out = Path(args.out)
        out.mkdir(parents=True, exist_ok=True)
        for s in args.stills.split(","):
            t = float(s)
            i = int(round(t * film.fps))
            path = out / f"still_{t:06.2f}s.png"
            src.frame(i).save(path)
            print(path)
        return 0

    narration = None
    if args.narration:
        data = json.loads(Path(args.narration_timeline).read_text())
        narration = parse_narration(data, film.bpm, args.narration)

    from render import FrameSource
    probe = FrameSource(film, cut, Footage(roots))
    n = probe.n_frames
    first, last = 0, n - 1
    if args.range:
        a, b = args.range.split(":")
        first, last = int(a), min(int(b), n - 1)
    duration = (last - first + 1) / film.fps
    cmd = encode_cmd(args.out, film.fps, film.size, duration, args.music, narration, args.narration)
    frames_dir = Path(args.frames) if args.frames else None
    if frames_dir:
        frames_dir.mkdir(parents=True, exist_ok=True)
    enc = subprocess.Popen(cmd, stdin=subprocess.PIPE)
    t0 = time.time()
    with Pool(args.workers, initializer=_init, initargs=(timeline, cut, roots)) as pool:
        for k, (i, raw) in enumerate(pool.imap(_frame, range(first, last + 1), chunksize=2)):
            enc.stdin.write(raw)
            if frames_dir:
                from PIL import Image
                Image.frombytes("RGB", film.size, raw).save(frames_dir / f"{i:05d}.png", compress_level=1)
            if k % 30 == 0:
                print(f"frame {i}/{last}  {time.time() - t0:.0f}s", flush=True)
    enc.stdin.close()
    rc = enc.wait()
    print(f"{args.out}: {last - first + 1} frames in {time.time() - t0:.0f}s, ffmpeg {rc}")
    return rc


if __name__ == "__main__":
    sys.exit(main())
