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

"""A software TPM 2.0 for measured boot, with an endorsement key, in a state
directory of its own. The state is kept from boot to boot, as a machine keeps
its TPM: the data volume's key is derived from it, so a TPM made anew each
boot left every volume the last boot made unopenable, and the machine ran
from RAM alone. --fresh makes a new one with the new data disk."""

import os
import shutil
import subprocess
import time


# What swtpm keeps a TPM 2.0's persistent state in.
PERMALL = "tpm2-00.permall"


def start(state, fresh=False):
    if fresh:
        shutil.rmtree(state, ignore_errors=True)
    os.makedirs(state, exist_ok=True)
    made = not os.path.exists(os.path.join(state, PERMALL))
    if made:
        # The endorsement key is what the boot chain binds the device identity
        # to. swtpm_setup makes it; the certificate it then tries to issue needs
        # a CA nobody has here, so that step may fail and the key is what matters.
        subprocess.run(["swtpm_setup", "--tpm2", "--tpmstate", state, "--create-ek-cert", "--overwrite",
                        "--config", "/dev/null"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    sock = os.path.join(state, "swtpm-sock")
    # A socket left by a boot that ended badly would read as this one's.
    if os.path.exists(sock):
        os.remove(sock)
    proc = subprocess.Popen(["swtpm", "socket", "--tpm2", "--tpmstate", f"dir={state}",
                             "--ctrl", f"type=unixio,path={sock}", "--flags", "startup-clear"])
    for _ in range(100):
        if os.path.exists(sock):
            return proc, sock, made
        time.sleep(0.1)
    proc.kill()
    raise SystemExit("swtpm did not start")
