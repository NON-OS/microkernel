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

"""What a clip of the recording gets on its way into the film, as one ffmpeg
filter graph from [0:v] to [v]:

- speed: played S times faster, with a tag that says so (x8), so a download
  does not drag and nobody takes it for real time;
- aspect: a screen the guest drew at another shape (the boot menu is
  1280x800, which the recording stretched to 1920x1080) put back to it,
  with bars;
- zoom: an eased move in onto a box of the frame and back out, for the line
  that matters, made 16:9 about the box's centre;
- lower: a lower-third, a panel with a title and a line of text that slides
  in, holds and fades.

Times are seconds from the clip's own start, after its speed."""

from . import brand

RAMP = 0.9      # seconds an eased zoom takes to go in, and to come out
FADE = 0.4      # seconds a lower-third takes to come up and to go
MARGIN = 64


def q(text):
    """A filter option value, quoted: an expression's commas and colons are
    the value's, not the graph's."""
    return "'" + str(text) + "'"


def ease(start, end, ramp=RAMP):
    """0 outside [start, end], 1 inside it but for a ramp at each end, and a
    smoothstep along each ramp: an expression in t."""
    p = f"min(clip((t-{start:g})/{ramp:g},0,1),clip(({end:g}-t)/{ramp:g},0,1))"
    return f"({p})*({p})*(3-2*({p}))"


def box169(box, width=brand.WIDTH, height=brand.HEIGHT):
    """The box (x, y, w, h) grown to 16:9 about its centre and kept inside
    the frame, so a zoom onto it neither stretches nor shows past an edge."""
    x, y, w, h = box
    w = max(w, h * width / height, 64)
    h = w * height / width
    cx, cy = x + box[2] / 2, y + box[3] / 2
    w, h = min(w, width), min(h, height)
    x = min(max(cx - w / 2, 0), width - w)
    y = min(max(cy - h / 2, 0), height - h)
    return round(x), round(y), round(w), round(h)


def zoom(zooms):
    """Eased zooms, none overlapping another: the window looked through
    moves from the whole frame to each box and back, and is scaled up to
    fill the frame (scale per frame, then a crop that follows)."""
    if not zooms:
        return []
    terms = []
    for z in zooms:
        bx, by, bw, _ = box169(z["box"])
        k = ease(z["at"], z["at"] + z["for"])
        terms.append((k, bx, by, bw))
    ww = f"({brand.WIDTH}" + "".join(f"+({k})*({bw}-{brand.WIDTH})" for k, _, _, bw in terms) + ")"
    wx = "(0" + "".join(f"+({k})*{bx}" for k, bx, _, _ in terms) + ")"
    wy = "(0" + "".join(f"+({k})*{by}" for k, _, by, _ in terms) + ")"
    scale = f"({brand.WIDTH}/{ww})"
    return [f"scale=w={q(f'trunc(iw*{scale}/2)*2')}:h={q(f'trunc(ih*{scale}/2)*2')}"
            ":eval=frame:flags=bicubic",
            f"crop={brand.WIDTH}:{brand.HEIGHT}"
            f":x={q(f'max(floor({wx}*{scale})-1,0)')}"
            f":y={q(f'max(floor({wy}*{scale})-1,0)')}"]


def aspect(ratio):
    """A frame the recording stretched to 16:9 put back to `ratio`, with
    bars at the sides."""
    if not ratio:
        return []
    width = round(brand.HEIGHT * float(ratio) / 2) * 2
    return [f"scale={width}:{brand.HEIGHT}",
            f"pad={brand.WIDTH}:{brand.HEIGHT}:(ow-iw)/2:0:color={brand.BACK}"]


def text(path, font, size, colour, x, y, extra=""):
    """drawtext of the file at `path`, as written."""
    return (f"drawtext=fontfile={q(font)}:textfile={q(path)}:expansion=none:"
            f"fontsize={size}:fontcolor={colour}:x={x}:y={y}{extra}")


def tag(label, path):
    """The speed tag, top right, in the mono face: x8."""
    if not label:
        return []
    return [text(path, brand.MONO, 44, brand.ACCENT, f"w-tw-{MARGIN}", 88,
                 f":box=1:boxcolor={brand.BACK}@0.72:boxborderw=16")]


def panel_size(title, line):
    """A lower-third panel wide enough for its two lines: Geist at 40 and 28
    pixels run about 23 and 15 pixels a character."""
    return max(len(title) * 23, len(line) * 15) + 2 * 36 + 8, 132


def lower(n, lt, length, title_path, line_path):
    """Lower-third `n`: a chain that draws its panel from nothing, faded and
    slid in at its time, and the overlay that lays it on [b{n}], to [b{n+1}]."""
    w, h = panel_size(lt["title"], lt.get("text", ""))
    a, b = lt["at"], lt["at"] + lt["for"]
    x, y = lt.get("pos", (MARGIN, 88))
    panel = (f"color=c={brand.PANEL}@0.9:s={w}x{h}:r={brand.FPS}:d={length:g},format=rgba,"
             f"drawbox=x=0:y=0:w=6:h=ih:color={brand.ACCENT}:t=fill,"
             + text(title_path, brand.TITLE, 40, brand.TEXT, 34, 22) + ","
             + text(line_path, brand.BODY, 28, brand.DIM, 34, 78) + ","
             f"fade=t=in:st={a:g}:d={FADE}:alpha=1,fade=t=out:st={b - FADE:g}:d={FADE}:alpha=1"
             f"[lt{n}]")
    slide = f"{x}-40*(1-clip((t-{a:g})/{FADE},0,1))"
    over = (f"[b{n}][lt{n}]overlay=x={q(slide)}:y={y}:enable={q(f'between(t,{a:g},{b:g})')}"
            f"[b{n + 1}]")
    return panel, over


def graph(clip, texts):
    """The clip's whole filter graph, [0:v] to [v]. `texts` names a file for
    each piece of drawn text: tag, and title{n} and line{n} for each
    lower-third."""
    chain = [f"setpts=(PTS-STARTPTS)/{clip['speed']:g}", f"fps={brand.FPS}"]
    chain += aspect(clip.get("aspect")) + zoom(clip.get("zoom", []))
    chain += tag(clip.get("tag"), texts.get("tag"))
    parts = ["[0:v]" + ",".join(chain) + "[b0]"]
    lowers = clip.get("lower", [])
    for n, lt in enumerate(lowers):
        parts += lower(n, lt, clip["length"], texts[f"title{n}"], texts[f"line{n}"])
    parts.append(f"[b{len(lowers)}]null[v]")
    return ";".join(parts)
