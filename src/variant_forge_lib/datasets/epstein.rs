//! Auto-generated from data/processed/epstein.csv
//! Matrix: Epstein
//! Symmetric: false
use crate::{
    AminoAcid,
    DistanceMetric,
};

/// # Epstein Distance Matrix
///
/// The Epstein distance matrix is an asymmetric matrix encoding substitution preferences
/// based on experimental mutation frequency data. It captures evolutionary pressures
/// in a directional manner, reflecting how likely one amino acid is to mutate into another.
///
/// **Citation**:
/// Epstein, C. J. (1967). Non-randomness of Amino Acid Changes in the Evolution of Homologous Proteins. *Nature*, 215(5096), 355–359. <https://doi.org/10.1038/215355a0>
///
/// **Value type**: `f32`
///
/// **Symmetry**: No (asymmetric)
///
/// **Missing values**: Represented as `f32::NAN` or skipped with `"."` in the source
///
/// **Amino Acid Row/Column Order**:
/// A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
#[derive(Debug, Copy, Clone)]
pub struct Epstein;

impl Epstein {
    const MATRIX: [f32; 400] = [
        0f32, 0.67f32, 0.62f32, 0.62f32, 0.22f32, 0.63f32, 0.63f32, 0.1f32, 0.47f32, 0.43f32,
        0.43f32, 0.63f32, 0.45f32, 0.5f32, 0.41f32, 0.4f32, 0.41f32, 0.49f32, 0.4f32, 0.41f32,
        0.62f32, 0f32, 0.08f32, 0.08f32, 0.81f32, 0.05f32, 0.05f32, 0.53f32, 0.2f32, 1f32, 1f32,
        0.05f32, 1f32, 1f32, 1.01f32, 0.24f32, 0.22f32, 0.8f32, 0.8f32, 1.01f32, 0.61f32, 0.08f32,
        0f32, 0f32, 0.8f32, 0.03f32, 0.03f32, 0.51f32, 0.21f32, 1f32, 1f32, 0.03f32, 1f32, 1f32,
        1f32, 0.21f32, 0.2f32, 0.81f32, 0.81f32, 1f32, 0.61f32, 0.08f32, 0f32, 0f32, 0.8f32,
        0.03f32, 0.03f32, 0.51f32, 0.21f32, 1f32, 1f32, 0.03f32, 1f32, 1f32, 1f32, 0.21f32, 0.2f32,
        0.81f32, 0.81f32, 1f32, 0.25f32, 0.82f32, 0.8f32, 0.8f32, 0f32, 0.81f32, 0.81f32, 0.31f32,
        0.62f32, 0.21f32, 0.21f32, 0.81f32, 0.22f32, 0.28f32, 0.2f32, 0.6f32, 0.6f32, 0.35f32,
        0.25f32, 0.2f32, 0.61f32, 0.05f32, 0.03f32, 0.03f32, 0.8f32, 0f32, 0f32, 0.52f32, 0.2f32,
        1f32, 1f32, 0f32, 1f32, 1f32, 1f32, 0.22f32, 0.21f32, 0.81f32, 0.8f32, 1f32, 0.61f32,
        0.05f32, 0.03f32, 0.03f32, 0.8f32, 0f32, 0f32, 0.52f32, 0.2f32, 1f32, 1f32, 0f32, 1f32,
        1f32, 1f32, 0.22f32, 0.21f32, 0.81f32, 0.8f32, 1f32, 0.1f32, 0.61f32, 0.54f32, 0.54f32,
        0.34f32, 0.56f32, 0.56f32, 0f32, 0.42f32, 0.54f32, 0.54f32, 0.56f32, 0.56f32, 0.61f32,
        0.52f32, 0.32f32, 0.34f32, 0.58f32, 0.5f32, 0.52f32, 0.42f32, 0.2f32, 0.21f32, 0.21f32,
        0.61f32, 0.2f32, 0.2f32, 0.34f32, 0f32, 1f32, 1f32, 0.2f32, 0.8f32, 0.8f32, 0.8f32, 0.1f32,
        0.08f32, 0.61f32, 0.6f32, 0.8f32, 0.43f32, 1.01f32, 1f32, 1f32, 0.2f32, 1f32, 1f32,
        0.51f32, 0.81f32, 0f32, 0f32, 1f32, 0.05f32, 0.15f32, 0.03f32, 0.8f32, 0.8f32, 0.36f32,
        0.28f32, 0.03f32, 0.43f32, 1.01f32, 1f32, 1f32, 0.2f32, 1f32, 1f32, 0.51f32, 0.81f32, 0f32,
        0f32, 1f32, 0.05f32, 0.15f32, 0.03f32, 0.8f32, 0.8f32, 0.36f32, 0.28f32, 0.03f32, 0.61f32,
        0.05f32, 0.03f32, 0.03f32, 0.8f32, 0f32, 0f32, 0.52f32, 0.2f32, 1f32, 1f32, 0f32, 1f32,
        1f32, 1f32, 0.22f32, 0.21f32, 0.81f32, 0.8f32, 1f32, 0.41f32, 1f32, 1f32, 1f32, 0.21f32,
        1f32, 1f32, 0.42f32, 0.8f32, 0.03f32, 0.03f32, 1f32, 0f32, 0.1f32, 0.1f32, 0.8f32, 0.8f32,
        0.32f32, 0.25f32, 0.1f32, 0.43f32, 1f32, 1f32, 1f32, 0.22f32, 1f32, 1f32, 0.53f32, 0.8f32,
        0.08f32, 0.08f32, 1f32, 0.05f32, 0f32, 0.1f32, 0.81f32, 0.81f32, 0.25f32, 0.21f32, 0.1f32,
        0.4f32, 1.02f32, 1f32, 1f32, 0.2f32, 1f32, 1f32, 0.5f32, 0.81f32, 0.05f32, 0.05f32, 1f32,
        0.1f32, 0.2f32, 0f32, 0.8f32, 0.8f32, 0.4f32, 0.32f32, 0f32, 0.4f32, 0.24f32, 0.2f32,
        0.2f32, 0.6f32, 0.21f32, 0.21f32, 0.3f32, 0.1f32, 0.8f32, 0.8f32, 0.21f32, 0.8f32, 0.81f32,
        0.8f32, 0f32, 0.03f32, 0.63f32, 0.62f32, 0.8f32, 0.4f32, 0.22f32, 0.2f32, 0.2f32, 0.6f32,
        0.21f32, 0.21f32, 0.31f32, 0.08f32, 0.8f32, 0.8f32, 0.21f32, 0.8f32, 0.81f32, 0.8f32,
        0.03f32, 0f32, 0.63f32, 0.61f32, 0.8f32, 0.3f32, 0.8f32, 0.81f32, 0.81f32, 0.18f32,
        0.81f32, 0.81f32, 0.39f32, 0.61f32, 0.25f32, 0.25f32, 0.81f32, 0.24f32, 0.21f32, 0.27f32,
        0.63f32, 0.63f32, 0f32, 0.05f32, 0.27f32, 0.27f32, 0.8f32, 0.81f32, 0.81f32, 0.13f32,
        0.8f32, 0.8f32, 0.36f32, 0.6f32, 0.22f32, 0.22f32, 0.8f32, 0.22f32, 0.2f32, 0.24f32,
        0.62f32, 0.61f32, 0.1f32, 0f32, 0.24f32, 0.4f32, 1.02f32, 1f32, 1f32, 0.2f32, 1f32, 1f32,
        0.5f32, 0.81f32, 0.05f32, 0.05f32, 1f32, 0.1f32, 0.2f32, 0f32, 0.8f32, 0.8f32, 0.4f32,
        0.32f32, 0f32,
    ];

    /// Returns a reference to the flattened 20×20 distance matrix for the `Epstein` dataset.
    ///
    /// The matrix contains `f32` values stored in row-major order.
    /// Use `lookup(a, b)` to retrieve distances between two amino acids using
    /// the correct symmetric or asymmetric logic.
    ///
    /// # Returns
    /// A reference to a static array of length 400 representing the full matrix.
    ///
    /// # Example
    /// ```
    /// use variant_forge_lib::datasets::epstein::Epstein;
    ///
    /// let matrix = Epstein::matrix();
    /// let distance = matrix[0 * 20 + 1]; // A → R
    /// ```
    #[must_use]
    pub const fn matrix() -> &'static [f32; 400] {
        &Self::MATRIX
    }
}

impl DistanceMetric for Epstein {
    type Value = f32;

    fn name(&self) -> &'static str {
        "Epstein"
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
