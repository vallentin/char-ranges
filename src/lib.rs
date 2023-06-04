//! Similar to the standard library's [`.char_indicies()`], but instead of only
//! producing the start byte position. This library implements [`.char_ranges()`],
//! that produce both the start and end byte positions.
//!
//! Note that simply using [`.char_indicies()`] and creating a range by mapping the
//! returned index `i` to `i..(i + 1)` is not guaranteed to be valid. Given that
//! some UTF-8 characters can be up to 4 bytes.
//!
//! | Char | Bytes | Range |
//! |:----:|:-----:|:-----:|
//! | `'O'` | 1 | `0..1` |
//! | `'Ø'` | 2 | `0..2` |
//! | `'∈'` | 3 | `0..3` |
//! | `'🌏'` | 4 | `0..4` |
//!
//! _Assumes encoded in UTF-8._
//!
//! # Example
//!
//! ```rust
//! use char_ranges::CharRangesExt;
//!
//! let text = "Hello 🗻∈🌏";
//!
//! let mut chars = text.char_ranges();
//! assert_eq!(chars.as_str(), "Hello 🗻∈🌏");
//!
//! assert_eq!(chars.next(), Some((0..1, 'H'))); // These chars are 1 byte
//! assert_eq!(chars.next(), Some((1..2, 'e')));
//! assert_eq!(chars.next(), Some((2..3, 'l')));
//! assert_eq!(chars.next(), Some((3..4, 'l')));
//! assert_eq!(chars.next(), Some((4..5, 'o')));
//! assert_eq!(chars.next(), Some((5..6, ' ')));
//!
//! // Get the remaining substring
//! assert_eq!(chars.as_str(), "🗻∈🌏");
//!
//! assert_eq!(chars.next(), Some((6..10, '🗻'))); // This char is 4 bytes
//! assert_eq!(chars.next(), Some((10..13, '∈'))); // This char is 3 bytes
//! assert_eq!(chars.next(), Some((13..17, '🌏'))); // This char is 4 bytes
//! assert_eq!(chars.next(), None);
//! ```
//!
//! # Example - `DoubleEndedIterator`
//!
//! [`CharRanges`] also implements [`DoubleEndedIterator`] making it possible to iterate backwards.
//!
//! ```rust
//! use char_ranges::CharRangesExt;
//!
//! let text = "ABCDE";
//!
//! let mut chars = text.char_ranges();
//! assert_eq!(chars.as_str(), "ABCDE");
//!
//! assert_eq!(chars.next(), Some((0..1, 'A')));
//! assert_eq!(chars.next_back(), Some((4..5, 'E')));
//! assert_eq!(chars.as_str(), "BCD");
//!
//! assert_eq!(chars.next_back(), Some((3..4, 'D')));
//! assert_eq!(chars.next(), Some((1..2, 'B')));
//! assert_eq!(chars.as_str(), "C");
//!
//! assert_eq!(chars.next(), Some((2..3, 'C')));
//! assert_eq!(chars.as_str(), "");
//!
//! assert_eq!(chars.next(), None);
//! ```
//!
//! [`.char_ranges()`]: CharRangesExt::char_ranges
//! [`CharRanges`]: CharRanges
//!
//! [`.char_indicies()`]: https://doc.rust-lang.org/core/primitive.str.html#method.char_indices
//! [`DoubleEndedIterator`]: https://doc.rust-lang.org/core/iter/trait.DoubleEndedIterator.html

#![no_std]
#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

use core::fmt;
use core::iter::{DoubleEndedIterator, FusedIterator};
use core::ops::Range;
use core::str::CharIndices;

pub trait CharRangesExt {
    /// Returns an iterator over [`char`]s and their start and end byte positions.
    ///
    /// See examples in the [crate root](crate).
    fn char_ranges(&self) -> CharRanges<'_>;
}

impl CharRangesExt for str {
    #[inline]
    fn char_ranges(&self) -> CharRanges<'_> {
        CharRanges::new(self)
    }
}

/// Note: Cloning this iterator is essentially a copy.
#[derive(Clone)]
pub struct CharRanges<'a> {
    iter: CharIndices<'a>,
}

impl<'a> CharRanges<'a> {
    /// Creates an iterator over [`char`]s and their start and end byte positions.
    ///
    /// Consider using <code>text.[char_ranges()]</code> instead.
    ///
    /// See examples in the [crate root](crate).
    ///
    /// [char_ranges()]: CharRangesExt::char_ranges
    #[inline]
    pub fn new(text: &'a str) -> Self {
        Self {
            iter: text.char_indices(),
        }
    }

    /// Returns the remaining substring.
    ///
    /// # Example
    ///
    /// ```rust
    /// use char_ranges::CharRangesExt;
    ///
    /// let text = "ABCDE";
    ///
    /// let mut chars = text.char_ranges();
    /// assert_eq!(chars.as_str(), "ABCDE");
    ///
    /// assert_eq!(chars.next(), Some((0..1, 'A')));
    /// assert_eq!(chars.next_back(), Some((4..5, 'E')));
    /// assert_eq!(chars.as_str(), "BCD");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.iter.as_str()
    }
}

