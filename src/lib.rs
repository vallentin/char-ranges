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
//! The implementation specializes [`last()`], [`nth()`], [`next_back()`],
//! and [`nth_back()`]. Such that the length of intermediate characters is
//! not wastefully calculated.
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
//! # `DoubleEndedIterator`
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
//! # Offset Ranges
//!
//! If the input `text` is a substring of some original text, and the produced
//! ranges are desired to be offset in relation to the substring. Then instead
//! of [`.char_ranges()`] use <code>[.char_ranges_offset]\(offset)</code>
//! or <code>.[char_ranges]\().[offset]\(offset)</code>.
//!
//! ```rust
//! use char_ranges::CharRangesExt;
//!
//! let text = "Hello 👋 World 🌏";
//!
//! let start = 11; // Start index of 'W'
//! let text = &text[start..]; // "World 🌏"
//!
//! let mut chars = text.char_ranges_offset(start);
//! // or
//! // let mut chars = text.char_ranges().offset(start);
//!
//! assert_eq!(chars.next(), Some((11..12, 'W'))); // These chars are 1 byte
//! assert_eq!(chars.next(), Some((12..13, 'o')));
//! assert_eq!(chars.next(), Some((13..14, 'r')));
//!
//! assert_eq!(chars.next_back(), Some((17..21, '🌏'))); // This char is 4 bytes
//! ```
//!
//! [`.char_ranges()`]: CharRangesExt::char_ranges
//! [char_ranges]: CharRangesExt::char_ranges
//! [.char_ranges_offset]: CharRangesExt::char_ranges_offset
//! [offset]: CharRanges::offset
//! [`CharRanges`]: CharRanges
//!
//! [`.char_indicies()`]: https://doc.rust-lang.org/std/primitive.str.html#method.char_indices
//! [`DoubleEndedIterator`]: https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html
//!
//! [`last()`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.last
//! [`nth()`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth
//! [`next_back()`]: https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#tymethod.next_back
//! [`nth_back()`]: https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#method.nth_back

#![no_std]
#![forbid(unsafe_code)]
#![forbid(elided_lifetimes_in_paths)]

use core::fmt;
use core::iter::FusedIterator;
use core::ops::Range;
use core::str::CharIndices;

pub trait CharRangesExt {
    /// Returns an iterator over [`char`]s and their start and end byte positions.
    ///
    /// See examples in the [crate root](crate).
    fn char_ranges(&self) -> CharRanges<'_>;

    /// Returns an iterator over [`char`]s and their start and end byte positions,
    /// with an offset applied to all positions.
    ///
    /// See examples in the [crate root](crate).
    #[inline]
    fn char_ranges_offset(&self, offset: usize) -> CharRangesOffset<'_> {
        self.char_ranges().offset(offset)
    }
}

impl CharRangesExt for str {
    #[inline]
    fn char_ranges(&self) -> CharRanges<'_> {
        CharRanges::new(self)
    }
}

/// An iterator over [`char`]s and their start and end byte positions.
///
/// Note: Cloning this iterator is essentially a copy.
///
/// See examples in the [crate root](crate).
#[derive(Clone)]
pub struct CharRanges<'a> {
    iter: CharIndices<'a>,
}

impl<'a> CharRanges<'a> {
    /// Creates an iterator over [`char`]s and their start and end byte positions.
    ///
    /// Consider using <code>text.[char_ranges()]</code>, instead of
    /// explicitly using `CharRanges::new()`.
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
    ///
    /// assert_eq!(chars.as_str(), "BCD");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.iter.as_str()
    }

    /// Returns an iterator over the remaining [`char`]s and their start and
    /// end byte positions, with an offset applied to all positions.
    ///
    /// See examples in the [crate root](crate).
    #[inline]
    pub fn offset(self, offset: usize) -> CharRangesOffset<'a> {
        CharRangesOffset { iter: self, offset }
    }
}

impl Iterator for CharRanges<'_> {
    type Item = (Range<usize>, char);

    #[inline]
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

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (start, c) = self.iter.nth(n)?;
        let end = start + c.len_utf8();
        Some((start..end, c))
    }
}

impl DoubleEndedIterator for CharRanges<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (start, c) = self.iter.next_back()?;
        let end = start + c.len_utf8();
        Some((start..end, c))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (start, c) = self.iter.nth_back(n)?;
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

/// An iterator over [`char`]s and their start and end byte positions,
/// with an offset applied to all positions.
///
/// Note: Cloning this iterator is essentially a copy.
///
/// See examples in the [crate root](crate).
#[derive(Clone)]
pub struct CharRangesOffset<'a> {
    iter: CharRanges<'a>,
    offset: usize,
}

impl<'a> CharRangesOffset<'a> {
    /// Creates an iterator over [`char`]s and their start and end byte positions,
    /// with an offset applied to all positions.
    ///
    /// Consider using <code>text.[char_ranges_offset()]</code> or
    /// <code>text.[char_ranges()].[offset()]</code>, instead of
    /// explicitly using `CharRangesOffset::new()`.
    ///
    /// See examples in the [crate root](crate).
    ///
    /// [char_ranges()]: CharRangesExt::char_ranges
    /// [char_ranges_offset()]: CharRangesExt::char_ranges_offset
    /// [offset()]: CharRanges::offset
    #[inline]
    pub fn new(offset: usize, text: &'a str) -> Self {
        Self {
            iter: text.char_ranges(),
            offset,
        }
    }

