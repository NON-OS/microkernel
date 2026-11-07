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

"""GTK's caches for an image, built once at pack time: what capsule_linux's
gtk_prep builds at every launch of a GTK program (GSettings' compiled
schemas, gdk-pixbuf's loader cache, the mime database), from the image's
own files, by the image's own tools run under the image's own musl
loader. They ship as one member of the image, covered by its measurement,
under usr/share/nonos-gtk/.
Built twice and compared: an image whose caches do not come out the same
twice is refused, so the same inputs give the same bytes."""

import io
import os
import shutil
import subprocess
import tarfile
import tempfile
from pathlib import Path

from . import image_format, image_members
from .image_hash import digest

LOADER = "lib/ld-musl-x86_64.so.1"
COMPILE, QUERY, MIME = "usr/bin/glib-compile-schemas", "usr/bin/gdk-pixbuf-query-loaders", "usr/bin/update-mime-database"
SCHEMAS, LOADERS, MIME_SRC = "usr/share/glib-2.0/schemas", "usr/lib/gdk-pixbuf-2.0/2.10.0/loaders", "usr/share/mime"
BASE = "usr/share/nonos-gtk/"
# What GLib reads of the mime database; the per-type files and the copied
# sources go, as gtk_prep's mime.rs drops them.
MIME_KEEP = ("XMLnamespaces", "aliases", "generic-icons", "globs", "globs2", "icons", "magic", "mime.cache",
             "subclasses", "treemagic", "types", "version")
SECONDS = 600


def loaders_text(raw: bytes, root: str) -> bytes:
    """The loader cache as the device reads it: the build's root cut from
    every path, and the comments, which name the build's directories,
    dropped."""
    prefix = ('"' + root.rstrip("/") + "/").encode()
    lines = [l for l in raw.split(b"\n") if not l.startswith(b"#")]
    return b"\n".join(l.replace(prefix, b'"/') for l in lines)


def tar(files: dict) -> bytes:
    """`files` (path -> bytes) as one tar, in path order, no times, no owners."""
    buf = io.BytesIO()
    with tarfile.open(fileobj=buf, mode="w", format=tarfile.PAX_FORMAT) as t:
        for path in sorted(files):
            info = tarfile.TarInfo(path)
            info.size, info.mode, info.mtime, info.uid, info.gid = len(files[path]), 0o644, 0, 0, 0
            info.uname = info.gname = ""
            t.addfile(info, io.BytesIO(files[path]))
    return buf.getvalue()


def sysroot(parts: list, at: Path) -> None:
    """Every member's files under `at`, later members over earlier, as the
    device places them."""
    for p in parts:
        with tarfile.open(fileobj=io.BytesIO(p.tar)) as t:
            t.extractall(at, filter="tar")


def run(root: Path, argv: list, env: dict) -> bytes:
    lp = f"{root}/usr/lib:{root}/lib"
    out = subprocess.run([str(root / LOADER), "--library-path", lp, *argv], env=dict(env, LD_LIBRARY_PATH=lp),
                         capture_output=True, timeout=SECONDS, check=False)
    if out.returncode != 0:
        raise ValueError(f"{Path(argv[0]).name} failed: {out.stderr.decode(errors='replace')[:200]}")
    return out.stdout


def built(root: Path, work: Path) -> dict:
    """The caches the closure's tools make, path -> bytes; empty when it
    carries none of them."""
    have = lambda rel: (root / rel).exists()
    files = {}
    if have(LOADER) and have(COMPILE) and have(SCHEMAS):
        (work / "schemas").mkdir()
        run(root, [str(root / COMPILE), f"--targetdir={work / 'schemas'}", str(root / SCHEMAS)], {})
        files[BASE + "schemas/gschemas.compiled"] = (work / "schemas/gschemas.compiled").read_bytes()
    if have(LOADER) and have(QUERY) and have(LOADERS):
        raw = run(root, [str(root / QUERY)], {"GDK_PIXBUF_MODULEDIR": str(root / LOADERS)})
        files[BASE + "loaders.cache"] = loaders_text(raw, str(root))
    if have(LOADER) and have(MIME) and have(MIME_SRC + "/packages"):
        mime = work / "share/mime"
        shutil.copytree(root / MIME_SRC, mime, symlinks=True)
        run(root, [str(root / MIME), str(mime)], {})
        for name in MIME_KEEP:
            if (mime / name).is_file():
                files[BASE + "share/mime/" + name] = (mime / name).read_bytes()
    return files


def caches(parts: list):
    """The member of GTK's caches for `parts`, or None when the closure
    carries no GTK tool; ValueError when a tool fails or two builds differ."""
    twice = []
    for _ in range(2):
        with tempfile.TemporaryDirectory() as d:
            root, work = Path(d) / "root", Path(d) / "work"
            root.mkdir(), work.mkdir()
            sysroot(parts, root)
            twice.append(built(root, work))
    if twice[0] != twice[1]:
        raise ValueError("GTK's caches came out differently twice")
    if not twice[0]:
        return None
    body = tar(twice[0])
    codec, stored = image_members.stored(body)
    return image_format.Part(b"nonos-gtk-" + digest(body)[:8].hex().encode(), codec, body, stored)
