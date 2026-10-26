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

"""The machine a drive runs: nix run .#qemu, headless with a software TPM,
its serial console to a file and its own output to DIR/qemu.log."""

import subprocess


def start(root, image, out, serial, qemu_args):
    cmd = ["nix", "run", ".#qemu", "--", "--image", image, "--tpm", "--headless",
           "--serial", str(serial), *qemu_args]
    log = open(out / "qemu.log", "w")
    return subprocess.Popen(cmd, cwd=root, stdout=log, stderr=subprocess.STDOUT)


def stop(qemu):
    """Wait for QEMU to leave after a quit, and end it when it does not."""
    try:
        qemu.wait(timeout=30)
    except subprocess.TimeoutExpired:
        qemu.terminate()
