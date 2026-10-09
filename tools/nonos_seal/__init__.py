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

"""The seal: ek's step between the reproducible build and a bootable image.

The flake builds every artifact from the source alone. Sealing adds what only
the keys can add, in the order the trust chain needs it:

  1. inputs    the market index and the model catalogue, signed by the market
               operator, which two capsules embed
  2. capsules  each capsule's NONOS-ID certificate and signed manifest, then
               one STARK enrollment of the whole set under the policy root
  3. kernel    the kernel, built against that trust set, enrolled under its
               own root
  4. loader    the bootloader, built against the kernel's root and public
               keys, enrolled under its own root
  5. image     the kernel signed and its trailer embedded, the release
               records, Secure Boot, the ESP, the store, the USB image and the
               ISO
  6. verify    every check the boot will make, run against what was written

Each phase writes only public files into the tree (nonos-data/), stages them,
and asks the flake to build the next artifact from the tree as it now stands.
So the kernel and loader in a sealed image are byte for byte what `nix build`
gives anyone from the commit ek makes afterwards. The enrollment draws fresh
randomness and the signatures need ek's keys, so two seals of one commit are
not identical; the artifacts inside them are."""
