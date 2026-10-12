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
"""One capsule as its Capsule.mk and signed artifacts say, and the image's list."""

import os

from system_map_consts import TARGET, TRUST, USERLAND
from system_map_files import field_of, machine_of, size_of
from system_map_out import decode

class Capsule(object):
    def __init__(self, mk):
        self.dir = os.path.dirname(mk)
        self.slug = field_of(mk, "CAPSULE_SLUG")
        # The signed artifacts are named by the binary, not the slug: wrong, a
        # fully signed capsule reads as unsigned, worse than reporting nothing.
        self.bin_name = field_of(mk, "CAPSULE_BIN_NAME") or self.slug
        self.namespace = field_of(mk, "CAPSULE_NAMESPACE")
        self.service = field_of(mk, "CAPSULE_SERVICE_ENDPOINT") or "none"
        raw = field_of(mk, "CAPSULE_REQUIRED_CAPS") or "0"
        try:
            self.mask = int(raw, 0)
        except ValueError:
            self.mask = 0
        self.binary = os.path.join(self.dir, "target", TARGET, "release", self.bin_name)
        self.bytes = size_of(self.binary)
        self.machine = machine_of(self.binary)
        self.artifacts = {}
        for kind, ext in (
            ("cert", "nonos_id_cert.bin"),
            ("manifest", "manifest.bin"),
            ("trailer", "zk_trailer.bin"),
        ):
            self.artifacts[kind] = size_of(
                os.path.join(TRUST, "capsules", "{}.{}".format(self.bin_name, ext))
            )

    @property
    def caps(self):
        return decode(self.mask)

    @property
    def attested(self):
        return all(size > 0 for size in self.artifacts.values())

    @property
    def port(self):
        parts = self.service.split(":")
        return parts[1] if len(parts) >= 2 else ""


def load():
    found = []
    if not os.path.isdir(USERLAND):
        return found
    for entry in sorted(os.listdir(USERLAND)):
        mk = os.path.join(USERLAND, entry, "Capsule.mk")
        if os.path.isfile(mk):
            capsule = Capsule(mk)
            if capsule.slug:
                found.append(capsule)
    return found
