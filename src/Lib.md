A simple & modular data model.

# Features

- `std` (default): enables functionality that depends on the standard library.
  Disabling it makes the crate `no_std` compatible.
- `alloc`: enables functionality that depends on allocation. Included in `std`.
- `no-std`: enables functionality incompatible with `std` (unused).

---

- `safe`: forbids all `unsafe` code at the crate level.
- `unsafe`: meta feature enabling every specific unsafe feature:
  - `unsafe_constructors`: enables usage of unchecked constructors.
