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

"""Resolve a Linux program's shared library closure and place it in the store.

The Linux personality reads a program and its libraries out of the NONOS store
at the paths the program itself names: the interpreter from PT_INTERP, then
whatever that interpreter opens. So installing an application is placing the
real files at the real paths, and the only hard part is knowing which files.

This walks the ELF dynamic section for DT_NEEDED, resolves each name against a
sysroot using DT_RUNPATH and then the default library directories, and repeats
until nothing new appears. It emits the entry list that tools/nonos-store-pack
takes, or writes them into an image directly.

Nothing here guesses. A name that cannot be resolved inside the sysroot is
reported and the run fails, because a missing library at boot is a dynamic
linker abort with no message worth reading.
"""
