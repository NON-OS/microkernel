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

"""Pulling a device out of the running machine, as a person pulls a USB stick
or a headset: QEMU removes the device by its id (QMP device_del), and the
guest sees it leave its port. The lane gives each device it adds an id."""


def unplug(q, ident):
    """Remove device `ident`; say whether QEMU took it. False when QEMU
    refused, as it does for an id it does not have."""
    reply = q.cmd("device_del", id=ident)
    ok = "error" not in reply
    print(f"  unplug {ident}: {'done' if ok else reply['error'].get('desc', reply['error'])}",
          flush=True)
    return ok
