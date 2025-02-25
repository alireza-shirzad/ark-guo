use serde::{Deserialize, Serialize};
use ark_serialize::{
    CanonicalDeserialize, CanonicalSerialize, Compress, Valid, Validate
};
use ark_std::{rand::{distributions::Standard, prelude::Distribution, Rng}, One, Zero};
use zeroize::Zeroize;

use ark_std::{
    fmt::{Debug, Display},
    hash::Hash,
    iter::*,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::AdditiveGroup;
use crate::class::config::{ClassConfig, TestClassConfig};

pub mod config;

// Class group compressed
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ClassGroupCompressed <T: ClassConfig> {
    pub ap: T::Int,
    pub g: T::Int,
    pub tp: T::Int,
    pub b0: T::Int,
    pub is_neg: bool,
}

// Class group uncompressed
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash, CanonicalSerialize, CanonicalDeserialize)]
pub struct ClassGroup <T: ClassConfig> {
    pub a: T::Int,
    pub b: T::Int,
    pub c: T::Int,
}

impl <T: ClassConfig> ClassGroup<T> {
    // Print the discriminant from the config
    pub fn discriminant(&self) -> T::Int {
        T::discriminant()
    }

    // Print the default nucomp bound from the config
    pub fn default_nucomp_bound(&self) -> T::Int {
        T::default_nucomp_bound()
    }

    // Unchecked constructor - only use when discriminant is guaranteed to be equal
    pub fn new_unchecked(a: T::Int, b: T::Int, c: T::Int) -> Self {
        Self { a, b, c }
    }

    // Constructor - checks that discriminant is equal
    pub fn new(a: T::Int, b: T::Int, c: T::Int) -> Self {
        let disc = b.clone() * b.clone() - T::Int::from(4) * a.clone() * c.clone();
        assert_eq!(disc, T::discriminant());

        Self { a, b, c }
    }

}

impl ClassGroup<TestClassConfig> {
    // Normalize
    pub fn normalize(&mut self) {
        let mut q = self.b.clone();
        let mut r = self.a.clone();
        
        q.div_rem_ceil_mut(&mut r);
        if q.is_odd() {
            r += self.a.clone();
        }
        q = q >> 1;
        std::mem::swap(&mut r, &mut self.b);
        r += self.b.clone();
        r = r >> 1;
        self.c -= q * r;
    }

    // Reduction
    fn rho(&mut self) {
        std::mem::swap(&mut self.a, &mut self.c);
        self.b.neg_in_place();
        self.normalize();
    }

    pub fn reduce(&mut self) {
        while self.a > self.c {
            self.rho();
        }

        if self.a == self.c {
            self.b.neg_in_place();
        }
    }

    // NUDUPL
    pub fn nudupl(r: &mut Self, f: &Self) {
        // f = (a,b,c)
        // r = result

        // d = gcd(a,b) = ua + vb
        let mut d = f.a.clone();
        let mut u = f.b.clone();
        let mut v = <TestClassConfig as ClassConfig>::Int::zero();

        d.extended_gcd_mut(&mut u, &mut v);

        r.a = f.a.clone();
        r.b = f.b.clone();

        if !d.is_one() {
            r.a.div_exact(&mut d);
            r.b.div_exact(&mut d);
        }
        
        // Dx = -uc
        let mut dx = f.c.clone();
        dx.mul_assign(&mut u);
        dx.neg_in_place();

        // r.c = vc
        r.c = f.c.clone();
        r.c.mul_assign(&mut v);

        let mut q = r.c.clone();
        let mut rr = r.a.clone();
        q.div_rem_floor_mut(&mut rr);
        r.c = rr;

        dx -= q * r.b.clone();

        todo!("Partial euclid and rest");
    }

    // NUCOMP
    pub fn nucomp(_r: &mut Self, _f1: &Self, _f2: &Self) {
        todo!("NUCOMP");
    }

    // NUPOW
    pub fn nupow(_r: &mut Self, _f: &Self, _n: &<TestClassConfig as ClassConfig>::Int) {
        todo!("NUPOW");
    }
}

impl AdditiveGroup for ClassGroup<TestClassConfig> {
    type Scalar = <TestClassConfig as ClassConfig>::Int;
}

impl Display for ClassGroup<TestClassConfig> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CG({}, {}, {})", self.a, self.b, self.c)
    }
}

// Dist
impl Distribution<ClassGroup<TestClassConfig>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> ClassGroup<TestClassConfig> {
        unimplemented!();
    }
}

// Constants
impl Zero for ClassGroup<TestClassConfig> {
    fn zero() -> Self {
        let a = <TestClassConfig as ClassConfig>::Int::one();
        let b = <TestClassConfig as ClassConfig>::Int::one();
        let c = (<TestClassConfig as ClassConfig>::Int::one() - TestClassConfig::discriminant()) >> 2;
        Self { a, b, c }
    }

    fn is_zero(&self) -> bool {
        let a_check = self.a.is_one();
        let b_check = self.b.is_one();
        let c_check = self.c == ((<TestClassConfig as ClassConfig>::Int::one() - TestClassConfig::discriminant()) >> 2);
        a_check && b_check && c_check
    }
}

