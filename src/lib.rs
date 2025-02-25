use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, 
    CanonicalDeserializeWithFlags, CanonicalSerializeWithFlags,
};
use ark_std::{One, Zero};
use zeroize::Zeroize;

use ark_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::*,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    UniformRand,
};


pub mod class;
pub mod integer;

#[cfg(feature = "parallel")]
use ark_std::cmp::max;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub trait AdditiveGroup:
    Eq
    + 'static
    + Sized
    + CanonicalSerialize
    + CanonicalDeserialize
    + Clone
    + Default
    + Send
    + Sync
    + Hash
    + Debug
    + Display
    + UniformRand
    + Zeroize
    + Zero
    + Neg<Output = Self>
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<<Self as AdditiveGroup>::Scalar, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<<Self as AdditiveGroup>::Scalar>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> Mul<&'a <Self as AdditiveGroup>::Scalar, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> MulAssign<&'a <Self as AdditiveGroup>::Scalar>
    + for<'a> Add<&'a mut Self, Output = Self>
    + for<'a> Sub<&'a mut Self, Output = Self>
    + for<'a> Mul<&'a mut <Self as AdditiveGroup>::Scalar, Output = Self>
    + for<'a> AddAssign<&'a mut Self>
    + for<'a> SubAssign<&'a mut Self>
    + for<'a> MulAssign<&'a mut <Self as AdditiveGroup>::Scalar>
    + ark_std::iter::Sum<Self>
    + for<'a> ark_std::iter::Sum<&'a Self>
{
    type Scalar: Integer;

    // /// The additive identity of the field.
    // const ZERO: Self;

    /// Doubles `self`.
    #[must_use]
    fn double(&self) -> Self {
        let mut copy = self.clone();
        copy.double_in_place();
        copy
    }
    /// Doubles `self` in place.
    fn double_in_place(&mut self) -> &mut Self {
        *self += self.clone();
        self
    }

    /// Negates `self` in place.
    fn neg_in_place(&mut self) -> &mut Self {
        *self = -(self.clone());
        self
    }
}


pub trait Integer:
    'static
    + Clone
    + Debug
    + Display
    + Default
    + Send
    + Sync
    + Eq
    + Zero
    + One
    + Ord
    + Neg<Output = Self>
    + UniformRand
    + Zeroize
    + Sized
    + Hash
    + CanonicalSerialize
    + CanonicalSerializeWithFlags
    + CanonicalDeserialize
    + CanonicalDeserializeWithFlags
    + AdditiveGroup<Scalar = Self>
    + for<'a> core::iter::Product<&'a Self>
    + From<u128>
    + From<u64>
    + From<u32>
    + From<u16>
    + From<u8>
    + From<i128>
    + From<i64>
    + From<i32>
    + From<i16>
    + From<i8>
    + From<bool>
    + Product<Self>
{
    
}