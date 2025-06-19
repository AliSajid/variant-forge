// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Auto-generated from `data/processed/miyata.csv`
//! Matrix: `Miyata`
//! Symmetric: true

use crate::models::{
    AminoAcid,
    DistanceMetric,
};

/// # Miyata Distance Matrix
///
///  The Miyata matrix uses a continuous float-based approach to encode physicochemical differences
///  between amino acids, accounting for both volume and polarity differences.
///  This matrix is symmetric and reflects molecular dissimilarity.
///
///  **Citation**:
///  Miyata, T., Miyazawa, S., & Yasunaga, T. (1979). Two types of amino acid substitutions in protein evolution. *Journal of Molecular Evolution*, 12(3), 219–236. <https://doi.org/10.1007/BF01732340>
///
///  **Value type**: `f32`
///
///  **Symmetry**: Yes (symmetric)
///
///  **Missing values**: Represented as `f32::NAN` or skipped with `"."` in the source
///
///  **Amino Acid Row/Column Order**:
///  A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
#[derive(Debug, Copy, Clone, Default)]
pub struct Miyata;

impl Miyata {
    const MATRIX: [f64; 400] = [
        0f64, 2.92f64, 1.78f64, 2.37f64, 1.39f64, 1.92f64, 2.46f64, 0.91f64, 2.17f64, 2.69f64,
        2.76f64, 2.96f64, 2.42f64, 3.23f64, 0.06f64, 0.51f64, 0.9f64, 4.23f64, 3.18f64, 1.85f64,
        2.92f64, 0f64, 2.04f64, 2.34f64, 3.06f64, 1.13f64, 1.45f64, 3.58f64, 0.82f64, 2.49f64,
        2.62f64, 0.4f64, 2.29f64, 2.47f64, 2.9f64, 2.74f64, 2.03f64, 2.72f64, 2.02f64, 2.43f64,
        1.78f64, 2.04f64, 0f64, 0.65f64, 2.83f64, 0.99f64, 0.85f64, 1.96f64, 1.29f64, 3.37f64,
        3.49f64, 1.84f64, 3.08f64, 3.7f64, 1.8f64, 1.31f64, 1.4f64, 4.39f64, 3.42f64, 2.76f64,
        2.37f64, 2.34f64, 0.65f64, 0f64, 3.48f64, 1.47f64, 0.9f64, 2.37f64, 1.72f64, 3.98f64,
        4.1f64, 2.05f64, 3.69f64, 4.27f64, 2.4f64, 1.87f64, 2.05f64, 4.88f64, 3.95f64, 3.4f64,
        1.39f64, 3.06f64, 2.83f64, 3.48f64, 0f64, 2.48f64, 3.26f64, 2.22f64, 2.56f64, 1.63f64,
        1.65f64, 3.27f64, 1.46f64, 2.24f64, 1.33f64, 2.84f64, 1.45f64, 3.34f64, 2.38f64, 0.86f64,
        1.92f64, 1.13f64, 0.99f64, 1.47f64, 2.48f64, 0f64, 0.84f64, 2.48f64, 0.32f64, 2.57f64,
        2.7f64, 1.06f64, 2.3f64, 2.81f64, 1.92f64, 1.65f64, 1.12f64, 3.42f64, 2.48f64, 2.13f64,
        2.46f64, 1.45f64, 0.85f64, 0.9f64, 3.26f64, 0.84f64, 0f64, 2.78f64, 0.96f64, 3.39f64,
        3.53f64, 1.14f64, 3.13f64, 3.59f64, 2.48f64, 2.06f64, 1.83f64, 4.08f64, 3.22f64, 2.97f64,
        0.91f64, 3.58f64, 1.96f64, 2.37f64, 2.22f64, 2.48f64, 2.78f64, 0f64, 2.78f64, 3.6f64,
        3.67f64, 3.54f64, 3.34f64, 4.14f64, 0.97f64, 0.85f64, 1.7f64, 5.13f64, 4.08f64, 2.76f64,
        2.17f64, 0.82f64, 1.29f64, 1.72f64, 2.56f64, 0.32f64, 0.96f64, 2.78f64, 0f64, 2.45f64,
        2.59f64, 0.79f64, 2.19f64, 2.63f64, 2.15f64, 1.94f64, 1.32f64, 3.16f64, 2.27f64, 2.11f64,
        2.69f64, 2.49f64, 3.37f64, 3.98f64, 1.63f64, 2.57f64, 3.39f64, 3.6f64, 2.45f64, 0f64,
        0.14f64, 2.84f64, 0.29f64, 0.61f64, 2.62f64, 2.95f64, 2.14f64, 1.72f64, 0.86f64, 0.85f64,
        2.76f64, 2.62f64, 3.49f64, 4.1f64, 1.65f64, 2.7f64, 3.53f64, 3.67f64, 2.59f64, 0.14f64,
        0f64, 2.98f64, 0.41f64, 0.63f64, 2.7f64, 3.04f64, 2.25f64, 1.73f64, 0.94f64, 0.91f64,
        2.96f64, 0.4f64, 1.84f64, 2.05f64, 3.27f64, 1.06f64, 1.14f64, 3.54f64, 0.79f64, 2.84f64,
        2.98f64, 0f64, 2.63f64, 2.85f64, 2.94f64, 2.71f64, 2.1f64, 3.11f64, 2.42f64, 2.7f64,
        2.42f64, 2.29f64, 3.08f64, 3.69f64, 1.46f64, 2.3f64, 3.13f64, 3.34f64, 2.19f64, 0.29f64,
        0.41f64, 2.63f64, 0f64, 0.82f64, 2.36f64, 2.67f64, 1.86f64, 1.89f64, 0.93f64, 0.62f64,
        3.23f64, 2.47f64, 3.7f64, 4.27f64, 2.24f64, 2.81f64, 3.59f64, 4.14f64, 2.63f64, 0.61f64,
        0.63f64, 2.85f64, 0.82f64, 0f64, 3.17f64, 3.45f64, 2.6f64, 1.11f64, 0.48f64, 1.43f64,
        0.06f64, 2.9f64, 1.8f64, 2.4f64, 1.33f64, 1.92f64, 2.48f64, 0.97f64, 2.15f64, 2.62f64,
        2.7f64, 2.94f64, 2.36f64, 3.17f64, 0f64, 0.56f64, 0.87f64, 4.17f64, 3.12f64, 1.79f64,
        0.51f64, 2.74f64, 1.31f64, 1.87f64, 2.84f64, 1.65f64, 2.06f64, 0.85f64, 1.94f64, 2.95f64,
        3.04f64, 2.71f64, 2.67f64, 3.45f64, 0.56f64, 0f64, 0.89f64, 4.38f64, 3.33f64, 2.15f64,
        0.9f64, 2.03f64, 1.4f64, 2.05f64, 1.45f64, 1.12f64, 1.83f64, 1.7f64, 1.32f64, 2.14f64,
        2.25f64, 2.1f64, 1.86f64, 2.6f64, 0.87f64, 0.89f64, 0f64, 3.5f64, 2.45f64, 1.42f64,
        4.23f64, 2.72f64, 4.39f64, 4.88f64, 3.34f64, 3.42f64, 4.08f64, 5.13f64, 3.16f64, 1.72f64,
        1.73f64, 3.11f64, 1.89f64, 1.11f64, 4.17f64, 4.38f64, 3.5f64, 0f64, 1.06f64, 2.51f64,
        3.18f64, 2.02f64, 3.42f64, 3.95f64, 2.38f64, 2.48f64, 3.22f64, 4.08f64, 2.27f64, 0.86f64,
        0.94f64, 2.42f64, 0.93f64, 0.48f64, 3.12f64, 3.33f64, 2.45f64, 1.06f64, 0f64, 1.52f64,
        1.85f64, 2.43f64, 2.76f64, 3.4f64, 0.86f64, 2.13f64, 2.97f64, 2.76f64, 2.11f64, 0.85f64,
        0.91f64, 2.7f64, 0.62f64, 1.43f64, 1.79f64, 2.15f64, 1.42f64, 2.51f64, 1.52f64, 0f64,
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
    /// use variant_forge_lib::datasets::Miyata;
    ///
    /// let matrix = Miyata::matrix();
    /// let distance = matrix[0 * 20 + 1]; // A → R
    /// assert!(distance >= 0.0);
    /// ```
    #[must_use]
    pub const fn matrix() -> &'static [f64; 400] {
        &Self::MATRIX
    }
}

impl DistanceMetric for Miyata {
    fn name(&self) -> &'static str {
        "Miyata"
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
        let metric = Miyata;
        let d = metric.lookup(AminoAcid::A, AminoAcid::V);
        assert!(d.is_some());
        assert!(d.unwrap() > 0.0);
    }
    #[test]
    fn test_symmetry() {
        let metric = Miyata;
        assert_eq!(
            metric.lookup(AminoAcid::A, AminoAcid::V),
            metric.lookup(AminoAcid::V, AminoAcid::A)
        );
    }
}
