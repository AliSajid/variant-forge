// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Auto-generated from `data/processed/epstein.csv`
//! Matrix: `Epstein`
//! Symmetric: false

use crate::models::{
    AminoAcid,
    DistanceMetric,
};

/// # Epstein Distance Matrix
///
///  The Epstein distance matrix is an asymmetric matrix encoding substitution preferences
///  based on experimental mutation frequency data. It captures evolutionary pressures
///  in a directional manner, reflecting how likely one amino acid is to mutate into another.
///
///  **Citation**:
///  Epstein, C. J. (1967). Non-randomness of Amino Acid Changes in the Evolution of Homologous Proteins. *Nature*, 215(5096), 355–359. <https://doi.org/10.1038/215355a0>
///
///  **Value type**: `f32`
///
///  **Symmetry**: No (asymmetric)
///
///  **Missing values**: Represented as `f32::NAN` or skipped with `"."` in the source
///
///  **Amino Acid Row/Column Order**:
///  A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
#[derive(Debug, Copy, Clone, Default)]
pub struct Epstein;

impl Epstein {
    const MATRIX: [f64; 400] = [
        0f64, 0.67f64, 0.62f64, 0.62f64, 0.22f64, 0.63f64, 0.63f64, 0.1f64, 0.47f64, 0.43f64,
        0.43f64, 0.63f64, 0.45f64, 0.5f64, 0.41f64, 0.4f64, 0.41f64, 0.49f64, 0.4f64, 0.41f64,
        0.62f64, 0f64, 0.08f64, 0.08f64, 0.81f64, 0.05f64, 0.05f64, 0.53f64, 0.2f64, 1f64, 1f64,
        0.05f64, 1f64, 1f64, 1.01f64, 0.24f64, 0.22f64, 0.8f64, 0.8f64, 1.01f64, 0.61f64, 0.08f64,
        0f64, 0f64, 0.8f64, 0.03f64, 0.03f64, 0.51f64, 0.21f64, 1f64, 1f64, 0.03f64, 1f64, 1f64,
        1f64, 0.21f64, 0.2f64, 0.81f64, 0.81f64, 1f64, 0.61f64, 0.08f64, 0f64, 0f64, 0.8f64,
        0.03f64, 0.03f64, 0.51f64, 0.21f64, 1f64, 1f64, 0.03f64, 1f64, 1f64, 1f64, 0.21f64, 0.2f64,
        0.81f64, 0.81f64, 1f64, 0.25f64, 0.82f64, 0.8f64, 0.8f64, 0f64, 0.81f64, 0.81f64, 0.31f64,
        0.62f64, 0.21f64, 0.21f64, 0.81f64, 0.22f64, 0.28f64, 0.2f64, 0.6f64, 0.6f64, 0.35f64,
        0.25f64, 0.2f64, 0.61f64, 0.05f64, 0.03f64, 0.03f64, 0.8f64, 0f64, 0f64, 0.52f64, 0.2f64,
        1f64, 1f64, 0f64, 1f64, 1f64, 1f64, 0.22f64, 0.21f64, 0.81f64, 0.8f64, 1f64, 0.61f64,
        0.05f64, 0.03f64, 0.03f64, 0.8f64, 0f64, 0f64, 0.52f64, 0.2f64, 1f64, 1f64, 0f64, 1f64,
        1f64, 1f64, 0.22f64, 0.21f64, 0.81f64, 0.8f64, 1f64, 0.1f64, 0.61f64, 0.54f64, 0.54f64,
        0.34f64, 0.56f64, 0.56f64, 0f64, 0.42f64, 0.54f64, 0.54f64, 0.56f64, 0.56f64, 0.61f64,
        0.52f64, 0.32f64, 0.34f64, 0.58f64, 0.5f64, 0.52f64, 0.42f64, 0.2f64, 0.21f64, 0.21f64,
        0.61f64, 0.2f64, 0.2f64, 0.34f64, 0f64, 1f64, 1f64, 0.2f64, 0.8f64, 0.8f64, 0.8f64, 0.1f64,
        0.08f64, 0.61f64, 0.6f64, 0.8f64, 0.43f64, 1.01f64, 1f64, 1f64, 0.2f64, 1f64, 1f64,
        0.51f64, 0.81f64, 0f64, 0f64, 1f64, 0.05f64, 0.15f64, 0.03f64, 0.8f64, 0.8f64, 0.36f64,
        0.28f64, 0.03f64, 0.43f64, 1.01f64, 1f64, 1f64, 0.2f64, 1f64, 1f64, 0.51f64, 0.81f64, 0f64,
        0f64, 1f64, 0.05f64, 0.15f64, 0.03f64, 0.8f64, 0.8f64, 0.36f64, 0.28f64, 0.03f64, 0.61f64,
        0.05f64, 0.03f64, 0.03f64, 0.8f64, 0f64, 0f64, 0.52f64, 0.2f64, 1f64, 1f64, 0f64, 1f64,
        1f64, 1f64, 0.22f64, 0.21f64, 0.81f64, 0.8f64, 1f64, 0.41f64, 1f64, 1f64, 1f64, 0.21f64,
        1f64, 1f64, 0.42f64, 0.8f64, 0.03f64, 0.03f64, 1f64, 0f64, 0.1f64, 0.1f64, 0.8f64, 0.8f64,
        0.32f64, 0.25f64, 0.1f64, 0.43f64, 1f64, 1f64, 1f64, 0.22f64, 1f64, 1f64, 0.53f64, 0.8f64,
        0.08f64, 0.08f64, 1f64, 0.05f64, 0f64, 0.1f64, 0.81f64, 0.81f64, 0.25f64, 0.21f64, 0.1f64,
        0.4f64, 1.02f64, 1f64, 1f64, 0.2f64, 1f64, 1f64, 0.5f64, 0.81f64, 0.05f64, 0.05f64, 1f64,
        0.1f64, 0.2f64, 0f64, 0.8f64, 0.8f64, 0.4f64, 0.32f64, 0f64, 0.4f64, 0.24f64, 0.2f64,
        0.2f64, 0.6f64, 0.21f64, 0.21f64, 0.3f64, 0.1f64, 0.8f64, 0.8f64, 0.21f64, 0.8f64, 0.81f64,
        0.8f64, 0f64, 0.03f64, 0.63f64, 0.62f64, 0.8f64, 0.4f64, 0.22f64, 0.2f64, 0.2f64, 0.6f64,
        0.21f64, 0.21f64, 0.31f64, 0.08f64, 0.8f64, 0.8f64, 0.21f64, 0.8f64, 0.81f64, 0.8f64,
        0.03f64, 0f64, 0.63f64, 0.61f64, 0.8f64, 0.3f64, 0.8f64, 0.81f64, 0.81f64, 0.18f64,
        0.81f64, 0.81f64, 0.39f64, 0.61f64, 0.25f64, 0.25f64, 0.81f64, 0.24f64, 0.21f64, 0.27f64,
        0.63f64, 0.63f64, 0f64, 0.05f64, 0.27f64, 0.27f64, 0.8f64, 0.81f64, 0.81f64, 0.13f64,
        0.8f64, 0.8f64, 0.36f64, 0.6f64, 0.22f64, 0.22f64, 0.8f64, 0.22f64, 0.2f64, 0.24f64,
        0.62f64, 0.61f64, 0.1f64, 0f64, 0.24f64, 0.4f64, 1.02f64, 1f64, 1f64, 0.2f64, 1f64, 1f64,
        0.5f64, 0.81f64, 0.05f64, 0.05f64, 1f64, 0.1f64, 0.2f64, 0f64, 0.8f64, 0.8f64, 0.4f64,
        0.32f64, 0f64,
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
    /// use variant_forge_lib::datasets::Epstein;
    ///
    /// let matrix = Epstein::matrix();
    /// let distance = matrix[0 * 20 + 1]; // A → R
    /// assert!(distance >= 0.0);
    /// ```
    #[must_use]
    pub const fn matrix() -> &'static [f64; 400] {
        &Self::MATRIX
    }
}

impl DistanceMetric for Epstein {
    fn name(&self) -> &'static str {
        "Epstein"
    }

    fn is_symmetric(&self) -> bool {
        false
    }

    fn lookup(&self, a: AminoAcid, b: AminoAcid) -> Option<f64> {
        let i = a.index() * 20 + b.index();
        let idx = i;
        Some(Self::MATRIX[idx])
    }
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lookup_nonzero() {
        let metric = Epstein;
        let d = metric.lookup(AminoAcid::A, AminoAcid::V);
        assert!(d.is_some());
        assert!(d.unwrap() > 0.0);
    }
}
