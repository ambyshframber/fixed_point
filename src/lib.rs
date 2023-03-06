use std::ops::{
    Add, Sub, Mul, Div
};

macro_rules! gen_fixed {
    ($name:ident, $inner:ty) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name<const P: i32>($inner);

        impl<const P: i32> $name<P> {
            pub const BITS: usize = std::mem::size_of::<$inner>() * 8;
            pub const MAX: Self = Self(<$inner>::MAX);
            pub const MIN: Self = Self(<$inner>::MIN);
        }

        impl<const P: i32> From<$inner> for $name<P> {
            fn from(i: $inner) -> Self {
                Self(i << P)
            }
        }
        impl<const P: i32> Into<$inner> for $name<P> {
            fn into(self) -> $inner {
                self.0 >> P
            }
        }

        impl<const P: i32> Add for $name<P> {
            type Output = $name<P>;
            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }
        impl<const P: i32> Sub for $name<P> {
            type Output = $name<P>;
            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }

        impl<const P: i32> Mul for $name<P> {
            type Output = $name<P>;
            fn mul(self, other: Self) -> Self {
                let result = self.0 * other.0;
                Self(result >> P)
            }
        }

        impl<const P: i32> Div for $name<P> {
            type Output = $name<P>;
            fn div(self, other: Self) -> Self {
                let result = self.0 / other.0;
                Self(result << P)
            }
        }
    };
}

gen_fixed!(Fixed8, u8);
gen_fixed!(FixedS8, i8);
gen_fixed!(Fixed16, u16);
gen_fixed!(FixedS16, i16);
gen_fixed!(Fixed32, u32);
gen_fixed!(FixedS32, i32);
gen_fixed!(Fixed64, u64);
gen_fixed!(FixedS64, i64);
gen_fixed!(Fixed128, u128);
gen_fixed!(FixedS128, i128);

pub type U12_4 = Fixed16<4>;
pub type U24_8 = Fixed32<8>;
pub type U48_16 = Fixed64<16>;
pub type S12_4 = FixedS16<4>;
pub type S24_8 = FixedS32<8>;
pub type S48_16 = FixedS64<16>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let a = Fixed32::<3>::from(123);
        let b = Fixed32::<3>::from(200);
        let c = a + b;
        assert_eq!(c, Fixed32::<3>::from(323));

        let d = Fixed32::<3>::from(2);
        let e = a * d;
        assert_eq!(e, Fixed32::<3>::from(123 * 2));
        assert_eq!(e / d, a);
    }
}
