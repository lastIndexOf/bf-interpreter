#[cfg(feature = "ir")]
pub mod ir;
#[cfg(feature = "ir")]
pub use ir as vm;

#[cfg(not(feature = "ir"))]
pub mod base;
#[cfg(not(feature = "ir"))]
pub use base as vm;
