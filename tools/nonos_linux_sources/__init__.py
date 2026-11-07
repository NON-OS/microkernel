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

"""The built content of the catalogue's source recipes (SimpleX, DarkFi),
made present at the seal's linux_sources directory with no hand step.

Each recipe's content is pinned in userland/capsule_market/
linux-catalogue.sources.lock.json by `B`, the BLAKE3 of its bundled tar,
and `S`, its length (nonos_market_catalogue source_lock.py). For each
package whose <package>.tar is missing or off its pin, the first of these
that holds the pin is taken, and none is ever taken unchecked:

  1. the cache, keyed by the pin (cache.py), filled by every earlier run;
  2. the content branch (store.py), whose tars were built on a machine
     that builds them and are held to the same pins;
  3. for a recipe that lists upstream's release binary, upstream itself,
     through tools/nonos-source-asset (asset.py).

A package none of them gives stops the run by name, with how its content
is built. Nothing is relaxed: the seal still requires every source recipe
of a release (tools/nonos_seal/market.py)."""