    /// Returns the remaining substring.
    ///
    /// # Example
    ///
    /// ```rust
    /// use char_ranges::CharRangesExt;
    ///
    /// let text = "Hello 👋 World 🌏";
    ///
    /// let start = 11; // Start index of 'W'
    /// let text = &text[start..]; // "World 🌏"
    ///
    /// let mut chars = text.char_ranges_offset(start);
    /// assert_eq!(chars.as_str(), "World 🌏");
    ///
    /// assert_eq!(chars.next(), Some((11..12, 'W'))); // These chars are 1 byte
    /// assert_eq!(chars.next_back(), Some((17..21, '🌏'))); // This char is 4 bytes
    ///
    /// assert_eq!(chars.as_str(), "orld ");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'a str {
        self.iter.as_str()
    }

    /// Returns the `offset` this [`CharRangesOffset`] was created with.
    ///
    /// # Example
    ///
    /// ```rust
    /// use char_ranges::CharRangesExt;
    ///
    /// let text = "Hello 👋 World 🌏";
    ///
    /// let start = 11; // Start index of 'W'
    /// let text = &text[start..]; // "World 🌏"
    ///
    /// let mut chars = text.char_ranges_offset(start);
    /// // Offset is `start`
    /// assert_eq!(chars.offset(), start);
    ///
    /// assert_eq!(chars.next(), Some((11..12, 'W'))); // These chars are 1 byte
    /// assert_eq!(chars.next_back(), Some((17..21, '🌏'))); // This char is 4 bytes
    ///
    /// // Offset remains as `start` always
    /// assert_eq!(chars.offset(), start);
    /// ```
    #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }
}

impl Iterator for CharRangesOffset<'_> {
    type Item = (Range<usize>, char);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (r, c) = self.iter.next()?;
        let start = r.start + self.offset;
        let end = r.end + self.offset;
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

    #[inline]
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (r, c) = self.iter.nth(n)?;
        let start = r.start + self.offset;
        let end = r.end + self.offset;
        Some((start..end, c))
    }
}

impl DoubleEndedIterator for CharRangesOffset<'_> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (r, c) = self.iter.next_back()?;
        let start = r.start + self.offset;
        let end = r.end + self.offset;
        Some((start..end, c))
    }

    #[inline]
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        let (r, c) = self.iter.nth_back(n)?;
        let start = r.start + self.offset;
        let end = r.end + self.offset;
        Some((start..end, c))
    }
}

impl FusedIterator for CharRangesOffset<'_> {}

impl fmt::Debug for CharRangesOffset<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CharRangesOffset(")?;
        f.debug_list().entries(self.clone()).finish()?;
        write!(f, ")")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::iter;

    use super::CharRangesExt;

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
        let text = "🗻∈🌏";

        let mut chars = text.char_ranges();
        assert_eq!(chars.as_str(), "🗻∈🌏");

        assert_eq!(chars.next(), Some((0..4, '🗻')));
        assert_eq!(chars.next(), Some((4..7, '∈')));
        assert_eq!(chars.next(), Some((7..11, '🌏')));
        assert_eq!(chars.next(), None);
    }

    #[test]
    fn test_simple_mixed_multi_byte() {
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
    fn test_last() {
        let cases = [
            ("Hello World", 10..11, 'd'),
            ("Hello 👋 World 🌏", 17..21, '🌏'),
            ("🗻12∈45🌏", 11..15, '🌏'),
            ("Hello 🗻12∈45🌏 World", 26..27, 'd'),
        ];
        for (text, r, c) in cases {
            let actual = text.char_ranges().last().unwrap();

            assert_eq!(actual.0, r);
            assert_eq!(actual.1, c);
            assert!(text[r].chars().eq([c]));
        }
    }

    #[test]
    fn test_nth() {
        let cases = [
            "Hello World",
            "Hello 👋 World 🌏",
            "🗻12∈45🌏",
            "Hello 🗻12∈45🌏 World",
        ];
        for text in cases {
            // Since `nth()` doesn't use `next()` internally,
            // then they are be compared
            let char_ranges1 = text.char_ranges();
            let char_ranges2 = iter::from_fn({
                let mut char_ranges = text.char_ranges();
                move || char_ranges.nth(0)
            });
            assert!(char_ranges1.eq(char_ranges2));
        }
    }

    #[test]
    fn test_nth_back() {
        let cases = [
            "Hello World",
            "Hello 👋 World 🌏",
            "🗻12∈45🌏",
            "Hello 🗻12∈45🌏 World",
        ];
        for text in cases {
            // Since `nth_back()` doesn't use `next_back()` internally,
            // then they are be compared
            let char_ranges1 = text.char_ranges().rev();
            let char_ranges2 = iter::from_fn({
                let mut char_ranges = text.char_ranges();
                move || char_ranges.nth_back(0)
            });
            assert!(char_ranges1.eq(char_ranges2));
        }
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

    #[test]
    fn test_offset() {
        let text = "Hello 👋 World 🌏";
        let mut chars = text.char_ranges();

        let emoji_end = {
            assert_eq!(chars.next(), Some((0..1, 'H')));
            assert_eq!(chars.next(), Some((1..2, 'e')));
            assert_eq!(chars.next(), Some((2..3, 'l')));
            assert_eq!(chars.next(), Some((3..4, 'l')));
            assert_eq!(chars.next(), Some((4..5, 'o')));
            assert_eq!(chars.next(), Some((5..6, ' ')));

            let emoji_waving_hand = chars.next();
            assert_eq!(emoji_waving_hand, Some((6..10, '👋')));

            emoji_waving_hand.unwrap().0.end
        };

        let offset_chars = text[emoji_end..].char_ranges().offset(emoji_end);
        assert_eq!(chars.as_str(), offset_chars.as_str());

        for offset_char in offset_chars {
            assert_eq!(chars.next(), Some(offset_char));
        }

        assert_eq!(chars.next(), None);
    }
}
