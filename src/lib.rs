#![doc(issue_tracker_base_url = "https://github.com/gabrielfalcao/unique-pointer/issues/")]
//! # Gradient Slice
//!
//! gradient-slice is a safe crate to iterate over a gradient of
//! permutations of slices of a Vec
//!
//! ## Example
//!
//! ```
//! use gradient_slice::Gradient;
//! let result = Gradient::new(" abc ".chars().collect::<Vec<char>>())
//!     .map(Vec::from)
//!     .map(|vec| {
//!         vec.iter()
//!             .map(Clone::clone)
//!             .map(String::from)
//!             .collect::<String>()
//!     })
//!     .collect::<Vec<String>>();
//! assert_eq!(
//!     result,
//!     vec![
//!         " ", "a", "b", "c", " ", " a", "ab", "bc", "c ", " ab", "abc", "bc ", " abc",
//!         "abc ", " abc "
//!     ]
//! );
//! ```

use std::iter::Iterator;
use std::marker::PhantomData;

/// ```
/// use gradient_slice::Gradient;
/// let result = Gradient::new(0x1BADB002u32.to_be_bytes().to_vec())
///     .map(Vec::from)
///     .collect::<Vec<Vec<u8>>>();
/// assert_eq!(
///     result,
///     vec![
///         vec![27], vec![173], vec![176], vec![2],
///         vec![27, 173], vec![173, 176], vec![176, 2],
///         vec![27, 173, 176], vec![173, 176, 2],
///         vec![27, 173, 176, 2]
///     ]
/// );
/// ```
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gradient<'a, G> {
    input: Vec<G>,
    start: usize,
    end: usize,
    width: usize,
    wide: bool,
    _marker: PhantomData<&'a G>,
}
impl<'a, G: 'a> Iterator for Gradient<'a, G> {
    type Item = &'a [G];

    fn next(&mut self) -> Option<&'a [G]> {
        if self.end == self.input.len() {
            if self.width == self.len() {
                return None;
            }
        }
        self.end += 1;
        if !self.wide {
            self.wide = true;
            self.width += 1;
            self.start = 0;
            self.end = self.width;
        }

        self.start = self.end - self.width;
        if self.end == self.len() {
            self.wide = false;
        }
        Some(self.window())
    }
}
impl<'a, G: 'a> Gradient<'a, G> {
    fn window(&self) -> &'a [G] {
        unsafe { std::mem::transmute::<&[G], &'a [G]>(&self.input[self.range()]) }
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn range(&self) -> std::ops::Range<usize> {
        self.start()..self.end()
    }

    fn len(&self) -> usize {
        self.input.len()
    }

    pub fn new(s: Vec<G>) -> Gradient<'a, G> {
        Gradient {
            input: s,
            start: 0,
            end: 0,
            width: 1,
            wide: true,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradient() {
        let result = Gradient::new(" abc ".chars().collect())
            .map(Vec::from)
            .map(|vec| {
                vec.iter()
                    .map(Clone::clone)
                    .map(String::from)
                    .collect::<String>()
            })
            .collect::<Vec<String>>();
        assert_eq!(
            result,
            vec![
                " ", "a", "b", "c", " ", " a", "ab", "bc", "c ", " ab", "abc", "bc ", " abc",
                "abc ", " abc "
            ]
        );
    }
}
