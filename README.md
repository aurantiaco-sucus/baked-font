# Baked Font

A Rust library that provide a bitmap format optimised for usage with UTF-16 data, single code units or surrogate pairs.
`serde` and `postcard` are used for storage format.

## Usage
* Use a `Font` to look the needed UTF-16 character up with `lookup` method.
* Paint the according area of the bitmap in the `Font` as specified by the position and size (in bitmap), and by the 
  specified offset.