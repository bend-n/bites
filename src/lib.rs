//! tired of writing endless UNDERSCORES?
//! i have the crate for you!
//!
//! [`to_le_bytes`](u32::to_le_bytes) no more! now, with one simple function, you can [`le`]`(n)` to easily eat all of those bites.
//! ```
//! bites::le(5);
//! bites::be(25);
//! bites::ne(421);
//! ```
#![forbid(unsafe_code)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

/// Convert a number to its little endian bites.
/// ```
/// # assert_eq!(
/// bites::le(5u32)
/// # , 5u32.to_le_bytes()
/// # );
/// ```
pub fn le<T: Bites>(n: T) -> [u8; <T as Bites>::SIZE] {
    n.le()
}

/// Convert a number to its big endian bites.
/// ```
/// # assert_eq!(
/// bites::be(42u16)
/// # , 42u16.to_be_bytes()
/// # );
/// ```
pub fn be<T: Bites>(n: T) -> [u8; <T as Bites>::SIZE] {
    n.be()
}

/// Convert a number to its native endian bites.
/// ```
/// # assert_eq!(
/// bites::ne(7236284524343093608u64)
/// # , 7236284524343093608u64.to_ne_bytes()
/// # );
/// ```
pub fn ne<T: Bites>(n: T) -> [u8; <T as Bites>::SIZE] {
    n.ne()
}

mod seal {
    #[diagnostic::on_unimplemented(message = "not a number", label = "this is not a number")]
    pub trait Seal {}
}

use seal::Seal;
/// Convert a number to its bites.
pub trait Bites: Seal
where
    Self: Sized,
{
    const SIZE: usize = std::mem::size_of::<Self>();
    fn le(self) -> [u8; Self::SIZE];
    fn be(self) -> [u8; Self::SIZE];
    fn ne(self) -> [u8; Self::SIZE];
}

macro_rules! le { ($($x:ident)+) => { $(
    impl Seal for $x {}
    impl Bites for $x {
        #[inline] fn le(self) -> [u8; Self::SIZE] { self.to_le_bytes() }
        #[inline] fn be(self) -> [u8; Self::SIZE] { self.to_be_bytes() }
        #[inline] fn ne(self) -> [u8; Self::SIZE] { self.to_ne_bytes() }
    }
)+ }; }
le!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128);
