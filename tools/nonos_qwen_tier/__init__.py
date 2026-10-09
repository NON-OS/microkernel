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
"""Put a Qwen tier on a NONOS disk, or into the NONOS model repository.

    nonos-qwen-tier.py list
    nonos-qwen-tier.py fetch TIER... --dir DIR
    nonos-qwen-tier.py plan TIER... --dir DIR --image IMAGE [--fresh]
    nonos-qwen-tier.py catalogue --out FILE [--nonos-mirror BASE] [--serial N]
    nonos-qwen-tier.py mirror (TIER... | --all) --dir DIR --nonos-mirror BASE
    nonos-qwen-tier.py show FILE

The tiers, their files, lengths and SHA-256 digests are read from the
signed personality's tables (pinned.rs and the pinned_*.rs beside it, one
a family), so this tool and the system can never disagree on what a tier
is. Each file comes from the Qwen team's Hugging Face repository for its
model. fetch resumes a cut download; plan checks every file against its pin
and hands the layout to nonos-data-plan.py, and the first boot seals the
files into the encrypted volume. catalogue writes the model repository's
catalogue, every tier and every part, signed with the marketplace operator
key (--seed, --pubkey); it names the NONOS repository first when given its
base as --nonos-mirror, laid out as BASE/TIER/FILE, then the upstream file.
mirror fetches the tiers into DIR in that layout, checked, with the signed
catalogue beside them, ready to upload. On a running NONOS, `qwen get` in
the Terminal downloads a tier through the model fetcher capsule.
"""
