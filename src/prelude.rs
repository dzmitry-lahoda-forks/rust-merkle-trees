//! a way to have one place to be std vs not,
//! and with per blockchain integration and tuning (fits well into your blockchain)
pub use sp_std::{marker::PhantomData, hash::{Hasher, Hash}};
pub use sp_core::RuntimeDebug;