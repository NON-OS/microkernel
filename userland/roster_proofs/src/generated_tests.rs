// NONOS Operating System (AGPL-3.0-or-later)
//! Generated CPU lists (sparse affinities with Aff3, duplicates, the boot
//! CPU anywhere or nowhere, more CPUs than the roster holds) against the
//! rule written out, and each running CPU's index read the way the kernel
//! reads it: no two CPUs share an index, and every index fits the tables.

use crate::roster_order::arrange;

fn next(seed: &mut u64, bound: u64) -> u64 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    (*seed >> 33) % bound
}

#[test]
fn the_layout_follows_the_rule_and_no_index_aliases() {
    let mut seed = 39;
    for _ in 0..200_000 {
        let capacity = 1 + next(&mut seed, 6) as usize;
        let pool: Vec<u64> = (0..12).map(|i| (i % 3) << 32 | (i / 3) << 8 | (i % 2)).collect();
        let tree: Vec<u64> =
            (0..next(&mut seed, 12)).map(|_| pool[next(&mut seed, 12) as usize]).collect();
        let own = pool[next(&mut seed, 12) as usize];
        let (mut dense, mut at) = (vec![0; capacity], vec![None; capacity]);
        let n = arrange(&tree, own, &mut dense, &mut at);
        let (dense, at) = (&dense[..n], &at[..n]);

        // The rule: the boot CPU, then the distinct others in tree order,
        // as many as fit; only the boot CPU when the tree lacks it.
        let mut want = vec![own];
        if tree.contains(&own) {
            for affinity in &tree {
                if want.len() < capacity && !want.contains(affinity) {
                    want.push(*affinity);
                }
            }
        }
        assert_eq!(dense, &want[..], "tree {tree:x?} own {own:x}");
        assert_eq!(at[0], tree.iter().position(|a| *a == own));
        for (i, slot) in at.iter().enumerate().skip(1) {
            let t = slot.expect("a CPU past the boot CPU has a tree position");
            assert_eq!(tree[t], dense[i], "tree position names another CPU");
            assert_eq!(tree.iter().position(|a| *a == dense[i]), Some(t), "not its first entry");
        }

        // Every CPU of the machine asks for its index as `cpu::cpu_id` does.
        let mut taken = vec![false; n];
        for cpu in pool.iter().filter(|cpu| tree.contains(cpu) || **cpu == own) {
            match dense.iter().position(|a| a == cpu) {
                Some(index) => {
                    assert!(index < capacity && !taken[index], "index {index} twice");
                    taken[index] = true;
                }
                None => assert!(!dense.contains(cpu), "parked, so it never indexes"),
            }
        }
        assert!(taken.iter().all(|t| *t), "an index with no CPU behind it");
    }
}
