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

"""A drive filmed: nonos-drive --record OUT.mp4.

Xvfb gives QEMU an X display of its own, 1920x1080, that nothing else draws
on. QEMU shows the guest there full screen (nonos-qemu --shown), its window
held over the whole display (fill.py, since Xvfb has no window manager to
make it full screen). ffmpeg films
that display from before power-on, 30 frames a second, and draws over each
frame the caption a `say:` step sets (caption.py), in the brand's font.

At the end ffmpeg is told to stop, `q` on its stdin, and waited for, so the
file is whole and its index is at the front (+faststart): it plays from the
start while it downloads. Then QEMU stops, then Xvfb. A drive filmed puts
ffmpeg.log and xvfb.log beside serial.log."""

import contextlib
import os
import subprocess
import time
from dataclasses import dataclass
from pathlib import Path

from . import caption, fill

WIDTH, HEIGHT = 1920, 1080
SIZE = f"{WIDTH}x{HEIGHT}"
RATE = "30"
FONT = "userland/assets/fonts/Geist-SemiBold.ttf"
# White on a translucent dark box, centred, its middle 153 pixels above the
# bottom edge: under a window centred in the work area, as the Terminal is,
# and above the dock, whose tiles sit near y=1020. One line of about 85
# characters fits across the screen.
STYLE = ("fontsize=40:fontcolor=white:box=1:boxcolor=black@0.62:boxborderw=22:"
         "x=(w-text_w)/2:y=h-153-text_h/2")
FIRST_DISPLAY = 99
DISPLAYS = 50
CUES = "cues.txt"


def clock(secs):
    """Seconds as the video's H:MM:SS.s, as ffmpeg's -ss takes them."""
    h, rest = divmod(max(0.0, secs), 3600)
    m, s = divmod(rest, 60)
    return f"{int(h)}:{int(m):02d}:{s:04.1f}"


@dataclass
class Film:
    display: int
    caption: Path
    out: Path
    xvfb: subprocess.Popen
    ffmpeg: subprocess.Popen
    status: int | None = None
    started: float = 0.0
    filler: fill.Filler | None = None

    def cue(self, text):
        """Note `text` in cues.txt beside the caption, at its time in the
        video, where a section can be cut; that time, as clock() writes it."""
        at = clock(time.time() - self.started)
        with open(self.caption.with_name(CUES), "a", encoding="utf-8") as f:
            f.write(f"{at}\t{text}\n")
        return at


def quoted(path):
    """A path as a filtergraph option value: quoted, so a colon or a comma in
    it is not read as the next option or the next filter."""
    text = str(path)
    if "'" in text:
        raise ValueError(f"nonos-drive: a recording cannot name a path with a quote in it: {text}")
    return f"'{text}'"


def drawtext(font, caption_path):
    """The caption over each frame, read again from its file at every frame,
    printed as written (no %{...} expansion)."""
    return (f"drawtext=fontfile={quoted(font)}:textfile={quoted(caption_path)}:"
            f"reload=1:expansion=none:{STYLE}")


def xvfb_cmd(display):
    return ["Xvfb", f":{display}", "-screen", "0", f"{SIZE}x24", "-nolisten", "tcp"]


def ffmpeg_cmd(display, font, caption_path, out):
    return ["ffmpeg", "-hide_banner", "-nostats", "-y",
            "-f", "x11grab", "-video_size", SIZE, "-framerate", RATE, "-draw_mouse", "0",
            "-i", f":{display}", "-vf", drawtext(font, caption_path),
            "-c:v", "libx264", "-preset", "veryfast", "-crf", "20", "-pix_fmt", "yuv420p",
            "-movflags", "+faststart", str(out)]


def socket_of(display):
    return Path(f"/tmp/.X11-unix/X{display}")


def taken(display):
    return socket_of(display).exists() or Path(f"/tmp/.X{display}-lock").exists()


