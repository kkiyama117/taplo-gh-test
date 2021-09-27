# Change Log

## 0.6.3

### Fixes

- Formatter fixes

## 0.6.2

### Fixes

- Several comment formatting fixes

## 0.6.1

This is a quick followup version containing a feature that was forgotten in `0.6.0`.

### Features

- Indent entries formatter option

### Fixes

- Formatter indentation fixes

## 0.6.0

### Breaking Changes

- Bumped various dependency versions, most importantly Rowan
- `wasm-bindgen` dependency was made into a feature that can be disabled ([#133](https://github.com/tamasfe/taplo/pull/133))
- Added new formatter options, and formatting results might not always match the existing behaviour

### Fixes

- Fixed false parser and DOM errors
- Fixed some formatter incostencies

## 0.5.2

### Fixes

- Formatting fixes

## 0.5.1

### Fixes

- Fixed comment formatting

## 0.5.0

### Breaking Changes

- Removed `lsp-types` dependency
- Removed builtin schemas

### Fixes

- Formatting fixes

## 0.4.0

### Breaking Changes

- `lsp-types` dependency version bump (this dependency is likely to be removed in future versions)

## 0.3.1

### Additions
- Formatter options can be updated with string key=value pairs.

## 0.3.0

**From this release Taplo only guarantees to support the latest stable Rust release**

### Breaking Changes

- Updated the library to use Rust 1.47.0 stable, it will definitely fail to build on versions older than 1.45.0.

### Fixes
- Added features to documentation
- Documentation should actually compile

## 0.2.0

### Features
- Moved analytics and schema utilities into this library
- Optional `time` and `chrono` support

### Fixes

- Fixed offset-position mapping.

## 0.1.0

### Features

- Initial proper release

## 1.0.0-alpha.x

These releases were labelled way too early incorrectly, and were yanked from the registry.
