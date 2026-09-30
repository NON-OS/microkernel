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
"""Put a Qwen tier on a NONOS disk: fetch it, check it, lay it out.

    nonos-qwen-tier.py list
    nonos-qwen-tier.py fetch TIER... --dir DIR
    nonos-qwen-tier.py plan TIER... --dir DIR --image IMAGE [--fresh]

The tiers, their files, lengths and SHA-256 digests are read from the
signed personality's tables (pinned.rs and the pinned_*.rs beside it, one
a family), so this tool and the kernel can never disagree on what a tier
is. Each file comes from the Qwen team's Hugging Face repository for its
model: Qwen/Qwen2.5-<size>-Instruct-GGUF, Qwen/Qwen2.5-Coder-<size>-
Instruct-GGUF or Qwen/Qwen3-<size>-GGUF. fetch runs on any machine with a
network and resumes a cut download; plan checks every file against its pin
before writing a byte, sizes the data volume to hold all the tiers asked
for, and hands the layout to nonos-data-plan.py. NONOS itself never downloads a
model: the disk carries the files, and the first boot seals and verifies
them into the encrypted volume.
"""
