#![allow(dead_code)]
#![allow(clippy::struct_field_names)]

use gset::Getset;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

#[derive(Getset)]
struct Struct1<'a, T, M: Default>
where
    T: Debug,
{
    /// Field 1.
    #[getset(get_copy, name = "get_field_1", vis = "pub")]
    #[getset(get_mut, vis = "pub")]
    #[getset(get, vis = "pub")]
    #[getset(set)]
    field_1: f64,

    /// Field 2.
    #[getset(get_deref, vis = "pub")]
    #[getset(get_deref_mut, vis = "pub")]
    #[getset(get, name = "get_field_2")]
    #[getset(set, vis = "pub")]
    field_2: Vec<T>,

    /// Field 3.
    #[getset(get_deref_mut, name = "get_field_3", vis = "pub(crate)")]
    field_3: Vec<M>,

    /// Field 4.
    #[getset(get_deref_copy, name = "get_field_4")]
    field_4: F64,

    /// Field 5.
    #[getset(get_as_ref, name = "get_field_5", ty = "Option<&F64>")]
    #[getset(get_as_deref, name = "get_field_5_deref", ty = "Option<&f64>")]
    #[getset(
        get_as_deref_mut,
        name = "get_field_5_deref_mut",
        ty = "Option<&mut f64>"
    )]
    field_5: Option<F64>,

    /// Field 6.
    #[getset(set, name = "set_field_6")]
    #[getset(set_borrow, name = "set_field_6_borrow")]
    #[getset(set_own, name = "set_field_6_own")]
    field_6: &'a f64,
}

#[derive(Getset)]
struct Struct2<T, M: Default>(
    /// Field 1.
    #[getset(get_copy, name = "get_field_1", vis = "pub")]
    #[getset(get_mut, name = "get_field_1_mut", vis = "pub")]
    #[getset(get, name = "get_field_1_ref", vis = "pub")]
    #[getset(set, name = "set_field_1")]
    f64,
    /// Field 2.
    #[getset(get_deref, name = "get_field_2", vis = "pub")]
    #[getset(get_deref_mut, name = "get_field_2_mut", vis = "pub")]
    #[getset(get, name = "get_field_2_ref")]
    #[getset(set, name = "set_field_2", vis = "pub")]
    Vec<T>,
    /// Field 3.
    #[getset(get_deref_mut, name = "get_field_3", vis = "pub(crate)")]
    Vec<M>,
    /// Field 4.
    #[getset(get_deref_copy, name = "get_field_4")]
    F64,
    /// Field 5.
    #[getset(get_as_ref, name = "get_field_5", ty = "Option<&F64>")]
    #[getset(get_as_deref, name = "get_field_5_deref", ty = "Option<&f64>")]
    #[getset(
        get_as_deref_mut,
        name = "get_field_5_deref_mut",
        ty = "Option<&mut f64>"
    )]
    Option<F64>,
)
where
    T: Debug;

struct F64(f64);

impl Deref for F64 {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for F64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
