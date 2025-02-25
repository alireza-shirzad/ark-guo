use std::ops::Shr;

use serde::{Deserialize, Serialize};
use ark_serialize::{
    CanonicalDeserialize, CanonicalDeserializeWithFlags, 
    CanonicalSerialize, CanonicalSerializeWithFlags, 
    Compress, Flags, Valid, Validate
};
use ark_std::{
    rand::{distributions::Standard, prelude::Distribution, Rng}, 
    One, Zero
};
use zeroize::Zeroize;

use ark_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::*,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rug::{
    Complete, 
    Integer as RugInteger,
    ops::Pow
};
use crate::{
    AdditiveGroup, 
    Integer
};

// Implement the integer trait for ZZ
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ZZ {
    pub value: RugInteger,
}

// Additional useful methods
impl ZZ {
    pub fn is_odd(&self) -> bool {
        self.value.is_odd()
    }

    pub fn modulus(&self, other: &Self) -> Self {
        Self {
            value: self.value.clone() % &other.value,
        }
    }

    pub fn pow(&self, other: &u32) -> Self {
        Self {
            value: self.value.clone().pow(other),
        }
    }

    pub fn div_rem_ceil_mut(&mut self, divisor: &mut Self) {
        self.value.div_rem_ceil_mut(&mut divisor.value);
    }

    pub fn div_rem_floor_mut(&mut self, divisor: &mut Self) {
        self.value.div_rem_floor_mut(&mut divisor.value);
    }

    pub fn extended_gcd_mut(&mut self, other: &mut Self, rop: &mut Self) {
        self.value.extended_gcd_mut(&mut other.value, &mut rop.value);
    }

    pub fn div_exact(&mut self, other: &Self) {
        self.value.div_exact_mut(&other.value);
    } 
}


impl AdditiveGroup for ZZ {
    type Scalar = Self;
}

impl Integer for ZZ { }

// Serialization
// Uses serde serialization
// No compression
impl CanonicalSerialize for ZZ {
    fn serialize_with_mode<W: ark_std::io::Write>(
        &self,
        mut writer: W,
        _compress: Compress,
    ) -> Result<(), ark_serialize::SerializationError> {
        let bytes = bincode::serialize(self).unwrap();
        writer.write_all(&bytes)?;
        Ok(())
    }

    fn serialized_size(&self, _compress: Compress) -> usize {
        let bytes = bincode::serialize(self).unwrap();
        bytes.len()
    }
}

impl CanonicalDeserialize for ZZ {
    fn deserialize_with_mode<R: ark_std::io::Read>(
        mut reader: R,
        _compress: Compress,
        _validate: Validate,
    ) -> Result<Self, ark_serialize::SerializationError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;
        let value = bincode::deserialize(&bytes).unwrap();
        Ok(value)
    }
}

impl CanonicalSerializeWithFlags for ZZ {
    fn serialize_with_flags<W: ark_std::io::Write, F: Flags>(
        &self,
        mut writer: W,
        flags: F,
    ) -> Result<(), ark_serialize::SerializationError> {
        let flag_byte = flags.u8_bitmask();
        let mut bytes = bincode::serialize(self).unwrap();
        bytes.push(flag_byte);

        writer.write_all(&bytes)?;
        Ok(())
    }

    fn serialized_size_with_flags<F: Flags>(&self) -> usize {
        let bytes = bincode::serialize(self).unwrap();
        bytes.len()
    }
}

impl CanonicalDeserializeWithFlags for ZZ {
    fn deserialize_with_flags<R: ark_std::io::Read, F: Flags>(
        mut reader: R,
    ) -> Result<(Self, F), ark_serialize::SerializationError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes)?;

        let value = bincode::deserialize(&bytes[..bytes.len() - 1]).unwrap();
        let flag_byte = bytes[bytes.len() - 1];
        let flags = F::from_u8(flag_byte).unwrap();
        Ok((value, flags))
    }
}

// No checks for integers
impl Valid for ZZ {
    fn check(&self) -> Result<(), ark_serialize::SerializationError> {
        Ok(())
    }
}

// Rand
// Use rug's internal random number generator instead
impl Distribution<ZZ> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> ZZ {
        unimplemented!();
    }
}

// Display
impl Display for ZZ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZZ({})", self.value)
    }
}

// Constants
impl Zero for ZZ {
    fn zero() -> Self {
        Self {
            value: RugInteger::ZERO,
        }
    }

    fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
}

impl One for ZZ {
    fn one() -> Self {
        Self {
            value: RugInteger::from(1),
        }
    }
}

impl Zeroize for ZZ {
    fn zeroize(&mut self) {
        self.value = RugInteger::from(0);
    }
}

// Ops
impl Neg for ZZ {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: -self.value,
        }
    }
}

impl Add for ZZ {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: self.value + other.value,
        }
    }
}

impl Sub for ZZ {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            value: self.value - other.value,
        }
    }
}

impl Mul for ZZ {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            value: self.value * other.value,
        }
    }
}

impl Shr<u32> for ZZ {
    type Output = Self;

    fn shr(self, other: u32) -> Self::Output {
        Self {
            value: self.value >> other,
        }
    }
}

impl AddAssign for ZZ {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl SubAssign for ZZ {
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
    }
}

impl MulAssign for ZZ {
    fn mul_assign(&mut self, other: Self) {
        self.value *= other.value;
    }
}

impl<'a> Add<&'a Self> for ZZ {
    type Output = Self;

