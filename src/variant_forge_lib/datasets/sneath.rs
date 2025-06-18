//! Auto-generated from data/processed/sneath.csv
//! Matrix: Sneath
//! Symmetric: false
use crate::{
    AminoAcid,
    DistanceMetric,
};

/// # Sneath Similarity Index Matrix
///
/// The Sneath matrix encodes the similarity between amino acids based on shared features
/// across multiple biochemical and structural properties. It is derived from binary feature
/// profiles for each amino acid, such as polarity, aromaticity, size, and charge.
///
/// The Sneath index is a symmetric similarity matrix (higher values mean greater similarity),
/// rather than a distance matrix.
///
/// **Citation**:
/// Sneath, P. H. A. (1966). Relations between chemical structure and biological activity in
/// peptides. *Journal of Theoretical Biology*, 12(2), 157–195. <https://doi.org/10.1016/0022-5193(66)90112-9>
///
/// **Value type**: `u16`
///
/// **Symmetry**: Yes (symmetric)
///
/// **Missing values**: None; all values are filled
///
/// **Amino Acid Row/Column Order**:
/// A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
#[derive(Debug, Copy, Clone)]
pub struct Sneath;

impl Sneath {
    const MATRIX: [u16; 400] = [
        0, 37, 25, 30, 13, 26, 34, 9, 29, 17, 15, 26, 25, 26, 16, 16, 20, 36, 34, 12, 37, 0, 31,
        39, 36, 23, 31, 43, 31, 34, 33, 14, 28, 34, 43, 37, 38, 36, 36, 36, 25, 31, 0, 14, 19, 10,
        19, 26, 24, 23, 20, 27, 21, 24, 31, 15, 19, 32, 28, 23, 30, 39, 14, 0, 28, 22, 7, 33, 35,
        28, 25, 34, 31, 35, 40, 25, 29, 45, 34, 28, 13, 36, 19, 28, 0, 22, 33, 21, 31, 26, 24, 32,
        17, 29, 25, 13, 19, 37, 34, 21, 26, 23, 10, 22, 22, 0, 14, 32, 27, 24, 22, 21, 13, 24, 33,
        21, 24, 31, 29, 25, 34, 31, 19, 7, 33, 14, 0, 37, 27, 31, 30, 26, 26, 35, 43, 29, 34, 43,
        34, 31, 9, 43, 26, 33, 21, 32, 37, 0, 34, 25, 24, 31, 34, 29, 17, 19, 20, 39, 36, 19, 29,
        31, 24, 35, 31, 27, 27, 34, 0, 28, 25, 27, 30, 18, 36, 28, 34, 25, 23, 31, 17, 34, 23, 28,
        26, 24, 31, 25, 28, 0, 5, 24, 22, 22, 24, 25, 21, 34, 34, 7, 15, 33, 20, 25, 24, 22, 30,
        24, 25, 5, 0, 23, 20, 19, 23, 23, 23, 30, 30, 9, 26, 14, 27, 34, 32, 21, 26, 31, 27, 24,
        23, 0, 24, 28, 31, 31, 34, 34, 34, 26, 25, 28, 21, 31, 17, 13, 26, 34, 30, 22, 20, 24, 0,
        24, 31, 22, 25, 31, 32, 23, 26, 34, 24, 35, 29, 24, 35, 29, 18, 22, 19, 28, 24, 0, 27, 25,
        28, 13, 13, 26, 16, 43, 31, 40, 25, 33, 43, 17, 36, 24, 23, 31, 31, 27, 0, 24, 25, 37, 37,
        20, 16, 37, 15, 25, 13, 21, 29, 19, 28, 25, 23, 31, 22, 25, 24, 0, 12, 35, 29, 20, 20, 38,
        19, 29, 19, 24, 34, 20, 34, 21, 23, 34, 25, 28, 25, 12, 0, 38, 32, 17, 36, 36, 32, 45, 37,
        31, 43, 39, 25, 34, 30, 34, 31, 13, 37, 35, 38, 0, 21, 37, 34, 36, 28, 34, 34, 29, 34, 36,
        23, 34, 30, 34, 32, 13, 37, 29, 32, 21, 0, 36, 12, 36, 23, 28, 21, 25, 31, 19, 31, 7, 9,
        26, 23, 26, 20, 20, 17, 37, 36, 0,
    ];

    /// Returns a reference to the flattened 20×20 distance matrix for the `Sneath` dataset.
    ///
    /// The matrix contains `u16` values stored in row-major order.
    /// Use `lookup(a, b)` to retrieve distances between two amino acids using
    /// the correct symmetric or asymmetric logic.
    ///
    /// # Returns
    /// A reference to a static array of length 400 representing the full matrix.
    ///
    /// # Example
    /// ```
    /// use variant_forge_lib::datasets::sneath::Sneath;
    ///
    /// let matrix = Sneath::matrix();
    /// let distance = matrix[0 * 20 + 1]; // A → R
    /// ```
    #[must_use]
    pub const fn matrix() -> &'static [u16; 400] {
        &Self::MATRIX
    }
}

impl DistanceMetric for Sneath {
    type Value = u16;

    fn name(&self) -> &'static str {
        "Sneath"
    }

    fn is_symmetric(&self) -> bool {
        false
    }

    fn lookup(&self, a: AminoAcid, b: AminoAcid) -> Option<Self::Value> {
        let i = a.index() * 20 + b.index();
        let idx = i;
        Some(Self::MATRIX[idx])
    }
}
