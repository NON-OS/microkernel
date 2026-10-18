// NONOS Operating System (AGPL-3.0-or-later)
//! The book by hand: the compare it asks for is always the earliest
//! sleeper's, a CPU past the book is refused and changes nothing, and a
//! firing takes out exactly the CPUs due.

use crate::book::{Book, OutOfRange};

#[test]
fn the_compare_is_the_earliest_sleeper() {
    let mut book = Book::<4>::new();
    assert_eq!(book.sleep(0, Some(500)), Ok(Some(500)));
    assert_eq!(book.sleep(1, Some(300)), Ok(Some(300)));
    assert_eq!(book.sleep(2, None), Ok(Some(300)));
    assert_eq!(book.sleep(3, Some(u64::MAX)), Ok(Some(300)));
    assert_eq!(book.wake(1), Some(500));
    assert_eq!(book.wake(1), Some(500));
    assert_eq!(book.wake(0), Some(u64::MAX));
    assert_eq!(book.wake(3), None);
    assert_eq!(book.wake(9), None);
}

#[test]
fn a_cpu_past_the_book_is_refused_and_changes_nothing() {
    let mut book = Book::<2>::new();
    assert_eq!(book.sleep(0, Some(7)), Ok(Some(7)));
    assert_eq!(book.sleep(2, Some(1)), Err(OutOfRange));
    assert_eq!(book.earliest(), Some(7));
}

#[test]
fn a_firing_takes_out_exactly_the_cpus_due() {
    let mut book = Book::<5>::new();
    for (cpu, due) in [(0, 10), (1, 20), (2, 20), (3, 21), (4, 5)] {
        book.sleep(cpu, Some(due)).unwrap();
    }
    let mut woken = Vec::new();
    assert_eq!(book.expire(20, |c| woken.push(c)), Some(21));
    assert_eq!(woken, vec![0, 1, 2, 4]);
    woken.clear();
    assert_eq!(book.expire(20, |c| woken.push(c)), Some(21));
    assert!(woken.is_empty());
    assert_eq!(book.expire(u64::MAX, |c| woken.push(c)), None);
    assert_eq!(woken, vec![3]);
}
