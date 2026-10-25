// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The display wire is one for every display: bga and virtio-gpu serve it
//! through the same server, wire and primary crates, and the compositor's
//! client reads it through that wire crate, so the compositor speaks to
//! both through one client and neither driver drifts into a dialect.

const BGA: &str = include_str!("../../../capsule_driver_bga/Cargo.toml");
const GPU: &str = include_str!("../../../capsule_driver_virtio_gpu/Cargo.toml");
const COMPOSITOR: &str = include_str!("../../../compositor/Cargo.toml");

fn depends(toml: &str, krate: &str) -> bool {
    toml.lines().any(|l| l.trim() == format!("{krate} = {{ path = \"../{krate}\" }}"))
}

#[test]
fn bga_and_virtio_gpu_serve_the_one_wire_through_the_same_crates() {
    for krate in ["nonos_gfx_serve", "nonos_gfx_wire", "nonos_gfx_pool"] {
        assert!(depends(BGA, krate), "bga does not serve through {krate}");
        assert!(depends(GPU, krate), "virtio-gpu does not serve through {krate}");
    }
    assert!(depends(COMPOSITOR, "nonos_gfx_wire"), "the compositor's client is another wire");
}
