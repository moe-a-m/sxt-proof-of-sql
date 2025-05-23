use crate::base::{
    if_rayon,
    scalar::{Scalar, ScalarExt},
};
use alloc::vec::Vec;
use core::{iter::Sum, ops::Mul};
#[cfg(feature = "rayon")]
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

/// This operation takes the inner product of two slices. In other words, it does `a[0] * b[0] + a[1] * b[1] + ... + a[n] * b[n]`.
/// If one of the slices is longer than the other, the extra elements are ignored/considered to be 0.
pub fn inner_product<'a, F, T>(a: &[F], b: &'a [T]) -> F
where
    F: Sync + Send + Mul<Output = F> + Sum + Copy,
    &'a T: Into<F>,
    T: Sync,
{
    if_rayon!(a.par_iter().with_min_len(super::MIN_RAYON_LEN), a.iter())
        .zip(b)
        .map(|(&a, b)| a * b.into())
        .sum()
}

pub fn inner_product_ref_cast<F, T>(a: &[F], b: &[T]) -> T
where
    for<'a> &'a F: Into<T>,
    F: Send + Sync,
    T: Sync + Send + Mul<Output = T> + Sum + Copy,
{
    if_rayon!(a.par_iter().with_min_len(super::MIN_RAYON_LEN), a.iter())
        .zip(b)
        .map(|(a, b)| a.into() * *b)
        .sum()
}

/// Cannot use blanket impls for `Vec<u8>` because bytes might have different embeddings as scalars
pub fn inner_product_with_bytes<S: Scalar>(a: &[Vec<u8>], b: &[S]) -> S {
    if_rayon!(a.par_iter().with_min_len(super::MIN_RAYON_LEN), a.iter())
        .zip(b)
        .map(|(lhs_bytes, &rhs)| S::from_byte_slice_via_hash(lhs_bytes) * rhs)
        .sum()
}
