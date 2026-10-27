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

"""The set's geometry: a pinhole camera in a world where y is up and the
floor is y = 0, flat planes placed in it (a screen, an evidence card), and
the perspective coefficients Pillow's Image.transform needs to put a
texture on a plane's four projected corners."""

import math

import numpy as np


def normalize(v):
    v = np.asarray(v, dtype=np.float64)
    n = np.linalg.norm(v)
    return v / n if n else v


def rotation(yaw=0.0, pitch=0.0, roll=0.0):
    """Yaw about y, then pitch about x, then roll about z, in degrees."""
    y, p, r = (math.radians(a) for a in (yaw, pitch, roll))
    ry = np.array([[math.cos(y), 0, math.sin(y)], [0, 1, 0], [-math.sin(y), 0, math.cos(y)]])
    rx = np.array([[1, 0, 0], [0, math.cos(p), -math.sin(p)], [0, math.sin(p), math.cos(p)]])
    rz = np.array([[math.cos(r), -math.sin(r), 0], [math.sin(r), math.cos(r), 0], [0, 0, 1]])
    return ry @ rx @ rz


class Camera:
    """A pinhole at `pos` looking at `target`, `fov` the vertical field of
    view in degrees, for an image `width` x `height` pixels."""

    def __init__(self, pos, target, fov, width, height, roll=0.0):
        self.pos = np.asarray(pos, dtype=np.float64)
        self.width, self.height = width, height
        self.f = (height / 2) / math.tan(math.radians(fov) / 2)
        self.cx, self.cy = width / 2, height / 2
        fwd = normalize(np.asarray(target, dtype=np.float64) - self.pos)
        right = normalize(np.cross(fwd, [0.0, 1.0, 0.0]))
        up = np.cross(right, fwd)
        if roll:
            c, s = math.cos(math.radians(roll)), math.sin(math.radians(roll))
            right, up = right * c + up * s, up * c - right * s
        self.fwd, self.right, self.up = fwd, right, up

    def to_view(self, pts):
        d = np.atleast_2d(np.asarray(pts, dtype=np.float64)) - self.pos
        return np.stack([d @ self.right, d @ self.up, d @ self.fwd], axis=1)

    def project(self, pts):
        """World points (N, 3) -> image points (N, 2) and depths (N,)."""
        v = self.to_view(pts)
        z = np.maximum(v[:, 2], 1e-6)
        u = self.cx + self.f * v[:, 0] / z
        w = self.cy - self.f * v[:, 1] / z
        return np.stack([u, w], axis=1), v[:, 2]

    def rays(self, width, height):
        """World-space ray directions for every pixel of a width x height
        image covering this camera's view (for a lower-resolution layer)."""
        sx = self.width / width
        us = (np.arange(width, dtype=np.float32) + 0.5) * sx
        vs = (np.arange(height, dtype=np.float32) + 0.5) * sx
        x = (us - self.cx) / self.f
        y = -(vs - self.cy) / self.f
        r, u, f = (a.astype(np.float32) for a in (self.right, self.up, self.fwd))
        d = (x[None, :, None] * r[None, None, :] + y[:, None, None] * u[None, None, :]
             + f[None, None, :])
        return d


class Plane:
    """A flat rectangle `width` units wide at `centre`, turned by yaw, pitch
    and roll; its height follows the texture's aspect. Texture coordinates
    (u, v) run 0..1 from the top left."""

    def __init__(self, centre, width, aspect, yaw=0.0, pitch=0.0, roll=0.0):
        self.centre = np.asarray(centre, dtype=np.float64)
        self.width = float(width)
        self.height = float(width) / float(aspect)
        rot = rotation(yaw, pitch, roll)
        self.right, self.up, self.normal = rot[:, 0], rot[:, 1], rot[:, 2]

    def point(self, u, v, lift=0.0):
        return (self.centre + (u - 0.5) * self.width * self.right
                + (0.5 - v) * self.height * self.up + lift * self.normal)

    def corners(self, grow=0.0, lift=0.0):
        """TL, TR, BR, BL, grown outward by `grow` units on every side."""
        gu = grow / self.width
        gv = grow / self.height
        return np.array([self.point(-gu, -gv, lift), self.point(1 + gu, -gv, lift),
                         self.point(1 + gu, 1 + gv, lift), self.point(-gu, 1 + gv, lift)])

    def rect(self, u0, v0, u1, v1, lift=0.0):
        return np.array([self.point(u0, v0, lift), self.point(u1, v0, lift),
                         self.point(u1, v1, lift), self.point(u0, v1, lift)])

    def mirrored(self, pts):
        """The points reflected in the floor, y = 0."""
        m = np.array(pts, dtype=np.float64)
        m[:, 1] = -m[:, 1]
        return m


def perspective_coeffs(out_quad, in_quad):
    """The eight coefficients (a..h) Image.transform(PERSPECTIVE) takes, so
    that output pixel (x, y) samples input ((ax+by+c)/(gx+hy+1),
    (dx+ey+f)/(gx+hy+1)). Both quads are four (x, y) points in order."""
    rows, rhs = [], []
    for (x, y), (X, Y) in zip(out_quad, in_quad):
        rows.append([x, y, 1, 0, 0, 0, -X * x, -X * y])
        rows.append([0, 0, 0, x, y, 1, -Y * x, -Y * y])
        rhs.extend([X, Y])
    return tuple(float(c) for c in np.linalg.solve(np.array(rows, float), np.array(rhs, float)))


def apply_coeffs(coeffs, x, y):
    a, b, c, d, e, f, g, h = coeffs
    den = g * x + h * y + 1
    return (a * x + b * y + c) / den, (d * x + e * y + f) / den


def inverse_depth_plane(pts2d, depths):
    """For a flat quad, 1/depth is affine in image space: returns (A, B, C)
    with 1/z = A*x + B*y + C, from three of its projected corners."""
    m = np.array([[pts2d[i][0], pts2d[i][1], 1.0] for i in range(3)])
    rhs = np.array([1.0 / depths[i] for i in range(3)])
    return tuple(np.linalg.solve(m, rhs))


def bbox(pts, width, height, pad=2):
    """Integer box (x0, y0, x1, y1) around points, clipped to the image;
    None when nothing of it is on the image."""
    pts = np.asarray(pts)
    x0 = max(int(math.floor(pts[:, 0].min())) - pad, 0)
    y0 = max(int(math.floor(pts[:, 1].min())) - pad, 0)
    x1 = min(int(math.ceil(pts[:, 0].max())) + pad, width)
    y1 = min(int(math.ceil(pts[:, 1].max())) + pad, height)
    if x1 <= x0 or y1 <= y0:
        return None
    return x0, y0, x1, y1
