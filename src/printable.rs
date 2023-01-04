use std::fmt::{Display, Formatter, Write};

/// Printable wrapper.
pub struct Printable<T>(T);

/// Wrap custom struct that can produce printable iterator into [`Printable`].
///
/// # Examples
///
/// ```rust
/// let v = vec![1, 2, 3];
/// assert_eq!(format!("{}", v.printable()), "[1, 2, 3]");
///
/// let v: Vec<usize> = vec![1];
/// assert_eq!(format!("{}", v.printable()), "[1]");
///
/// let v: Vec<usize> = vec![];
/// assert_eq!(format!("{}", v.printable()), "[]")
/// ```
pub trait AsPrintable
    where
        Self: IntoIterator + Sized,
        Self::Item: Display,
        Self::IntoIter: Clone
{
    /// Wrap custom struct that can produce printable iterator into [`Printable`].
    fn printable(self) -> Printable<Self::IntoIter> {
        Printable(self.into_iter())
    }
}

impl<T> AsPrintable for T
    where
        T: IntoIterator,
        T::Item: Display,
        T::IntoIter: Clone
{}

impl<T> From<T> for Printable<T::IntoIter>
    where
        T: IntoIterator,
        T::Item: Display,
        T::IntoIter: Clone
{
    fn from(value: T) -> Self {
        Self(value.into_iter())
    }
}

impl<T> Display for Printable<T>
    where
        T: Clone + Iterator,
        T::Item: Display
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
    {
        f.write_char('[')?;
        let mut iterator = self.0.clone();
        if let Some(v) = iterator.next()
        {
            v.fmt(f)?;
            for v in iterator
            {
                f.write_str(", ")?;
                v.fmt(f)?
            }
        }
        f.write_char(']')
    }
}