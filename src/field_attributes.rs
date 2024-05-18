macro_rules! define_field_attributes
{
    ($enum_name:ident { $($var_name:ident => $str_repr:literal),* }) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $enum_name {
            $($var_name),*
        }

        impl std::fmt::Display for $enum_name
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let msg: &str = self.into();
                write!(f, "{msg}")
            }
        }

        impl std::str::FromStr for $enum_name
        {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err>
            {
                use $enum_name::*;
                let res = match s {
                    $($str_repr => $var_name,)*
                    _ => return Err(())
                };
                Ok(res)
            }
        }

        impl $enum_name
        {
            const ALL_KINDS: &'static [&'static str] = &[$($str_repr),*];

            pub fn all_kinds() -> impl std::fmt::Display
            {
                use lazy_format::lazy_format;
                use printable::prelude::PrintableIter;

                Self::ALL_KINDS.iter()
                    .map(|kind| lazy_format!("`{kind}`"))
                    .printable()
            }
        }

        impl From<&$enum_name> for &'static str
        {
            fn from(value: &$enum_name) -> Self
            {
                use $enum_name::*;
                match value {
                    $($var_name => $str_repr),*
                }
            }
        }
    };
}

define_field_attributes!(
    AttributeKind {
        Get           => "get",
        GetMut        => "get_mut",
        GetCopy       => "get_copy",
        GetDeref      => "get_deref",
        GetDerefMut   => "get_deref_mut",
        GetDerefCopy  => "get_deref_copy",
        GetAsRef      => "get_as_ref",
        GetAsDeref    => "get_as_deref",
        GetAsDerefMut => "get_as_deref_mut",
        Set           => "set"
    }
);
