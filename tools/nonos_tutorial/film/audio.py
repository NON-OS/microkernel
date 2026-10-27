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

"""The sound hooks: a music bed, ducked under the narration and faded over
the last two seconds, and narration lines placed at their times. This only
builds ffmpeg's arguments; nothing here fetches or makes sound."""

from pathlib import Path


def audio_args(duration, music=None, narration=None, narration_dir=None, first_input=1,
               fade=2.0, music_gain=0.8):
    """ffmpeg input arguments, a filter graph and map arguments for the
    sound, given the film's `duration`, a `music` file and narration lines
    [(file, start, _), ...] in `narration_dir`. Input 0 is the video.
    Returns ([], None, []) when there is no sound."""
    inputs, chains, idx = [], [], first_input
    mus = narr = None
    if music:
        inputs += ["-i", str(music)]
        chains.append(f"[{idx}:a]atrim=0:{duration:.3f},asetpts=PTS-STARTPTS,volume={music_gain},"
                      f"afade=t=in:st=0:d=0.5,afade=t=out:st={max(0.0, duration - fade):.3f}:d={fade:.3f}[mus]")
        mus = "[mus]"
        idx += 1
    if narration:
        labels = []
        for k, (name, start, _dur) in enumerate(narration):
            inputs += ["-i", str(Path(narration_dir) / name)]
            ms = int(round(start * 1000))
            chains.append(f"[{idx}:a]aformat=sample_rates=48000:channel_layouts=stereo,"
                          f"adelay={ms}|{ms}[n{k}]")
            labels.append(f"[n{k}]")
            idx += 1
        chains.append(f"{''.join(labels)}amix=inputs={len(labels)}:normalize=0,"
                      f"apad,atrim=0:{duration:.3f}[narr]")
        narr = "[narr]"
    if mus and narr:
        chains.append("[narr]asplit=2[nsc][nmix]")
        chains.append(f"{mus}aformat=sample_rates=48000:channel_layouts=stereo[mus48]")
        chains.append("[mus48][nsc]sidechaincompress=threshold=0.02:ratio=8:attack=30:release=500[duck]")
        chains.append("[duck][nmix]amix=inputs=2:normalize=0[aout]")
    elif mus:
        chains.append(f"{mus}anull[aout]")
    elif narr:
        chains.append(f"{narr}anull[aout]")
    else:
        return [], None, []
    return inputs, ";".join(chains), ["-map", "0:v", "-map", "[aout]", "-c:a", "aac", "-b:a", "192k"]


def encode_cmd(out, fps, size, duration, music=None, narration=None, narration_dir=None, crf=18):
    """The whole ffmpeg command: raw RGB frames on stdin, H.264 out."""
    w, h = size
    cmd = ["ffmpeg", "-y", "-loglevel", "error", "-f", "rawvideo", "-pix_fmt", "rgb24",
           "-s", f"{w}x{h}", "-r", str(fps), "-i", "-"]
    inputs, graph, maps = audio_args(duration, music, narration, narration_dir)
    cmd += inputs
    if graph:
        cmd += ["-filter_complex", graph]
    cmd += maps or ["-map", "0:v"]
    cmd += ["-c:v", "libx264", "-preset", "slow", "-crf", str(crf), "-pix_fmt", "yuv420p",
            "-movflags", "+faststart", str(out)]
    return cmd