impl Iterator for CharRanges<'_> {
    type Item = (Range<usize>, char);

    fn next(&mut self) -> Option<Self::Item> {
        let (start, c) = self.iter.next()?;
        let end = start + c.len_utf8();
        Some((start..end, c))
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(mut self) -> Option<(Range<usize>, char)> {
        self.next_back()
    }
}

impl DoubleEndedIterator for CharRanges<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (start, c) = self.iter.next_back()?;
        let end = start + c.len_utf8();
        Some((start..end, c))
    }
}

impl FusedIterator for CharRanges<'_> {}

impl fmt::Debug for CharRanges<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CharRanges(")?;
        f.debug_list().entries(self.clone()).finish()?;
        write!(f, ")")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::CharRangesExt;

    #[test]
    fn test_simple2() {
        let text = "🗻∈🌏";

        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "🗻∈🌏");

        assert_eq!(chars.next(), Some((0..4, '🗻')));
        assert_eq!(chars.next(), Some((4..7, '∈')));
        assert_eq!(chars.next(), Some((7..11, '🌏')));
        assert_eq!(chars.next(), None);
    }

    #[test]
    fn test_empty() {
        let text = "";

        let mut chars = text.char_ranges();
        assert_eq!(chars.next(), None);
        assert_eq!(chars.next_back(), None);
        assert_eq!(chars.as_str(), "");

        let mut chars = text.char_ranges();
        assert_eq!(chars.next_back(), None);
        assert_eq!(chars.next(), None);
        assert_eq!(chars.as_str(), "");
    }

    #[test]
    fn test_empty_single_char() {
        let text = "a";

        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "a");
        assert_eq!(chars.next(), Some((0..1, 'a')));
        assert_eq!(chars.next_back(), None);
        assert_eq!(chars.as_str(), "");

        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "a");
        assert_eq!(chars.next_back(), Some((0..1, 'a')));
        assert_eq!(chars.next(), None);
        assert_eq!(chars.as_str(), "");
    }

    #[test]
    fn test_empty_single_char_multi_byte() {
        let text = "🌏";

        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "🌏");
        assert_eq!(chars.next(), Some((0..4, '🌏')));
        assert_eq!(chars.next_back(), None);
        assert_eq!(chars.as_str(), "");

        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "🌏");
        assert_eq!(chars.next_back(), Some((0..4, '🌏')));
        assert_eq!(chars.next(), None);
        assert_eq!(chars.as_str(), "");
    }

    #[test]
    fn test_simple() {
        let text = "Foo";
        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "Foo");

        assert_eq!(chars.next(), Some((0..1, 'F')));
        assert_eq!(chars.as_str(), "oo");

        assert_eq!(chars.next(), Some((1..2, 'o')));
        assert_eq!(chars.as_str(), "o");

        assert_eq!(chars.next(), Some((2..3, 'o')));
        assert_eq!(chars.as_str(), "");

        assert_eq!(chars.next(), None);
        assert_eq!(chars.as_str(), "");
    }

    #[test]
    fn test_simple_multi_byte() {
        let text = "🗻12∈45🌏";
        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "🗻12∈45🌏");

        assert_eq!(chars.next(), Some((0..4, '🗻')));
        assert_eq!(chars.as_str(), "12∈45🌏");

        assert_eq!(chars.next(), Some((4..5, '1')));
        assert_eq!(chars.as_str(), "2∈45🌏");

        assert_eq!(chars.next(), Some((5..6, '2')));
        assert_eq!(chars.as_str(), "∈45🌏");

        assert_eq!(chars.next(), Some((6..9, '∈')));
        assert_eq!(chars.as_str(), "45🌏");

        assert_eq!(chars.next(), Some((9..10, '4')));
        assert_eq!(chars.as_str(), "5🌏");

        assert_eq!(chars.next(), Some((10..11, '5')));
        assert_eq!(chars.as_str(), "🌏");

        assert_eq!(chars.next(), Some((11..15, '🌏')));
        assert_eq!(chars.as_str(), "");

        assert_eq!(chars.next(), None);
        assert_eq!(chars.as_str(), "");
    }

    #[test]
    fn test_simple_next_back() {
        let text = "Foo";
        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "Foo");

        assert_eq!(chars.next_back(), Some((2..3, 'o')));
        assert_eq!(chars.as_str(), "Fo");

        assert_eq!(chars.next_back(), Some((1..2, 'o')));
        assert_eq!(chars.as_str(), "F");

        assert_eq!(chars.next_back(), Some((0..1, 'F')));
        assert_eq!(chars.as_str(), "");

        assert_eq!(chars.next_back(), None);
        assert_eq!(chars.as_str(), "");
    }

    #[test]
    fn test_simple_next_back_multi_byte() {
        let text = "🗻12∈45🌏";
        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "🗻12∈45🌏");

        assert_eq!(chars.next_back(), Some((11..15, '🌏')));
        assert_eq!(chars.as_str(), "🗻12∈45");

        assert_eq!(chars.next_back(), Some((10..11, '5')));
        assert_eq!(chars.as_str(), "🗻12∈4");

        assert_eq!(chars.next_back(), Some((9..10, '4')));
        assert_eq!(chars.as_str(), "🗻12∈");

        assert_eq!(chars.next_back(), Some((6..9, '∈')));
        assert_eq!(chars.as_str(), "🗻12");

        assert_eq!(chars.next_back(), Some((5..6, '2')));
        assert_eq!(chars.as_str(), "🗻1");

        assert_eq!(chars.next_back(), Some((4..5, '1')));
        assert_eq!(chars.as_str(), "🗻");

        assert_eq!(chars.next_back(), Some((0..4, '🗻')));
        assert_eq!(chars.as_str(), "");

        assert_eq!(chars.next_back(), None);
        assert_eq!(chars.as_str(), "");
    }

    #[test]
    fn test_simple_next_and_next_back() {
        let text = "Foo Bar";

        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "Foo Bar");

        assert_eq!(chars.next_back(), Some((6..7, 'r')));
        assert_eq!(chars.as_str(), "Foo Ba");

        assert_eq!(chars.next_back(), Some((5..6, 'a')));
        assert_eq!(chars.as_str(), "Foo B");

        assert_eq!(chars.next(), Some((0..1, 'F')));
        assert_eq!(chars.as_str(), "oo B");

        assert_eq!(chars.next(), Some((1..2, 'o')));
        assert_eq!(chars.as_str(), "o B");

        assert_eq!(chars.next_back(), Some((4..5, 'B')));
        assert_eq!(chars.as_str(), "o ");

        assert_eq!(chars.next(), Some((2..3, 'o')));
        assert_eq!(chars.as_str(), " ");

        assert_eq!(chars.next(), Some((3..4, ' ')));
        assert_eq!(chars.as_str(), "");

        assert_eq!(chars.next(), None);
        assert_eq!(chars.as_str(), "");
    }

    #[test]
    fn test_simple_next_and_next_back_multi_byte() {
        let text = "🗻12∈45🌏";
        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "🗻12∈45🌏");

        assert_eq!(chars.next_back(), Some((11..15, '🌏')));
        assert_eq!(chars.as_str(), "🗻12∈45");

        assert_eq!(chars.next_back(), Some((10..11, '5')));
        assert_eq!(chars.as_str(), "🗻12∈4");

        assert_eq!(chars.next(), Some((0..4, '🗻')));
        assert_eq!(chars.as_str(), "12∈4");

        assert_eq!(chars.next(), Some((4..5, '1')));
        assert_eq!(chars.as_str(), "2∈4");

        assert_eq!(chars.next_back(), Some((9..10, '4')));
        assert_eq!(chars.as_str(), "2∈");

        assert_eq!(chars.next(), Some((5..6, '2')));
        assert_eq!(chars.as_str(), "∈");

        assert_eq!(chars.next(), Some((6..9, '∈')));
        assert_eq!(chars.as_str(), "");

        assert_eq!(chars.next(), None);
        assert_eq!(chars.as_str(), "");
    }

    #[test]
    fn test_char_ranges() {
        let text = "Hello World";
        for (r, c) in text.char_ranges() {
            let mut chars = text[r].chars();
            assert_eq!(chars.next(), Some(c));
            assert_eq!(chars.next(), None);
        }

        let text = "🗻12∈45🌏";
        for (r, c) in text.char_ranges() {
            let mut chars = text[r].chars();
            assert_eq!(chars.next(), Some(c));
            assert_eq!(chars.next(), None);
        }
    }

    #[test]
    fn test_char_ranges_start() {
        let text = "Hello 🗻12∈45🌏 World";
        let mut chars = text.char_ranges();
        while let Some((r, _c)) = chars.next_back() {
            assert_eq!(chars.as_str(), &text[..r.start]);
        }
    }

    #[test]
    fn test_char_ranges_end() {
        let text = "Hello 🗻12∈45🌏 World";
        let mut chars = text.char_ranges();
        while let Some((r, _c)) = chars.next() {
            assert_eq!(chars.as_str(), &text[r.end..]);
        }
    }

    #[test]
    fn test_full_range() {
        let text = "Hello 🗻12∈45🌏 World\n";
        let mut chars = text.char_ranges();
        while let Some((first, _)) = chars.next() {
            let (last, _) = chars.next_back().unwrap();
            assert_eq!(chars.as_str(), &text[first.end..last.start]);
        }
    }
}
