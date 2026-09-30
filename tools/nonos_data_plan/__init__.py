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
"""Lay out a disk for the data volume: the plan sector, and files to import.

The kernel reads one plain sector at LBA 65536, the disk plan, to find its
data volume and the files waiting to be imported (src/fs/blockfs_volume/
plan_types.rs has the layout). This writes that sector into a raw image,
copies each file to import past the volume, one after another, and grows
the image sparsely to hold them all. The plan is untrusted by design: the
kernel checks every range, and keeps an import only if it hashes to the
digest a signed capsule pins. Choosing a model tier is choosing which files
to pass here; the 7B tier is two files, both passed.

    nonos-data-plan.py IMAGE --volume-sectors N [--import FILE]... [--fresh]

IMAGE is a raw image file or a whole disk; a disk is never resized, and a
plan that does not fit on it is refused before anything is written.

--fresh zeroes the volume's 256-sector header ring, so the next boot formats
a new volume instead of opening the last one.
"""