def xvfb(log, wait=30.0):
    """Xvfb on the first display from :99 that no other X server holds, once
    its socket is there; the display and the process. A display another
    server took first (its Xvfb leaves at once) moves on to the next."""
    for display in range(FIRST_DISPLAY, FIRST_DISPLAY + DISPLAYS):
        if taken(display):
            continue
        proc = subprocess.Popen(xvfb_cmd(display), stdout=log, stderr=subprocess.STDOUT,
                                stdin=subprocess.DEVNULL)
        end = time.time() + wait
        while proc.poll() is None and not socket_of(display).exists() and time.time() < end:
            time.sleep(0.1)
        if proc.poll() is None and socket_of(display).exists():
            return display, proc
        proc.terminate()
        proc.wait()
    raise RuntimeError(f"nonos-drive: no X display from :{FIRST_DISPLAY} would start (xvfb.log)")


def ffmpeg(display, font, caption_path, out, log, settle=2.0):
    """ffmpeg filming `display`, with a pipe to its stdin for the `q` that
    ends it; a run whose ffmpeg left at once (no x11grab, no font) stops
    here, not an hour later with no video."""
    proc = subprocess.Popen(ffmpeg_cmd(display, font, caption_path, out), stdin=subprocess.PIPE,
                            stdout=log, stderr=subprocess.STDOUT)
    time.sleep(settle)
    if proc.poll() is not None:
        raise RuntimeError(f"nonos-drive: ffmpeg left at once (exit {proc.returncode}): see ffmpeg.log")
    return proc


def start(root, out, workdir):
    """Xvfb, an empty caption, then ffmpeg filming: a Film."""
    font = Path(root) / FONT
    if not font.is_file():
        raise RuntimeError(f"nonos-drive: no caption font at {font}")
    text = Path(workdir).resolve() / caption.NAME
    caption.say(text, "")
    text.with_name(CUES).write_text("")
    display, x = xvfb(open(Path(workdir) / "xvfb.log", "w"))
    filler = fill.Filler(display, WIDTH, HEIGHT).start()
    started = time.time()
    try:
        f = ffmpeg(display, font, text, out, open(Path(workdir) / "ffmpeg.log", "w"))
    except BaseException:
        filler.stop()
        stop(x)
        raise
    print(f"nonos-drive: filming X display :{display} to {out}", flush=True)
    return Film(display, text, Path(out), x, f, started=started, filler=filler)


def stop(proc, wait=10.0):
    """End a process that is told to by SIGTERM, and kill one that is not."""
    if proc.poll() is None:
        proc.terminate()
    try:
        proc.wait(timeout=wait)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait()


def cut(film, wait=300.0):
    """Stop filming: `q` on ffmpeg's stdin, then wait while it writes the
    file's end and moves its index to the front. True when ffmpeg finished
    the file; with no film, True. A second cut changes nothing."""
    if film is None:
        return True
    if film.status is None:
        with contextlib.suppress(OSError):
            film.ffmpeg.stdin.write(b"q")
            film.ffmpeg.stdin.flush()
        with contextlib.suppress(OSError):
            film.ffmpeg.stdin.close()
        try:
            film.status = film.ffmpeg.wait(timeout=wait)
        except subprocess.TimeoutExpired:
            stop(film.ffmpeg)
            film.status = film.ffmpeg.returncode
        print(f"nonos-drive: recording {'written' if film.status == 0 else 'NOT whole'}, "
              f"{film.out} (ffmpeg exit {film.status})", flush=True)
    return film.status == 0


@contextlib.contextmanager
def filming(root, out, workdir):
    """A Film for the drive inside, or None with no `out`. Leaving it cuts
    the film if the drive did not (a step that raised) and stops Xvfb, the
    last of the three: the drive itself stops QEMU between the two."""
    film = start(root, out, workdir) if out else None
    try:
        yield film
    finally:
        if film is not None:
            cut(film)
            if film.filler is not None:
                film.filler.stop()
            stop(film.xvfb)


def display_env(display, env=None):
    """The environment QEMU's window opens on `display` with: GTK on X even
    on a host whose own desktop is Wayland, and no accessibility bus to wait
    for."""
    env = dict(os.environ if env is None else env)
    env.pop("WAYLAND_DISPLAY", None)
    env.update(DISPLAY=f":{display}", GDK_BACKEND="x11", NO_AT_BRIDGE="1")
    return env
