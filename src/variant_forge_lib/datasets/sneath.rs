// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Auto-generated from `data/processed/sneath.csv`
//! Matrix: `Sneath`
//! Symmetric: true

use crate::models::{
    AminoAcid,
    DistanceMetric,
};

/// # Sneath Similarity Index Matrix
///
///  The Sneath matrix encodes the similarity between amino acids based on shared features
///  across multiple biochemical and structural properties. It is derived from binary feature
/// profiles  for each amino acid, such as polarity, aromaticity, size, and charge.
///
///  The Sneath index is a symmetric similarity matrix (higher values mean greater similarity),
///  rather than a distance matrix.
///
///  **Citation**:
///  Sneath, P. H. A. (1966). Relations between chemical structure and biological activity in
/// peptides.  *Journal of Theoretical Biology*, 12(2), 157–195. <https://doi.org/10.1016/0022-5193(66)90112-9>
///
///  **Value type**: `u16`
///
///  **Symmetry**: Yes (symmetric)
///
///  **Missing values**: None; all values are filled
///
///  **Amino Acid Row/Column Order**:
///  A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
#[derive(Debug, Copy, Clone, Default)]
pub struct Sneath;

impl Sneath {
    const MATRIX: [f64; 400] = [
        0f64, 37f64, 25f64, 30f64, 13f64, 26f64, 34f64, 9f64, 29f64, 17f64, 15f64, 26f64, 25f64,
        26f64, 16f64, 16f64, 20f64, 36f64, 34f64, 12f64, 37f64, 0f64, 31f64, 39f64, 36f64, 23f64,
        31f64, 43f64, 31f64, 34f64, 33f64, 14f64, 28f64, 34f64, 43f64, 37f64, 38f64, 36f64, 36f64,
        36f64, 25f64, 31f64, 0f64, 14f64, 19f64, 10f64, 19f64, 26f64, 24f64, 23f64, 20f64, 27f64,
        21f64, 24f64, 31f64, 15f64, 19f64, 32f64, 28f64, 23f64, 30f64, 39f64, 14f64, 0f64, 28f64,
        22f64, 7f64, 33f64, 35f64, 28f64, 25f64, 34f64, 31f64, 35f64, 40f64, 25f64, 29f64, 45f64,
        34f64, 28f64, 13f64, 36f64, 19f64, 28f64, 0f64, 22f64, 33f64, 21f64, 31f64, 26f64, 24f64,
        32f64, 17f64, 29f64, 25f64, 13f64, 19f64, 37f64, 34f64, 21f64, 26f64, 23f64, 10f64, 22f64,
        22f64, 0f64, 14f64, 32f64, 27f64, 24f64, 22f64, 21f64, 13f64, 24f64, 33f64, 21f64, 24f64,
        31f64, 29f64, 25f64, 34f64, 31f64, 19f64, 7f64, 33f64, 14f64, 0f64, 37f64, 27f64, 31f64,
        30f64, 26f64, 26f64, 35f64, 43f64, 29f64, 34f64, 43f64, 34f64, 31f64, 9f64, 43f64, 26f64,
        33f64, 21f64, 32f64, 37f64, 0f64, 34f64, 25f64, 24f64, 31f64, 34f64, 29f64, 17f64, 19f64,
        20f64, 39f64, 36f64, 19f64, 29f64, 31f64, 24f64, 35f64, 31f64, 27f64, 27f64, 34f64, 0f64,
        28f64, 25f64, 27f64, 30f64, 18f64, 36f64, 28f64, 34f64, 25f64, 23f64, 31f64, 17f64, 34f64,
        23f64, 28f64, 26f64, 24f64, 31f64, 25f64, 28f64, 0f64, 5f64, 24f64, 22f64, 22f64, 24f64,
        25f64, 21f64, 34f64, 34f64, 7f64, 15f64, 33f64, 20f64, 25f64, 24f64, 22f64, 30f64, 24f64,
        25f64, 5f64, 0f64, 23f64, 20f64, 19f64, 23f64, 23f64, 23f64, 30f64, 30f64, 9f64, 26f64,
        14f64, 27f64, 34f64, 32f64, 21f64, 26f64, 31f64, 27f64, 24f64, 23f64, 0f64, 24f64, 28f64,
        31f64, 31f64, 34f64, 34f64, 34f64, 26f64, 25f64, 28f64, 21f64, 31f64, 17f64, 13f64, 26f64,
        34f64, 30f64, 22f64, 20f64, 24f64, 0f64, 24f64, 31f64, 22f64, 25f64, 31f64, 32f64, 23f64,
        26f64, 34f64, 24f64, 35f64, 29f64, 24f64, 35f64, 29f64, 18f64, 22f64, 19f64, 28f64, 24f64,
        0f64, 27f64, 25f64, 28f64, 13f64, 13f64, 26f64, 16f64, 43f64, 31f64, 40f64, 25f64, 33f64,
        43f64, 17f64, 36f64, 24f64, 23f64, 31f64, 31f64, 27f64, 0f64, 24f64, 25f64, 37f64, 37f64,
        20f64, 16f64, 37f64, 15f64, 25f64, 13f64, 21f64, 29f64, 19f64, 28f64, 25f64, 23f64, 31f64,
        22f64, 25f64, 24f64, 0f64, 12f64, 35f64, 29f64, 20f64, 20f64, 38f64, 19f64, 29f64, 19f64,
        24f64, 34f64, 20f64, 34f64, 21f64, 23f64, 34f64, 25f64, 28f64, 25f64, 12f64, 0f64, 38f64,
        32f64, 17f64, 36f64, 36f64, 32f64, 45f64, 37f64, 31f64, 43f64, 39f64, 25f64, 34f64, 30f64,
        34f64, 31f64, 13f64, 37f64, 35f64, 38f64, 0f64, 21f64, 37f64, 34f64, 36f64, 28f64, 34f64,
        34f64, 29f64, 34f64, 36f64, 23f64, 34f64, 30f64, 34f64, 32f64, 13f64, 37f64, 29f64, 32f64,
        21f64, 0f64, 36f64, 12f64, 36f64, 23f64, 28f64, 21f64, 25f64, 31f64, 19f64, 31f64, 7f64,
        9f64, 26f64, 23f64, 26f64, 20f64, 20f64, 17f64, 37f64, 36f64, 0f64,
    ];

