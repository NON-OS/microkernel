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

//! A tensor's shape, from the file, bounded before it is multiplied.

use super::build::Gguf;
use crate::{parse, GgufError as E, TensorError as T, DEFAULT};

fn check(g: &Gguf) -> Result<crate::Summary, E> {
    parse(&mut g.out.as_slice(), g.out.len() as u64, &DEFAULT)
}

#[test]
fn a_tensor_shape_is_bounded_before_it_is_multiplied() {
    let one = |shape: &[u64], ty: u32| {
        let mut g = Gguf::header(3, 1, 0);
        g.tensor("t", shape, ty, 0).data(32, 64);
        check(&g)
    };
    assert_eq!(
        one(&[1 << 40], 0),
        Err(E::Tensor { tensor: 0, why: T::DimensionTooLarge(1 << 40) })
    );
    assert_eq!(one(&[1, 1, 1, 1, 1], 0), Err(E::Tensor { tensor: 0, why: T::Dims(5) }));
    assert_eq!(one(&[], 0), Err(E::Tensor { tensor: 0, why: T::Dims(0) }));
    assert_eq!(one(&[4, 0], 0), Err(E::Tensor { tensor: 0, why: T::ZeroDimension }));
    assert_eq!(one(&[256], 19), Err(E::Tensor { tensor: 0, why: T::UnknownType(19) }));
    assert_eq!(
        one(&[100], 12),
        Err(E::Tensor { tensor: 0, why: T::RowNotWholeBlocks { ne0: 100, block: 256 } })
    );
    /* 2^31 cubed overflows u64; 2^31 * 16 F32 is 2^37 bytes, past 2^34. */
    assert_eq!(
        one(&[1 << 31, 1 << 31, 1 << 31], 0),
        Err(E::Tensor { tensor: 0, why: T::TooLarge })
    );
    assert_eq!(one(&[1 << 31, 16], 0), Err(E::Tensor { tensor: 0, why: T::TooLarge }));
}
