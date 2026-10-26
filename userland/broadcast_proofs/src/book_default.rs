// NONOS Operating System (AGPL-3.0-or-later)
//! The book's `Default`, the proofs' alone: the kernel's static needs its
//! const `new`, and nothing in the kernel wants another way to make one.

use crate::book::Book;

impl<const N: usize> Default for Book<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::book::Book;

    #[test]
    fn the_default_book_is_the_one_new_makes_with_no_deadline() {
        let (mut d, mut n) = (Book::<4>::default(), Book::<4>::new());
        assert_eq!((d.earliest(), n.earliest()), (None, None));
        assert_eq!(d.sleep(3, Some(9)), n.sleep(3, Some(9)));
    }
}
