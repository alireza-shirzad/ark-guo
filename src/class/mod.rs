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

use rug::Integer as RugInteger;
use crate::{AdditiveGroup, Integer};

// Implement AdditiveGroup for Class group
#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ClassGroup {
    a: RugInteger,
    b: RugInteger,
    c: RugInteger,
}

// ClassConfig - Specs for group - discriminants
// ClassGroup generic on config?