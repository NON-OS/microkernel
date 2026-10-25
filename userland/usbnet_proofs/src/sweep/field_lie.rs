// NONOS Operating System (AGPL-3.0-or-later)
//! The lies a field tells while the framing holds: counts that disagree
//! with what follows, and an endpoint's size or interval out of bounds.

use super::frame_lie::pick_where;
use super::lie::Lie;
use super::rng::Rng;

pub fn field(r: &mut Rng, lie: Lie, d: &mut [Vec<u8>]) -> Option<()> {
    match lie {
        Lie::EndpointCount => {
            let i = pick_where(r, d, |x| x[1] == 0x04)?;
            let follow = d[i + 1..].iter().take_while(|x| x[1] != 0x04);
            let have = follow.filter(|x| x[1] == 0x05).count();
            d[i][4] = (have + r.range(1, 30)) as u8;
        }
        Lie::InterfaceCount => d[0][4] = d[0][4].wrapping_add(r.range(1, 200) as u8),
        Lie::IadCount => {
            let i = pick_where(r, d, |x| x[1] == 0x0B)?;
            (d[i][2], d[i][3]) = (r.byte(), r.pick(&[0, 2, 7, 255]));
        }
        Lie::EndpointField => {
            let i = pick_where(r, d, |x| x[1] == 0x05)?;
            let size = u16::from_le_bytes([d[i][4], d[i][5]]);
            let told = match r.below(5) {
                0 => size | 1 << r.range(11, 15),
                4 => size | 3 << 11,
                1 => 0,
                2 => r.range(1025, 2047) as u16,
                _ => {
                    d[i][6] = 0;
                    return Some(());
                }
            };
            d[i][4..6].copy_from_slice(&told.to_le_bytes());
        }
        _ => return None,
    }
    Some(())
}
