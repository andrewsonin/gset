use {gset::Getset, std::{fmt::Debug, ops::{Deref, DerefMut}}};

#[derive(Getset)]
struct Struct<T, M: Default>
    where
        T: Debug
{
    /// Field 1.
    #[getset(get_copy, name = "get_field_1", vis = "pub")]
    #[getset(get_mut, vis = "pub")]
    #[getset(get, vis = "pub")]
    #[getset(set)]
    field_1: f64,

    /// Field 2.
    #[getset(deref_get, vis = "pub")]
    #[getset(deref_get_mut, vis = "pub")]
    #[getset(get, name = "get_field_2")]
    #[getset(set, vis = "pub")]
    field_2: Vec<T>,

    /// Field 3.
    #[getset(deref_get_mut, name = "get_field_3", vis = "pub(crate)")]
    field_3: Vec<M>,

    /// Field 4.
    #[getset(deref_get_copy, name = "get_field_4")]
    field_4: F64,
}

struct F64(f64);

impl Deref for F64
{
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for F64
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}