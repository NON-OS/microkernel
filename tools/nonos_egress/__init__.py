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
"""What a guest sent on the wire, read from QEMU captures, and whether any of
it names the machine or the person outside the anonymity networks.

Two captures, from QEMU's filter-dump on the guest's netdev: the egress
(queue=rx, every frame the guest sent and nothing else) is judged, frame by
frame, with no address exempt; the ingress (queue=tx, what the network sent
the guest) is read only for the relays a pinned authority's consensus names.
The guest cannot write into the ingress capture, so it cannot vouch for a
host. What is allowed comes from a committed allow-list (allowlist.json),
which scrape.py checks against the capsules' sources.
"""
