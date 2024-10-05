/*
    Appellation: getter <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

macro_rules! get {
    (@alt $name:ident::<$ty:ty>) => {
        pub const fn $name(&self) -> &$ty {
            &self.$name
        }
    };
    (@impl $name:ident::<$ty:ty>) => {
        paste::paste! {
            pub const fn [<get_ref_ $name>](&self) -> &$ty {
                &self.$name
            }
        }
    };

    ($($name:ident::<$ty:ty>),* $(,)?) => {
        $(
            get!(@impl $name::<$ty>);
        )*
    };
}

macro_rules! get_mut {
    (@impl $name:ident::<$ty:ty>) => {
        paste::paste! {
            pub fn [<$name _mut>](&mut self) -> &mut $ty {
                &mut self.$name
            }
        }
    };

    ($($name:ident::<$ty:ty>),* $(,)?) => {
        $(
            get_mut!(@impl $name::<$ty>);
        )*
    };
}
