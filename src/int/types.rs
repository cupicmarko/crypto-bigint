//! Selection of [`Int`] types.
//! todo: replace with macro implementation once serde is set up.

use crate::Int;

#[cfg(feature = "u64")]
/// Signed bit integer.
pub type I64 = Int<1>;

#[cfg(feature = "u64")]
/// Signed bit integer.
pub type I128 = Int<2>;

#[cfg(feature = "u64")]
/// Signed bit integer.
pub type I256 = Int<4>;

#[cfg(feature = "u64")]
/// Signed bit integer.
pub type I512 = Int<8>;

#[cfg(feature = "u64")]
/// Signed bit integer.
pub type I1024 = Int<16>;

#[cfg(feature = "u64")]
/// Signed bit integer.
pub type I2048 = Int<32>;

#[cfg(feature = "u64")]
/// Signed bit integer.
pub type I4096 = Int<64>;

#[cfg(not(feature = "u64"))]
/// Signed bit integer.
pub type I64 = Int<2>;

#[cfg(not(feature = "u64"))]
/// Signed bit integer.
pub type I128 = Int<4>;

#[cfg(not(feature = "u64"))]
/// Signed bit integer.
pub type I256 = Int<8>;

#[cfg(not(feature = "u64"))]
/// Signed bit integer.
pub type I512 = Int<16>;

#[cfg(not(feature = "u64"))]
/// Signed bit integer.
pub type I1024 = Int<32>;

#[cfg(not(feature = "u64"))]
/// Signed bit integer.
pub type I2048 = Int<64>;

#[cfg(not(feature = "u64"))]
/// Signed bit integer.
pub type I4096 = Int<128>;
