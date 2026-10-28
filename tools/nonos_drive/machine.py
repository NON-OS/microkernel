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
its serial console to a file and its own output to DIR/qemu.log. A drive
filmed (record.py) shows it full screen on its X display instead."""

import subprocess

from .record import display_env


def command(image, serial, qemu_args, shown=False):
    view = "--shown" if shown else "--headless"
    return ["nix", "run", ".#qemu", "--", "--image", image, "--tpm", view,
            "--serial", str(serial), *qemu_args]


def start(root, image, out, serial, qemu_args, display=None):
    """QEMU, headless; or with `display`, an X display's number, in a window
    on it, full screen."""
    shown = display is not None
    log = open(out / "qemu.log", "w")
    return subprocess.Popen(command(image, serial, qemu_args, shown), cwd=root, stdout=log,
                            stderr=subprocess.STDOUT, env=display_env(display) if shown else None)


def stop(qemu):
    """Wait for QEMU to leave after a quit, and end it when it does not."""
    try:
        qemu.wait(timeout=30)
    except subprocess.TimeoutExpired:
        qemu.terminate()
