extern crate xxhash;

#[inline]
pub fn displace(f1: u32, f2: u32, d1: u32, d2: u32) -> u32 {
    d2 + f1 * d1 + f2
}

#[inline]
fn split(hash: u64) -> (u32, u32, u32) {
    let bits = 21;
    let mask = (1 << bits) - 1;

    ((hash & mask) as u32,
     ((hash >> bits) & mask) as u32,
     ((hash >> (2 * bits)) & mask) as u32)
}

/// A trait implemented by types which can be used in PHF data structures
pub trait PhfHash {
    /// Hashes the value of `self`, factoring in a seed
    fn phf_hash(&self, seed: u64) -> (u32, u32, u32);
}

impl<'a> PhfHash for &'a str {
    #[inline]
    fn phf_hash(&self, seed: u64) -> (u32, u32, u32) {
        split(xxhash::hash_with_seed(seed, self))
    }
}

impl<'a> PhfHash for &'a [u8] {
    #[inline]
    fn phf_hash(&self, seed: u64) -> (u32, u32, u32) {
        split(xxhash::oneshot(*self, seed))
    }
}

macro_rules! sip_impl(
    ($t:ty) => (
        impl PhfHash for $t {
            #[inline]
            fn phf_hash(&self, seed: u64) -> (u32, u32, u32) {
                split(xxhash::hash_with_seed(seed, self))
            }
        }
    )
)

sip_impl!(u8)
sip_impl!(i8)
sip_impl!(u16)
sip_impl!(i16)
sip_impl!(u32)
sip_impl!(i32)
sip_impl!(u64)
sip_impl!(i64)
sip_impl!(char)
sip_impl!(bool)
