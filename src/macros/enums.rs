/*
    Appellation: enum <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! enum_constructor {
    (@impl $variant:ident.$call:ident) => {
        pub fn $call() -> Self {
            Self::$variant
        }
    };

    ($($variant:ident.$call:ident),* $(,)?) => {
        $(
            enum_constructor!(@impl $variant.$call);
        )*
    };
}

macro_rules! unit_enum {
    ($name:ident { $($variant:ident),* $(,)? }) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            serde::Deserialize,
            serde::Serialize,
            strum::AsRefStr,
            strum::Display,
            strum::EnumCount,
            strum::EnumIter,
            strum::VariantArray,
            strum::VariantNames,
        )]
        #[serde(rename_all = "lowercase")]
        #[strum(serialize_all = "lowercase")]
        pub enum $name {
            $(
                $variant,
            )*
        }

        impl $name {
            enum_constructor!($($variant::$variant),*);
        }
    };
}
