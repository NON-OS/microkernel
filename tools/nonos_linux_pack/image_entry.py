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

"""The catalogue entry an image is listed by: the fields a release needs
and the measurement it pins, unsigned. the project's signing step adds the
publisher signature; nothing here holds a key."""

FORMAT = "nonos.app-image-entry.v1"
PREFIX = "nonos-image-"


def entry(name: str, built: dict, mirror_host: str, chunk: int) -> dict:
    root = built["measurement"].hex()
    version = dict(built["packages"])[name]
    return {
        "format": FORMAT,
        "listing_id": f"linux.{PREFIX}{name}",
        "name": name,
        "version": version,
        "release_id": f"{PREFIX}{name}@{version}",
        "measurement": root,
        "package_hash": root,
        "package_url": f"http://{mirror_host}/linux/{root}.chunks",
        "image_url": f"http://{mirror_host}/linux/{root}.image",
        "image_size": len(built["image"]),
        "list_size": len(built["list"]),
        "chunk": chunk,
        "publisher_signature": "",
        "packages": [f"{n}-{v}" for n, v in built["packages"]],
        "members": [
            {"name": p.name.decode(), "codec": ("tar", "xz")[p.codec], "stored": len(p.stored), "raw": len(p.tar)}
            for p in built["members"]
        ],
    }
