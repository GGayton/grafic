use std::hash::Hash;

use nohash_hasher::IsEnabled;

/// Types available for use as identities within the graph
pub trait Identity: IsEnabled + Eq + Hash + PartialEq + Clone + Copy{}

impl Identity for u8 {}
impl Identity for u16 {}
impl Identity for u32 {}
impl Identity for u64 {}
impl Identity for usize {}
impl Identity for i8 {}
impl Identity for i16 {}
impl Identity for i32 {}
impl Identity for i64 {}
impl Identity for isize {}

pub trait Scalar : Clone + Copy {}

//impl Scalar for f16 {}
impl Scalar for f32 {}
impl Scalar for f64 {}
//impl Scalar for f128 {}
impl Scalar for i8 {}
impl Scalar for i16 {}
impl Scalar for i32 {}
impl Scalar for i64 {}
impl Scalar for u8 {}
impl Scalar for u16 {}
impl Scalar for u32 {}
impl Scalar for u64 {}