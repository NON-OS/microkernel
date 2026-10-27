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

"""Easing and keyframes. Camera moves ease in and out (cubic by default);
nothing on screen moves linearly, so no frame jumps and no move starts or
stops with a jerk."""

import math


def clamp01(x):
    return 0.0 if x < 0.0 else 1.0 if x > 1.0 else x


def in_out_cubic(x):
    x = clamp01(x)
    return 4 * x * x * x if x < 0.5 else 1 - (-2 * x + 2) ** 3 / 2


def out_cubic(x):
    x = clamp01(x)
    return 1 - (1 - x) ** 3


def in_cubic(x):
    x = clamp01(x)
    return x * x * x


def in_out_sine(x):
    x = clamp01(x)
    return -(math.cos(math.pi * x) - 1) / 2


def out_expo(x):
    x = clamp01(x)
    return 1.0 if x >= 1.0 else 1 - 2 ** (-10 * x)


def in_out_quint(x):
    x = clamp01(x)
    return 16 * x ** 5 if x < 0.5 else 1 - (-2 * x + 2) ** 5 / 2


EASES = {
    "inOutCubic": in_out_cubic,
    "outCubic": out_cubic,
    "inCubic": in_cubic,
    "inOutSine": in_out_sine,
    "outExpo": out_expo,
    "inOutQuint": in_out_quint,
}


def ease(name):
    try:
        return EASES[name]
    except KeyError:
        raise ValueError(f"unknown ease {name!r}; one of {sorted(EASES)}") from None


def window(t, t0, t1, fn=in_out_cubic):
    """0 before t0, 1 after t1, eased between."""
    if t1 <= t0:
        return 1.0 if t >= t1 else 0.0
    return fn((t - t0) / (t1 - t0))


def pulse(t, t_in, t_out, fade_in, fade_out, fn=in_out_cubic):
    """0 -> 1 over [t_in, t_in+fade_in], held, 1 -> 0 over [t_out-fade_out, t_out]."""
    return min(window(t, t_in, t_in + fade_in, fn), 1.0 - window(t, t_out - fade_out, t_out, fn))


def lerp(a, b, k):
    if isinstance(a, (tuple, list)):
        return tuple(x + (y - x) * k for x, y in zip(a, b))
    return a + (b - a) * k


def _catmull(p0, p1, p2, p3, s):
    s2, s3 = s * s, s * s * s
    return 0.5 * ((2 * p1) + (-p0 + p2) * s + (2 * p0 - 5 * p1 + 4 * p2 - p3) * s2
                  + (-p0 + 3 * p1 - 3 * p2 + p3) * s3)


class Track:
    """Keyframes [(t, value), ...]. Values are numbers or equal-length tuples.

    mode "segments": each segment eases with `fn`, so the value rests at
    every key. mode "path" (for cameras): one ease over the whole track, and
    a Catmull-Rom curve through the keys, so the move starts and ends at
    rest and never stops between."""

    def __init__(self, keys, fn="inOutCubic", mode="segments"):
        if not keys:
            raise ValueError("a track needs at least one key")
        if mode not in ("segments", "path"):
            raise ValueError(f"unknown track mode {mode!r}")
        self.keys = sorted(((float(t), v) for t, v in keys), key=lambda k: k[0])
        self.fn = ease(fn) if isinstance(fn, str) else fn
        self.mode = mode

    def __call__(self, t):
        keys = self.keys
        if t <= keys[0][0]:
            return keys[0][1]
        if t >= keys[-1][0]:
            return keys[-1][1]
        if self.mode == "path":
            return self._path(t)
        for (t0, v0), (t1, v1) in zip(keys, keys[1:]):
            if t0 <= t <= t1:
                return lerp(v0, v1, self.fn((t - t0) / (t1 - t0)))
        return keys[-1][1]

    def _path(self, t):
        keys = self.keys
        ta, tb = keys[0][0], keys[-1][0]
        tt = ta + (tb - ta) * self.fn((t - ta) / (tb - ta))
        i = max(j for j in range(len(keys) - 1) if keys[j][0] <= tt)
        (t1, v1), (t2, v2) = keys[i], keys[i + 1]
        v0 = keys[i - 1][1] if i > 0 else v1
        v3 = keys[i + 2][1] if i + 2 < len(keys) else v2
        s = (tt - t1) / (t2 - t1)
        if isinstance(v1, (tuple, list)):
            return tuple(_catmull(a, b, c, d, s) for a, b, c, d in zip(v0, v1, v2, v3))
        return _catmull(v0, v1, v2, v3, s)
