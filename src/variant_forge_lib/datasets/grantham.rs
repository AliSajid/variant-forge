// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Auto-generated from `data/processed/grantham.csv`
//! Matrix: `Grantham`
//! Symmetric: true

use crate::models::{
    AminoAcid,
    DistanceMetric,
};

/// # Grantham Distance Matrix
///
///  The Grantham distance matrix was introduced in 1974 to quantify differences between amino acids
///  based on their physicochemical properties: composition, polarity, and molecular volume.
///
///  It is a symmetric matrix of integer values representing distances in multidimensional property
/// space.
///
///  **Citation**:
///  Grantham, R. (1974). Amino acid difference formula to help explain protein evolution. *Science*, 185(4154), 862-864. <https://doi.org/10.1126/science.185.4154.862>
///
///  **Value type**: `u16`
///
///  **Symmetry**: Yes (symmetric)
///
///  **Missing values**: Represented as `65535` (`u16::MAX`)
///
///  **Amino Acid Row/Column Order**:
///  A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
#[derive(Debug, Copy, Clone, Default)]
pub struct Grantham;

impl Grantham {
    const MATRIX: [f64; 400] = [
        0f64, 112f64, 111f64, 126f64, 195f64, 91f64, 107f64, 60f64, 86f64, 94f64, 96f64, 106f64,
        84f64, 113f64, 27f64, 99f64, 58f64, 148f64, 112f64, 64f64, 112f64, 0f64, 86f64, 96f64,
        180f64, 43f64, 54f64, 125f64, 29f64, 97f64, 102f64, 26f64, 91f64, 97f64, 103f64, 110f64,
        71f64, 101f64, 77f64, 96f64, 111f64, 86f64, 0f64, 23f64, 139f64, 46f64, 42f64, 80f64,
        68f64, 149f64, 153f64, 94f64, 142f64, 158f64, 91f64, 46f64, 65f64, 174f64, 143f64, 133f64,
        126f64, 96f64, 23f64, 0f64, 154f64, 61f64, 45f64, 94f64, 81f64, 168f64, 172f64, 101f64,
        160f64, 177f64, 108f64, 65f64, 85f64, 181f64, 160f64, 152f64, 195f64, 180f64, 139f64,
        154f64, 0f64, 154f64, 170f64, 159f64, 174f64, 198f64, 198f64, 202f64, 196f64, 205f64,
        169f64, 112f64, 149f64, 215f64, 194f64, 192f64, 91f64, 43f64, 46f64, 61f64, 154f64, 0f64,
        29f64, 87f64, 24f64, 109f64, 113f64, 53f64, 101f64, 116f64, 76f64, 68f64, 42f64, 130f64,
        99f64, 96f64, 107f64, 54f64, 42f64, 45f64, 170f64, 29f64, 0f64, 98f64, 40f64, 134f64,
        138f64, 56f64, 126f64, 140f64, 93f64, 80f64, 65f64, 152f64, 122f64, 121f64, 60f64, 125f64,
        80f64, 94f64, 159f64, 87f64, 98f64, 0f64, 98f64, 135f64, 138f64, 127f64, 127f64, 153f64,
        42f64, 56f64, 59f64, 184f64, 147f64, 109f64, 86f64, 29f64, 68f64, 81f64, 174f64, 24f64,
        40f64, 98f64, 0f64, 94f64, 99f64, 32f64, 87f64, 100f64, 77f64, 89f64, 47f64, 115f64, 83f64,
        84f64, 94f64, 97f64, 149f64, 168f64, 198f64, 109f64, 134f64, 135f64, 94f64, 0f64, 5f64,
        102f64, 10f64, 21f64, 95f64, 142f64, 89f64, 61f64, 33f64, 29f64, 96f64, 102f64, 153f64,
        172f64, 198f64, 113f64, 138f64, 138f64, 99f64, 5f64, 0f64, 107f64, 15f64, 22f64, 98f64,
        145f64, 92f64, 61f64, 36f64, 32f64, 106f64, 26f64, 94f64, 101f64, 202f64, 53f64, 56f64,
        127f64, 32f64, 102f64, 107f64, 0f64, 95f64, 102f64, 103f64, 121f64, 78f64, 110f64, 85f64,
        97f64, 84f64, 91f64, 142f64, 160f64, 196f64, 101f64, 126f64, 127f64, 87f64, 10f64, 15f64,
        95f64, 0f64, 28f64, 87f64, 135f64, 81f64, 67f64, 36f64, 21f64, 113f64, 97f64, 158f64,
        177f64, 205f64, 116f64, 140f64, 153f64, 100f64, 21f64, 22f64, 102f64, 28f64, 0f64, 114f64,
        155f64, 103f64, 40f64, 22f64, 50f64, 27f64, 103f64, 91f64, 108f64, 169f64, 76f64, 93f64,
        42f64, 77f64, 95f64, 98f64, 103f64, 87f64, 114f64, 0f64, 74f64, 38f64, 147f64, 110f64,
        68f64, 99f64, 110f64, 46f64, 65f64, 112f64, 68f64, 80f64, 56f64, 89f64, 142f64, 145f64,
        121f64, 135f64, 155f64, 74f64, 0f64, 58f64, 177f64, 144f64, 124f64, 58f64, 71f64, 65f64,
        85f64, 149f64, 42f64, 65f64, 59f64, 47f64, 89f64, 92f64, 78f64, 81f64, 103f64, 38f64,
        58f64, 0f64, 128f64, 92f64, 69f64, 148f64, 101f64, 174f64, 181f64, 215f64, 130f64, 152f64,
        184f64, 115f64, 61f64, 61f64, 110f64, 67f64, 40f64, 147f64, 177f64, 128f64, 0f64, 37f64,
        88f64, 112f64, 77f64, 143f64, 160f64, 194f64, 99f64, 122f64, 147f64, 83f64, 33f64, 36f64,
        85f64, 36f64, 22f64, 110f64, 144f64, 92f64, 37f64, 0f64, 55f64, 64f64, 96f64, 133f64,
        152f64, 192f64, 96f64, 121f64, 109f64, 84f64, 29f64, 32f64, 97f64, 21f64, 50f64, 68f64,
        124f64, 69f64, 88f64, 55f64, 0f64,
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
    /// use variant_forge_lib::datasets::Grantham;
    ///
    /// let matrix = Grantham::matrix();
    /// let distance = matrix[0 * 20 + 1]; // A → R
    /// assert!(distance >= 0.0);
    /// ```
    #[must_use]
    pub const fn matrix() -> &'static [f64; 400] {
        &Self::MATRIX
    }
}

impl DistanceMetric for Grantham {
    fn name(&self) -> &'static str {
        "Grantham"
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
        let metric = Grantham;
        let d = metric.lookup(AminoAcid::A, AminoAcid::V);
        assert!(d.is_some());
        assert!(d.unwrap() > 0.0);
    }
    #[test]
    fn test_symmetry() {
        let metric = Grantham;
        assert_eq!(
            metric.lookup(AminoAcid::A, AminoAcid::V),
            metric.lookup(AminoAcid::V, AminoAcid::A)
        );
    }
}
