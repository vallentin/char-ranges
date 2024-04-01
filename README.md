# char-ranges

[![CI](https://github.com/vallentin/char-ranges/workflows/CI/badge.svg)](https://github.com/vallentin/char-ranges/actions?query=workflow%3ACI)
[![Latest Version](https://img.shields.io/crates/v/char-ranges.svg)](https://crates.io/crates/char-ranges)
[![Docs](https://docs.rs/char-ranges/badge.svg)](https://docs.rs/char-ranges)
[![License](https://img.shields.io/github/license/vallentin/char-ranges.svg)](https://github.com/vallentin/char-ranges)

Similar to the standard library's [`.char_indicies()`], but instead of only
producing the start byte position. This library implements [`.char_ranges()`],
that produce both the start and end byte positions.

Note that simply using [`.char_indicies()`] and creating a range by mapping the
returned index `i` to `i..(i + 1)` is not guaranteed to be valid. Given that
some UTF-8 characters can be up to 4 bytes.

| Char  | Bytes | Range  |
| :---: | :---: | :----: |
| `'O'` |   1   | `0..1` |
| `'Ø'` |   2   | `0..2` |
| `'∈'` |   3   | `0..3` |
| `'🌏'` |   4   | `0..4` |

_Assumes encoded in UTF-8._

The implementation specializes [`last()`], [`nth()`], [`next_back()`],
and [`nth_back()`]. Such that the length of intermediate characters is
not wastefully calculated.

## Example

```rust
use char_ranges::CharRangesExt;

let text = "Hello 🗻∈🌏";

let mut chars = text.char_ranges();
assert_eq!(chars.as_str(), "Hello 🗻∈🌏");

assert_eq!(chars.next(), Some((0..1, 'H'))); // These chars are 1 byte
assert_eq!(chars.next(), Some((1..2, 'e')));
assert_eq!(chars.next(), Some((2..3, 'l')));
assert_eq!(chars.next(), Some((3..4, 'l')));
assert_eq!(chars.next(), Some((4..5, 'o')));
assert_eq!(chars.next(), Some((5..6, ' ')));

// Get the remaining substring
assert_eq!(chars.as_str(), "🗻∈🌏");

assert_eq!(chars.next(), Some((6..10, '🗻'))); // This char is 4 bytes
assert_eq!(chars.next(), Some((10..13, '∈'))); // This char is 3 bytes
assert_eq!(chars.next(), Some((13..17, '🌏'))); // This char is 4 bytes
assert_eq!(chars.next(), None);
```

## `DoubleEndedIterator`

[`CharRanges`] also implements [`DoubleEndedIterator`] making it possible to iterate backwards.

```rust
use char_ranges::CharRangesExt;

let text = "ABCDE";

let mut chars = text.char_ranges();
assert_eq!(chars.as_str(), "ABCDE");

assert_eq!(chars.next(), Some((0..1, 'A')));
assert_eq!(chars.next_back(), Some((4..5, 'E')));
assert_eq!(chars.as_str(), "BCD");

assert_eq!(chars.next_back(), Some((3..4, 'D')));
assert_eq!(chars.next(), Some((1..2, 'B')));
assert_eq!(chars.as_str(), "C");

assert_eq!(chars.next(), Some((2..3, 'C')));
assert_eq!(chars.as_str(), "");

assert_eq!(chars.next(), None);
```

## Offset Ranges

If the input `text` is a substring of some original text, and the produced
ranges are desired to be offset in relation to the substring. Then instead
of [`.char_ranges()`] use <code>[.char_ranges_offset]\(offset)</code>
or <code>.[char_ranges]\().[offset]\(offset)</code>.

```rust
use char_ranges::CharRangesExt;

let text = "Hello 👋 World 🌏";

let start = 11; // Start index of 'W'
let text = &text[start..]; // "World 🌏"

let mut chars = text.char_ranges_offset(start);
// or
// let mut chars = text.char_ranges().offset(start);

assert_eq!(chars.next(), Some((11..12, 'W'))); // These chars are 1 byte
assert_eq!(chars.next(), Some((12..13, 'o')));
assert_eq!(chars.next(), Some((13..14, 'r')));

assert_eq!(chars.next_back(), Some((17..21, '🌏'))); // This char is 4 bytes
```

[`.char_ranges()`]: https://docs.rs/char-ranges/*/char_ranges/trait.CharRangesExt.html#tymethod.char_ranges
[char_ranges]: https://docs.rs/char-ranges/*/char_ranges/trait.CharRangesExt.html#tymethod.char_ranges
[char_ranges()]: https://docs.rs/char-ranges/*/char_ranges/trait.CharRangesExt.html#tymethod.char_ranges
[.char_ranges_offset]: https://docs.rs/char-ranges/*/char_ranges/trait.CharRangesExt.html#tymethod.char_ranges_offset
[offset]: https://docs.rs/char-ranges/0.1.0/char_ranges/struct.CharRanges.html#method.offset
[`CharRanges`]: https://docs.rs/char-ranges/*/char_ranges/struct.CharRanges.html

[`.char_indicies()`]: https://doc.rust-lang.org/std/primitive.str.html#method.char_indices
[`DoubleEndedIterator`]: https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html

[`last()`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.last
[`nth()`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.nth
[`next_back()`]: https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#tymethod.next_back
[`nth_back()`]: https://doc.rust-lang.org/std/iter/trait.DoubleEndedIterator.html#method.nth_back
