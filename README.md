# gset

_Getters and Setters for Rust._

[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/gset.svg
[crates-url]: https://crates.io/crates/gset
[docs-badge]: https://img.shields.io/docsrs/gset
[docs-url]: https://docs.rs/gset
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/andrewsonin/gset/blob/main/LICENSE
[actions-badge]: https://github.com/andrewsonin/gset/actions/workflows/ci.yml/badge.svg
[actions-url]: https://github.com/andrewsonin/gset/actions/workflows/ci.yml

Provides a procedural macro capable of deriving  basic getters and setters for structs.

## Usage example

An example of using this library is provided below.

```rust
use gset::Getset;

#[derive(Getset)]
struct Struct<T>
{
    /// Field 1.
    #[getset(get_copy, name = "get_field_1", vis = "pub")]
    #[getset(set)]
    field_1: f64,

    /// Field 2.
    #[getset(get_deref, vis = "pub")]
    #[getset(get_deref_mut, vis = "pub")]
    #[getset(set, vis = "pub")]
    field_2: Vec<T>,
}
```

This also works well for tuple structures,
but the `name` parameter becomes mandatory.

```rust
use gset::Getset;

#[derive(Getset)]
struct Struct<T>(

    /// Field 1.
    #[getset(get_copy, name = "get_field_1", vis = "pub")]
    #[getset(set, name = "set_field_1")]
    f64,

    /// Field 2.
    #[getset(get_deref, name = "get_field_2", vis = "pub")]
    #[getset(get_deref_mut, name = "get_field_2_mut", vis = "pub")]
    #[getset(set, name = "set_field_2", vis = "pub")]
    Vec<T>,
);
```

## Field attributes

All field attributes have the following named parameters:
- `name` — name of the method being inferred.
  Must be a valid Rust [identifier](https://docs.rs/syn/1.0.109/syn/struct.Ident.html).
  This is a required parameter for tuple structs.
- `vis` — visibility of the method being inferred.
  Must be a valid Rust [visibility modifier](https://docs.rs/syn/1.0.109/syn/enum.Visibility.html).
  Visibility is `private` by default.

And some of them have the following named parameter:
- `ty` — return type of the method being inferred. Must be a valid Rust [type](https://docs.rs/syn/1.0.109/syn/enum.Type.html).

#### Legend
Here and further we will adhere to the following notation.
- `field` — field name.
- `T` — field type.

The field attributes currently supported are listed below.

### 1. `get`

Derives a reference getter for a field.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `field`.
- `vis` — visibility of the resulting method. If not set, it will be private.
- `ty` — return type of the resulting method. If not set, it will have the `&T` return type.

#### Example

```rust
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(get, vis = "pub")]
    a: f64,
}
```
will expand into
```rust
struct Struct {
    /// Doc comment.
    a: f64,
}

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn a(&self) -> &f64 {
        &self.a
    }
}
```

### 2. `get_mut`

Derives a mutable getter for a field.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `field_mut`.
- `vis` — visibility of the resulting method. If not set, it will be private.
- `ty` — return type of the resulting method. If not set, it will have the `&mut T` return type.

#### Example

```rust
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(get_mut, vis = "pub")]
    a: f64,
}
```
will expand into
```rust
struct Struct {
    /// Doc comment.
    a: f64,
}

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn a_mut(&mut self) -> &mut f64 {
        &mut self.a
    }
}
```

### 3. `get_copy`

Derives a copy getter for a field.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `field`.
- `vis` — visibility of the resulting method. If not set, it will be private.
- `ty` — return type of the resulting method. If not set, it will have the `T` return type.

#### Example

```rust
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(get_copy, vis = "pub")]
    a: f64,
}
```
will expand into
```rust
struct Struct {
    /// Doc comment.
    a: f64,
}

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn a(&self) -> f64 {
        self.a
    }
}
```

### 4. `get_deref`

Derives a reference getter for a field, which applies the `deref` operation to the resulting reference.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `field`.
- `vis` — visibility of the resulting method. If not set, it will be private.
- `ty` — return type of the resulting method. If not set, it will have the `&<T as ::std::ops:Deref>::Target` return type.

#### Example

```rust
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(get_deref, vis = "pub")]
    a: Vec<f64>,
}
```
will expand into
```rust
struct Struct {
    /// Doc comment.
    a: Vec<f64>,
}

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn a(&self) -> &[f64] {
        &self.a
    }
}
```

### 5. `get_deref_mut`

Derives a mutable getter for a field, which applies the `deref_mut` operation to the resulting reference.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `field_mut`.
- `vis` — visibility of the resulting method. If not set, it will be private.
- `ty` — return type of the resulting method. If not set, it will have the `&mut <T as ::std::ops:Deref>::Target` return type.

#### Example

```rust
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(get_deref_mut, vis = "pub")]
    a: Vec<f64>,
}
```
will expand into
```rust
struct Struct {
    /// Doc comment.
    a: Vec<f64>,
}

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn a_mut(&mut self) -> &mut [f64] {
        &mut self.a
    }
}
```

### 6. `get_deref_copy`

Derives a copy getter for a field, which applies dereferencing to the field value.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `field`.
- `vis` — visibility of the resulting method. If not set, it will be private.
- `ty` — return type of the resulting method. If not set, it will have the `<T as ::std::ops:Deref>::Target` return type.

#### Example

```rust
use derive_more::Deref;
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(get_deref_copy, vis = "pub")]
    a: F64,
}

#[derive(Deref)]
struct F64(f64);
```
will expand into
```rust
use derive_more::Deref;

struct Struct {
    /// Doc comment.
    a: F64,
}

#[derive(Deref)]
struct F64(f64);

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn a(&self) -> f64 {
        *self.a
    }
}
```

### 7. `get_as_ref`

Derives a reference getter for a field, which applies the `as_ref` operation to the resulting reference.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `field`.
- `vis` — visibility of the resulting method. If not set, it will be private.
- `ty` — return type of the resulting method. Required parameter.

#### Example

```rust
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(get_as_ref, vis = "pub", ty = "Option<&f64>")]
    a: Option<f64>,
}
```
will expand into
```rust
struct Struct {
    /// Doc comment.
    a: Option<f64>,
}

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn a(&self) -> Option<&f64> {
        self.a.as_ref()
    }
}
```

### 8. `get_as_deref`

Derives a reference getter for a field, which applies the `as_deref` operation to the resulting reference.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `field`.
- `vis` — visibility of the resulting method. If not set, it will be private.
- `ty` — return type of the resulting method. Required parameter.

#### Example

```rust
use derive_more::Deref;
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(get_as_deref, vis = "pub", ty = "Option<&f64>")]
    a: Option<F64>,
}

#[derive(Deref)]
struct F64(f64);
```
will expand into
```rust
use derive_more::Deref;

struct Struct {
    /// Doc comment.
    a: Option<F64>,
}

#[derive(Deref)]
struct F64(f64);

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn a(&self) -> Option<&f64> {
        self.a.as_deref()
    }
}
```

### 9. `get_as_deref_mut`

Derives a mutable getter for a field, which applies the `as_deref_mut` operation to the resulting reference.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `field_mut`.
- `vis` — visibility of the resulting method. If not set, it will be private.
- `ty` — return type of the resulting method. Required parameter.

#### Example

```rust
use derive_more::{Deref, DerefMut};
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(get_as_deref_mut, vis = "pub", ty = "Option<&mut f64>")]
    a: Option<F64>,
}

#[derive(Deref, DerefMut)]
struct F64(f64);
```
will expand into
```rust
use derive_more::{Deref, DerefMut};

struct Struct {
    /// Doc comment.
    a: Option<F64>,
}

#[derive(Deref, DerefMut)]
struct F64(f64);

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn a_mut(&mut self) -> Option<&mut f64> {
        self.a.as_deref_mut()
    }
}
```

### 10. `set`

Derives a setter for a field.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `set_field`.
- `vis` — visibility of the resulting method. If not set, it will be private.

#### Example

```rust
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(set, vis = "pub")]
    a: f64,
}
```
will expand into
```rust
struct Struct {
    /// Doc comment.
    a: f64,
}

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn set_a(&mut self, value: f64) {
        self.a = value
    }
}
```

### 11. `set_borrow`

Derives a borrowing setter for a field.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `set_field`.
- `vis` — visibility of the resulting method. If not set, it will be private.

#### Example

```rust
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(set_borrow, vis = "pub")]
    a: f64,
}
```
will expand into
```rust
struct Struct {
    /// Doc comment.
    a: f64,
}

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn set_a(&mut self, value: f64) -> &mut Self {
        self.a = value;
        self
    }
}
```

### 12. `set_own`

Derives an owning setter for a field.

#### Parameters
- `name` — name of the resulting method. If not set, it will be named as `set_field`.
- `vis` — visibility of the resulting method. If not set, it will be private.

#### Example

```rust
use gset::Getset;

#[derive(Getset)]
struct Struct {
    /// Doc comment.
    #[getset(set_own, vis = "pub")]
    a: f64,
}
```
will expand into
```rust
struct Struct {
    /// Doc comment.
    a: f64,
}

impl Struct {
    /// Doc comment.
    #[inline]
    pub fn set_a(mut self, value: f64) -> Self {
        self.a = value;
        self
    }
}
```