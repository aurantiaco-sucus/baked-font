# Baked Font

A Rust library that provide a bitmap format optimised for usage with Rust's `char`, single code units or pairs of them. `serde` and `postcard` are used for storage format.

## Usage
* Use a `Font` to look the needed `Glyph` up somewhere in a slice of `char`. It's important to avail the whole sequence because of the 2-unit glyphs.
* Paint the according area of the bitmap in the `Font` as specified by the position and size (in bitmap), and by the specified offset. The bitmap is stored in row-major grayscale `u8`.
