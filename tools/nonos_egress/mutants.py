#!/usr/bin/env python3
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
"""Each rule's mutant, put to the self-test: python3 tools/nonos_egress/mutants.py
prints CAUGHT for every mutant the self-test fails on, and exits 1 if any
survives. Each mutant undoes one rule the judge holds (judge.py)."""

import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
M = [
 ("report.py", "if src == self.nic_mac and not self.os_started and firmware.kind(f):", "if src == self.nic_mac and not self.os_started:", "firmware exempts anything from the NIC MAC"),
 ("report.py", "        if src == self.nic_mac and not self.os_started and firmware.kind(f):", "        if src == bytes.fromhex('52550a000202'):\n            return\n        if src == self.nic_mac and not self.os_started and firmware.kind(f):", "slirp MAC exempt again"),
 ("judge.py", "learned = relays(frames(ingress), allow.authorities) if ingress else set()", "learned = relays(frames(ingress) + out, allow.authorities) if ingress else set()", "relays learned from the egress"),
 ("relays.py", "        if src not in authorities:", "        if src[0] not in {a[0] for a in authorities}:", "authority by address alone"),
 ("report.py", '            self.fail("ethertype", f"ethertype {ethertype:#06x} from {mac_text(src)}")', '            pass', "other ethertypes unjudged"),
 ("report.py", '            self.fail("ip-proto", f"{what} to {dst}")\n            return', '            return', "other IP protocols unjudged"),
 ("pcap.py", '            raise Unreadable(f"record at byte {off} runs past the end")', '            return out', "a cut record ends the walk"),
 ("pcap.py", '            raise Unreadable(f"a record header cut short at byte {off}")', '            return out', "left-over bytes ignored"),
 ("pcap.py", "(snaplen and incl > snaplen) or incl > orig", "(snaplen and incl > snaplen)", "incl above orig allowed"),
 ("report.py", "        if proto == 17 and (sport, dport) == (68, 67):", "        if proto == 17 and (sport, dport) == (68, 67) and dst != '255.255.255.255':\n            return\n        if proto == 17 and (sport, dport) == (68, 67):", "dhcp-id only on broadcast (inverted: unicast skipped)"),
 ("report.py", "        if chaddr != src:", "        if False:", "chaddr unchecked"),
 ("report.py", '        if cid is not None and cid != b"\\x01" + src:', "        if False:", "client id unchecked"),
 ("report.py", "        if len(f) < 42 or f[22:28] != src:", "        if len(f) < 42:", "ARP sender unchecked"),
 ("report.py", "        if host and host.split(\":\")[0] not in self.allow.names:", "        if host and host.split(\":\")[0] not in self.allow.names and (dst, port) not in self.allow.peers:", "any Host to a pinned address"),
 ("report.py", '        elif not (self.dhcp_seen and reached):', '        elif False:', "no minimum"),
 ("report.py", '            pinned, relay = (ip, port) in self.allow.peers, (ip, port) in self.learned', '            pinned, relay = ip in {p[0] for p in self.allow.peers}, (ip, port) in self.learned', "peers by address alone"),
 ("report.py", '        elif ethertype == 0x86DD:\n            self.fail("ipv6", f"an IPv6 frame from {mac_text(src)}")', '        elif ethertype == 0x86DD:\n            pass', "guest IPv6 unjudged"),
 ("firmware.py", '    if ethertype == 0x0806:\n        return "arp"', '    if ethertype == 0x0806:\n        return "arp"\n    if ethertype == 0x0800:\n        return "dhcp"', "firmware may send any IPv4"),
]


def main():
    survived = 0
    for name, a, b, why in M:
        p = HERE / name
        orig = p.read_text()
        assert orig.count(a) == 1, (why, a)
        try:
            p.write_text(orig.replace(a, b))
            r = subprocess.run([sys.executable, str(HERE.parent / "nonos-pcap-egress"), "--self-test"],
                               capture_output=True, text=True)
        finally:
            p.write_text(orig)
        survived += r.returncode == 0
        print(("CAUGHT  " if r.returncode else "SURVIVED") + "  " + why)
    return 1 if survived else 0


if __name__ == "__main__":
    sys.exit(main())
