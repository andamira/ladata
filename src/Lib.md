A simple & modular data model.

# Features

- `std` (default): enables functionality that depends on the standard library.
  Disabling it makes the crate `no_std` compatible.
- `alloc`: enables functionality that depends on allocation. Included in `std`.
- `no-std`: enables functionality incompatible with `std` (unused).

---

- `safe`: forbid unsafe at the crate level.
- `safest`: forbid unsafe recursively.
- `unsafe`: enables all the unsafe features:
  - `unsafe_constructors`: enables usage of unchecked constructors.
  - `unsafe_unit`: enables the `unit::DataRaw` union.
- `unsafest`: enable unsafe recursively.