    /// Returns a reference to the full amino acid substitution matrix.
    ///
    /// This function provides direct access to the raw 20×20 distance matrix
    /// associated with this metric. The matrix is flattened into a one-dimensional
    /// array in **row-major order**, where the index of a pair `(i, j)` can be
    /// computed as `i * 20 + j`.
    ///
    /// The values represent pairwise distances between amino acids as defined
    /// by the metric. These values are `f64` values converted from the original
    /// input data. The matrix should generally be accessed through the
    /// [`lookup`](Self::lookup) method for semantic clarity and optionality.
    ///
    /// # Returns
    ///
    /// A reference to the static array of 400 `f64` values representing the
    /// amino acid substitution matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use variant_forge_lib::datasets::Sneath;
    ///
    /// let matrix = Sneath::matrix();
    /// let distance = matrix[0 * 20 + 1]; // A → R
    /// assert!(distance >= 0.0);
    /// ```
    #[must_use]
    pub const fn matrix() -> &'static [f64; 400] {
        &Self::MATRIX
    }
}

impl DistanceMetric for Sneath {
    fn name(&self) -> &'static str {
        "Sneath"
    }

    fn is_symmetric(&self) -> bool {
        true
    }

    fn lookup(&self, a: AminoAcid, b: AminoAcid) -> Option<f64> {
        let i = a.index() * 20 + b.index();
        let j = b.index() * 20 + a.index();
        let idx = if i <= j {
            i
        } else {
            j
        };
        Some(Self::MATRIX[idx])
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lookup_nonzero() {
        let metric = Sneath;
        let d = metric.lookup(AminoAcid::A, AminoAcid::V);
        assert!(d.is_some());
        assert!(d.unwrap() > 0.0);
    }
    #[test]
    fn test_symmetry() {
        let metric = Sneath;
        assert_eq!(
            metric.lookup(AminoAcid::A, AminoAcid::V),
            metric.lookup(AminoAcid::V, AminoAcid::A)
        );
    }
}
