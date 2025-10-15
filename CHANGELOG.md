# Changelog

<!-- changelogging: start -->

## [0.5.1](https://github.com/nekitdev/non-empty-slice/tree/v0.5.1) (2025-10-15)

No significant changes.

## [0.5.0](https://github.com/nekitdev/non-empty-slice/tree/v0.5.0) (2025-10-12)

### Features

- Added `escape_ascii` to `NonEmptyBytes`.

- Exported `NonEmptyBoxedBytes` and `NonEmptyBoxedSlice<T>` from `boxed` module.

- Added `repeat` method to `NonEmptySlice<T>` provided `T: Clone`.

## [0.4.1](https://github.com/nekitdev/non-empty-slice/tree/v0.4.1) (2025-10-05)

### Features

- Added unsafe `as_mut_vec` method to `NonEmptyVec<T>`.

## [0.4.0](https://github.com/nekitdev/non-empty-slice/tree/v0.4.0) (2025-10-04)

### Changes

- The entire crate was rewritten. See [docs](https://docs.rs/non-empty-slice) for more information.

## [0.3.1](https://github.com/nekitdev/non-empty-slice/tree/v0.3.1) (2025-08-05)

### Features

- Added `ownership` feature: `IntoOwned` is implemented for `OwnedSlice<T>` provided `T: IntoOwned`.

## [0.3.0](https://github.com/nekitdev/non-empty-slice/tree/v0.3.0) (2025-08-02)

### Changes

- The entire crate was rewritten; see [docs](https://docs.rs/non-empty-slice) for more information.

## [0.2.2](https://github.com/nekitdev/non-empty-slice/tree/v0.2.2) (2025-07-24)

No significant changes.

## [0.2.1](https://github.com/nekitdev/non-empty-slice/tree/v0.2.1) (2025-05-01)

No significant changes.

## [0.2.0](https://github.com/nekitdev/non-empty-slice/tree/v0.2.0) (2025-05-01)

### Changes

- All features except for `std` are now disabled by default.

## [0.1.0](https://github.com/nekitdev/non-empty-slice/tree/v0.1.0) (2025-05-01)

No significant changes.
