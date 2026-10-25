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
"""The DHCP rules (dhcp_rules.py) put to frames: where a message may go, the
options it may carry, sname and file, and the firmware's held to the same."""

from .selftest import CLEAN, INGRESS, caught, kinds, run
from .selftest_frames import GUEST, NIC, dhcp, offer, reply

DISCOVER = [(53, b"\x01"), (61, b"\x01" + GUEST)]


def dhcp_rules():
    # Where it goes: broadcast, or a server the ingress named, nothing else.
    caught("dhcp-dst", [dhcp(DISCOVER + [(224, b"exfiltrated")], dst=(6, 6, 6, 6))], "off-net, and data",
           naming="6.6.6.6")
    caught("dhcp-opt", [dhcp(DISCOVER + [(224, b"exfiltrated")], dst=(6, 6, 6, 6))], "option 224")
    renew = dhcp([(53, b"\x03"), (61, b"\x01" + GUEST)], dst=(10, 0, 2, 2))
    assert run(CLEAN + [renew])["status"] == "pass", "a renewal to the server the OFFER named"
    caught("dhcp-dst", [dhcp([(53, b"\x03")], dst=(10, 0, 2, 9))], "a renewal to a server never named")
    unnamed = [reply((49, 13, 145, 234), 9230, b"")]
    caught("dhcp-dst", [renew], "a server the egress alone names", ingress=unnamed)
    caught("dhcp-dst", [renew, offer()], "an OFFER in the egress names no server", ingress=unnamed)
    nak = unnamed + [offer(server=(10, 0, 2, 2), kind=b"\x06")]
    caught("dhcp-dst", [renew], "a NAK names no server", ingress=nak)
    # What it carries: six options, and nothing in sname or file.
    for code, label in [(97, "machine UUID"), (60, "vendor class")]:
        caught("dhcp-id", [dhcp(DISCOVER + [(code, b"PXEClient")])], label)
    for code in (43, 124, 52, 224):
        caught("dhcp-opt", [dhcp(DISCOVER + [(code, b"\x01")])], f"option {code}", naming=f"option {code}")
    caught("dhcp-opt", [dhcp(DISCOVER, sname=b"nonos-laptop")], "sname set", naming="sname or file")
    caught("dhcp-opt", [dhcp(DISCOVER, file=b"exfiltrated")], "file set", naming="sname or file")
    # The firmware's own DHCP, from the NIC's address before the OS sends.
    fw = dhcp([(53, b"\x01"), (60, b"PXEClient:Arch:00007")], chaddr=NIC, src=NIC)
    assert "dhcp-id" in kinds(run([fw] + CLEAN)), "the firmware's vendor class"
    fw_off = dhcp([(53, b"\x01")], chaddr=NIC, src=NIC, dst=(6, 6, 6, 6))
    assert "dhcp-dst" in kinds(run([fw_off] + CLEAN)), "the firmware's DHCP off-net"
    fw_224 = dhcp([(53, b"\x01"), (224, b"data")], chaddr=NIC, src=NIC)
    assert "dhcp-opt" in kinds(run([fw_224] + CLEAN)), "the firmware's option 224"
    assert run([dhcp([(53, b"\x01")], chaddr=NIC, src=NIC)] + CLEAN, INGRESS)["status"] == "pass"