impl Zeroize for ClassGroup<TestClassConfig> {
    fn zeroize(&mut self) {
        unimplemented!();
    }
}

// Ops
impl Neg for ClassGroup<TestClassConfig> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            a: self.a,
            b: -self.b,
            c: self.c,
        }
    }
}

// Add
impl Add for ClassGroup<TestClassConfig> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, &rhs);
        r
    }
}

impl AddAssign for ClassGroup<TestClassConfig> {
    fn add_assign(&mut self, rhs: Self) {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, &rhs);
        *self = r;
    }
}

impl<'a> Add<&'a Self> for ClassGroup<TestClassConfig> {
    type Output = Self;

    fn add(self, rhs: &'a Self) -> Self::Output {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, rhs);
        r
    }
}

impl<'a> AddAssign<&'a Self> for ClassGroup<TestClassConfig> {
    fn add_assign(&mut self, rhs: &'a Self) {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, rhs);
        *self = r;
    }
}

impl<'a> Add<&'a mut Self> for ClassGroup<TestClassConfig> {
    type Output = Self;

    fn add(self, rhs: &'a mut Self) -> Self::Output {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, rhs);
        r
    }
}

impl<'a> AddAssign<&'a mut Self> for ClassGroup<TestClassConfig> {
    fn add_assign(&mut self, rhs: &'a mut Self) {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, rhs);
        *self = r;
    }
}

// Sub
impl Sub for ClassGroup<TestClassConfig> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, &rhs);
        -r
    }
}

impl SubAssign for ClassGroup<TestClassConfig> {
    fn sub_assign(&mut self, rhs: Self) {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, &rhs);
        *self = -r;
    }
}

impl<'a> Sub<&'a Self> for ClassGroup<TestClassConfig> {
    type Output = Self;

    fn sub(self, rhs: &'a Self) -> Self::Output {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, rhs);
        -r
    }
}

impl<'a> SubAssign<&'a Self> for ClassGroup<TestClassConfig> {
    fn sub_assign(&mut self, rhs: &'a Self) {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, rhs);
        *self = -r;
    }
}

impl<'a> Sub<&'a mut Self> for ClassGroup<TestClassConfig> {
    type Output = Self;

    fn sub(self, rhs: &'a mut Self) -> Self::Output {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, rhs);
        -r
    }
}

impl<'a> SubAssign<&'a mut Self> for ClassGroup<TestClassConfig> {
    fn sub_assign(&mut self, rhs: &'a mut Self) {
        let mut r: Self = Default::default();
        Self::nucomp(&mut r, &self, rhs);
        *self = -r;
    }
}

// Mul by ZZ
impl Mul<<TestClassConfig as ClassConfig>::Int> for ClassGroup<TestClassConfig> {
    type Output = Self;

    fn mul(self, rhs: <TestClassConfig as ClassConfig>::Int) -> Self::Output {
        let mut r: Self = Default::default();
        Self::nupow(&mut r, &self, &rhs);
        r
    }
}

impl MulAssign<<TestClassConfig as ClassConfig>::Int> for ClassGroup<TestClassConfig> {
    fn mul_assign(&mut self, rhs: <TestClassConfig as ClassConfig>::Int) {
        let mut r: Self = Default::default();
        Self::nupow(&mut r, &self, &rhs);
        *self = r;
    }
}

impl<'a> Mul<&'a <TestClassConfig as ClassConfig>::Int> for ClassGroup<TestClassConfig> {
    type Output = Self;

    fn mul(self, rhs: &'a <TestClassConfig as ClassConfig>::Int) -> Self::Output {
        let mut r: Self = Default::default();
        Self::nupow(&mut r, &self, rhs);
        r
    }
}

impl<'a> MulAssign<&'a <TestClassConfig as ClassConfig>::Int> for ClassGroup<TestClassConfig> {
    fn mul_assign(&mut self, rhs: &'a <TestClassConfig as ClassConfig>::Int) {
        let mut r: Self = Default::default();
        Self::nupow(&mut r, &self, rhs);
        *self = r;
    }
}

impl<'a> Mul<&'a mut <TestClassConfig as ClassConfig>::Int> for ClassGroup<TestClassConfig> {
    type Output = Self;

    fn mul(self, rhs: &'a mut <TestClassConfig as ClassConfig>::Int) -> Self::Output {
        let mut r: Self = Default::default();
        Self::nupow(&mut r, &self, rhs);
        r
    }
}
impl<'a> MulAssign<&'a mut <TestClassConfig as ClassConfig>::Int> for ClassGroup<TestClassConfig> {
    fn mul_assign(&mut self, rhs: &'a mut <TestClassConfig as ClassConfig>::Int) {
        let mut r: Self = Default::default();
        Self::nupow(&mut r, &self, rhs);
        *self = r;
    }
}

impl Sum for ClassGroup<TestClassConfig> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut r: Self = Default::default();
        for i in iter {
            r += i;
        }
        r
    }
}
impl<'a> Sum<&'a ClassGroup<TestClassConfig>> for ClassGroup<TestClassConfig> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut r: Self = Default::default();
        for i in iter {
            r += i;
        }
        r
    }
}