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

"""Listings for the in-tree tools an image does not carry: the packages of
userland/linux_userland/Userland.mk, as tools/nix/store.json lists them.

A package's content is its program and the data it reads, as one tar whose
bytes depend on nothing but the files: entries in path order, no times, no
owners. The listing pins the BLAKE3 of that tar, and the NONOS package mirror
serves it at /linux/<blake3>.tar, so a mirror can serve nothing but what the
signed index pins. A tool's proof files are not part of its content: they
come from the seal after signing, and the market index is made before it.
"""

import io
import json
import sys
import tarfile
from pathlib import Path

from .digest import blake3
from .records import entry, release

NOTE = "built in the NONOS tree, pinned by BLAKE3"
PROOFS = (".nonos_id_cert.bin", ".manifest.bin", ".zk_trailer.bin")
USERLAND = "target/linux-userland/"


def content_files(files: list, capsules: list) -> list:
    """(path in the tree, file on disk) for a package's program and data,
    from its store.json entries: proof files left out, and the program named
    by the file its capsule was built from (target/linux-userland/...)."""
    built = {f"{c['dir']}/target/{c['target']}/release/{c['bin']}": c["prebuilt"]
             for c in capsules if c["prebuilt"]}
    out = []
    for f in files:
        if f["path"].endswith(PROOFS):
            continue
        src = built.get(f["file"], f["file"])
        if not src.startswith(USERLAND):
            raise ValueError(f"{f['path']}: {src} is not a Linux userland output")
        out.append((f["path"].removeprefix("/linux/"), src))
    return sorted(out)


def bundle(files: list, root: Path, links: dict = None) -> bytes:
    """The tar of `files`, each read from `root`, then `links` (path: target,
    the other names the tool answers to, which the personality adds to its
    link table): the same bytes for the same files, wherever and whenever it
    is made."""
    buf = io.BytesIO()
    with tarfile.open(fileobj=buf, mode="w", format=tarfile.USTAR_FORMAT) as tar:
        for path, src in sorted(files):
            # A longer path goes in the header's prefix field, which the
            # personality's tar reader does not take: refused here, not
            # misnamed on a machine.
            if len(path.encode()) > 100:
                raise ValueError(f"{path}: longer than a tar header's name field")
            body = (root / src).read_bytes()
            info = tarfile.TarInfo(path)
            info.size = len(body)
            info.mode = 0o755 if body.startswith(b"\x7fELF") else 0o644
            info.mtime = 0
            info.uid = info.gid = 0
            info.uname = info.gname = ""
            tar.addfile(info, io.BytesIO(body))
        for path, target in sorted((links or {}).items()):
            if len(path.encode()) > 100 or len(target.encode()) > 100:
                raise ValueError(f"{path} -> {target}: longer than a tar header holds")
            if not target.startswith("/"):
                raise ValueError(f"{path} -> {target}: a link table target is a full path")
            info = tarfile.TarInfo(path)
            info.type = tarfile.SYMTYPE
            info.linkname = target
            info.mode = 0o777
            info.mtime = 0
            info.uid = info.gid = 0
            info.uname = info.gname = ""
            tar.addfile(info)
    return buf.getvalue()


def package_entries(listing: Path, store: Path, capsules: Path, root: Path, mirror: str,
                    out: Path, key: str, when_ms: int) -> list:
    """A listing per tool in `listing` that store.json has as a package, its
    content written to `out` as <blake3>.tar for the mirror. A tool this build
    did not make is left out, not listed unready."""
    packages = json.loads(store.read_text()).get("packages", {})
    caps = json.loads(capsules.read_text())
    out.mkdir(parents=True, exist_ok=True)
    found = []
    for item in json.loads(listing.read_text()):
        files = packages.get(item["package"])
        if files is None:
            print(f"tool: {item['package']}: not a package in store.json, not listed", file=sys.stderr)
            continue
        content = content_files(files, caps)
        missing = [src for _, src in content if not (root / src).exists()]
        if missing:
            print(f"tool: {item['package']}: {missing[0]} not built, not listed", file=sys.stderr)
            continue
        body = bundle(content, root, item.get("links"))
        digest = blake3(body)
        (out / f"{digest}.tar").write_bytes(body)
        tail = f"nonos-{item['package']}"
        rel = release(f"{tail}@1", digest, digest, f"http://{mirror}/linux/{digest}.tar",
                      ["x86_64-linux"], [], NOTE, "nonos.operator.linux", when_ms)
        found.append(entry(f"linux.{tail}", digest, item["name"], "NONOS", key, item["text"], [rel]))
    return found
