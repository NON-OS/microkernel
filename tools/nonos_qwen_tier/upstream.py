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
"""The file each pin is published under, when the data volume keeps it
under another name.

The Qwen team names the parts of Qwen2.5-Coder 7B, 14B and 32B in 52 to 54
bytes, and the data volume keeps at most 48 (pins.keepable), so the signed
pins keep them under shorter names: the published name without
"-instruct", the part's "-0000N-of-0000M.gguf" ending unchanged. Every
download, from the Qwen team or from the NONOS model repository, asks for
the published name; the catalogue and the volume use the pinned one.
"""

PUBLISHED = {
    "qwen2.5-coder-7b-q4_k_m-00001-of-00002.gguf":
        "qwen2.5-coder-7b-instruct-q4_k_m-00001-of-00002.gguf",
    "qwen2.5-coder-7b-q4_k_m-00002-of-00002.gguf":
        "qwen2.5-coder-7b-instruct-q4_k_m-00002-of-00002.gguf",
    "qwen2.5-coder-14b-q4_k_m-00001-of-00002.gguf":
        "qwen2.5-coder-14b-instruct-q4_k_m-00001-of-00002.gguf",
    "qwen2.5-coder-14b-q4_k_m-00002-of-00002.gguf":
        "qwen2.5-coder-14b-instruct-q4_k_m-00002-of-00002.gguf",
    "qwen2.5-coder-32b-q4_k_m-00001-of-00003.gguf":
        "qwen2.5-coder-32b-instruct-q4_k_m-00001-of-00003.gguf",
    "qwen2.5-coder-32b-q4_k_m-00002-of-00003.gguf":
        "qwen2.5-coder-32b-instruct-q4_k_m-00002-of-00003.gguf",
    "qwen2.5-coder-32b-q4_k_m-00003-of-00003.gguf":
        "qwen2.5-coder-32b-instruct-q4_k_m-00003-of-00003.gguf",
}


def upstream(name):
    """The name `name`, as pinned, is published and downloaded under."""
    return PUBLISHED.get(name, name)