    fn add(self, other: &'a Self) -> Self::Output {
        Self {
            value: self.value + &other.value,
        }
    }
}

impl<'a> Sub<&'a Self> for ZZ {
    type Output = Self;

    fn sub(self, other: &'a Self) -> Self::Output {
        Self {
            value: self.value - &other.value,
        }
    }
}

impl<'a> Mul<&'a Self> for ZZ {
    type Output = Self;

    fn mul(self, other: &'a Self) -> Self::Output {
        Self {
            value: self.value * &other.value,
        }
    }
}

impl<'a> AddAssign<&'a Self> for ZZ {
    fn add_assign(&mut self, other: &'a Self) {
        self.value += &other.value;
    }
}

impl<'a> SubAssign<&'a Self> for ZZ {
    fn sub_assign(&mut self, other: &'a Self) {
        self.value -= &other.value;
    }
}

impl<'a> MulAssign<&'a Self> for ZZ {
    fn mul_assign(&mut self, other: &'a Self) {
        self.value *= &other.value;
    }
}

impl <'a> Add<&'a mut Self> for ZZ {
    type Output = Self;

    fn add(self, other: &'a mut Self) -> Self::Output {
        Self {
            value: self.value + &other.value,
        }
    }
}

impl <'a> Sub<&'a mut Self> for ZZ {
    type Output = Self;

    fn sub(self, other: &'a mut Self) -> Self::Output {
        Self {
            value: self.value - &other.value,
        }
    }
}

impl <'a> Mul<&'a mut Self> for ZZ {
    type Output = Self;

    fn mul(self, other: &'a mut Self) -> Self::Output {
        Self {
            value: self.value * &other.value,
        }
    }
}

impl <'a> AddAssign<&'a mut Self> for ZZ {
    fn add_assign(&mut self, other: &'a mut Self) {
        self.value += &other.value;
    }
}

impl <'a> SubAssign<&'a mut Self> for ZZ {
    fn sub_assign(&mut self, other: &'a mut Self) {
        self.value -= &other.value;
    }
}

impl <'a> MulAssign<&'a mut Self> for ZZ {
    fn mul_assign(&mut self, other: &'a mut Self) {
        self.value *= &other.value;
    }
}

// Sum
impl Sum<Self> for ZZ {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b)
    }
}

impl<'a> Sum<&'a Self> for ZZ {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::zero(), |a, b| a + b.clone())
    }
}

// Product
impl Product<Self> for ZZ {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |a, b| a * b)
    }
}

impl<'a> Product<&'a Self> for ZZ {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::one(), |a, b| a * b.clone())
    }
}

// From
impl From<u64> for ZZ {
    fn from(value: u64) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}
impl From<i64> for ZZ {
    fn from(value: i64) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}
impl From<usize> for ZZ {
    fn from(value: usize) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}
impl From<i32> for ZZ {
    fn from(value: i32) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}
impl From<i16> for ZZ {
    fn from(value: i16) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}
impl From<i8> for ZZ {
    fn from(value: i8) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}
impl From<u32> for ZZ {
    fn from(value: u32) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}
impl From<u16> for ZZ {
    fn from(value: u16) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}
impl From<u8> for ZZ {
    fn from(value: u8) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}
impl From<bool> for ZZ {
    fn from(value: bool) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}
impl From<u128> for ZZ {
    fn from(value: u128) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}
impl From<i128> for ZZ {
    fn from(value: i128) -> Self {
        Self {
            value: RugInteger::from(value),
        }
    }
}

impl From<&str> for ZZ {
    fn from(value: &str) -> Self {
        Self {
            value: RugInteger::parse(value).unwrap().complete(),
        }
    }
}

impl From<ZZ> for u64 {
    fn from(value: ZZ) -> Self {
        value.value.to_u64().unwrap()
    }
}
impl From<ZZ> for i64 {
    fn from(value: ZZ) -> Self {
        value.value.to_i64().unwrap()
    }
}
impl From<ZZ> for usize {
    fn from(value: ZZ) -> Self {
        value.value.to_usize().unwrap()
    }
}
impl From<ZZ> for i32 {
    fn from(value: ZZ) -> Self {
        value.value.to_i32().unwrap()
    }
}
impl From<ZZ> for i16 {
    fn from(value: ZZ) -> Self {
        value.value.to_i16().unwrap()
    }
}
impl From<ZZ> for i8 {
    fn from(value: ZZ) -> Self {
        value.value.to_i8().unwrap()
    }
}
impl From<ZZ> for u32 {
    fn from(value: ZZ) -> Self {
        value.value.to_u32().unwrap()
    }
}
impl From<ZZ> for u16 {
    fn from(value: ZZ) -> Self {
        value.value.to_u16().unwrap()
    }
}
impl From<ZZ> for u8 {
    fn from(value: ZZ) -> Self {
        value.value.to_u8().unwrap()
    }
}
impl From<ZZ> for bool {
    fn from(value: ZZ) -> Self {
        value.value.to_i32().unwrap() != 0
    }
}
impl From<ZZ> for u128 {
    fn from(value: ZZ) -> Self {
        value.value.to_u128().unwrap()
    }
}
impl From<ZZ> for i128 {
    fn from(value: ZZ) -> Self {
        value.value.to_i128().unwrap()
    }
}
impl From<ZZ> for String {
    fn from(value: ZZ) -> Self {
        value.value.to_string_radix(10)
    }
}