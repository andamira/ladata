A simple & modular data model.

# Features

- `default`: the default features: `std`, `deps_default`.
----

- `std` (default): enables functionality that depends on the standard library.
  Disabling it makes the crate `no_std` compatible.
- `alloc`: enables functionality that depends on allocation.
- `no-std`: enables functionality incompatible with `std` (unused).

---

- `safe`: forbids unsafe at the crate level.
- `safest`: forbids unsafe recursively.
- `unsafe`: enables all the unsafe features:
  - `unsafe_constructors`: enables using unchecked constructors (unused).
  - `unsafe_init`: enables fast array initialization.
  - `unsafe_pop`: enables pop methods not depending on `T: Clone`.
  - `unsafe_unit`: enables the `unit::DataRaw` union.
- `unsafest`: enables unsafe recursively.

---

- `nightly`: enables the nightly features.
- `nightly_docs`: features for `docs.rs`.

---

- `deps_default`: the default set of dependencies.
- `deps_all`: enables all the dependencies.
  - `deps_numerical`: enables the numerical types.
    - `deps_continuous`: enables the floating point number types.
    - `deps_discrete`: enables the integers and rational number types.
  - `deps_categorical`: enables the categorical types.
    - `deps_datetime`: enables the date and time types.
    - `deps_string`: enables the string types.
