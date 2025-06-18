//! Auto-generated from data/processed/miyata.csv
//! Matrix: Miyata
//! Symmetric: true
use crate::{
    AminoAcid,
    DistanceMetric,
};

/// # Miyata Distance Matrix
///
/// The Miyata matrix uses a continuous float-based approach to encode physicochemical differences
/// between amino acids, accounting for both volume and polarity differences.
/// This matrix is symmetric and reflects molecular dissimilarity.
///
/// **Citation**:
/// Miyata, T., Miyazawa, S., & Yasunaga, T. (1979). Two types of amino acid substitutions in protein evolution. *Journal of Molecular Evolution*, 12(3), 219–236. <https://doi.org/10.1007/BF01732340>
///
/// **Value type**: `f32`
///
/// **Symmetry**: Yes (symmetric)
///
/// **Missing values**: Represented as `f32::NAN` or skipped with `"."` in the source
///
/// **Amino Acid Row/Column Order**:
/// A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
#[derive(Debug, Copy, Clone)]
pub struct Miyata;

impl Miyata {
    const MATRIX: [f32; 400] = [
        0f32, 2.92f32, 1.78f32, 2.37f32, 1.39f32, 1.92f32, 2.46f32, 0.91f32, 2.17f32, 2.69f32,
        2.76f32, 2.96f32, 2.42f32, 3.23f32, 0.06f32, 0.51f32, 0.9f32, 4.23f32, 3.18f32, 1.85f32,
        2.92f32, 0f32, 2.04f32, 2.34f32, 3.06f32, 1.13f32, 1.45f32, 3.58f32, 0.82f32, 2.49f32,
        2.62f32, 0.4f32, 2.29f32, 2.47f32, 2.9f32, 2.74f32, 2.03f32, 2.72f32, 2.02f32, 2.43f32,
        1.78f32, 2.04f32, 0f32, 0.65f32, 2.83f32, 0.99f32, 0.85f32, 1.96f32, 1.29f32, 3.37f32,
        3.49f32, 1.84f32, 3.08f32, 3.7f32, 1.8f32, 1.31f32, 1.4f32, 4.39f32, 3.42f32, 2.76f32,
        2.37f32, 2.34f32, 0.65f32, 0f32, 3.48f32, 1.47f32, 0.9f32, 2.37f32, 1.72f32, 3.98f32,
        4.1f32, 2.05f32, 3.69f32, 4.27f32, 2.4f32, 1.87f32, 2.05f32, 4.88f32, 3.95f32, 3.4f32,
        1.39f32, 3.06f32, 2.83f32, 3.48f32, 0f32, 2.48f32, 3.26f32, 2.22f32, 2.56f32, 1.63f32,
        1.65f32, 3.27f32, 1.46f32, 2.24f32, 1.33f32, 2.84f32, 1.45f32, 3.34f32, 2.38f32, 0.86f32,
        1.92f32, 1.13f32, 0.99f32, 1.47f32, 2.48f32, 0f32, 0.84f32, 2.48f32, 0.32f32, 2.57f32,
        2.7f32, 1.06f32, 2.3f32, 2.81f32, 1.92f32, 1.65f32, 1.12f32, 3.42f32, 2.48f32, 2.13f32,
        2.46f32, 1.45f32, 0.85f32, 0.9f32, 3.26f32, 0.84f32, 0f32, 2.78f32, 0.96f32, 3.39f32,
        3.53f32, 1.14f32, 3.13f32, 3.59f32, 2.48f32, 2.06f32, 1.83f32, 4.08f32, 3.22f32, 2.97f32,
        0.91f32, 3.58f32, 1.96f32, 2.37f32, 2.22f32, 2.48f32, 2.78f32, 0f32, 2.78f32, 3.6f32,
        3.67f32, 3.54f32, 3.34f32, 4.14f32, 0.97f32, 0.85f32, 1.7f32, 5.13f32, 4.08f32, 2.76f32,
        2.17f32, 0.82f32, 1.29f32, 1.72f32, 2.56f32, 0.32f32, 0.96f32, 2.78f32, 0f32, 2.45f32,
        2.59f32, 0.79f32, 2.19f32, 2.63f32, 2.15f32, 1.94f32, 1.32f32, 3.16f32, 2.27f32, 2.11f32,
        2.69f32, 2.49f32, 3.37f32, 3.98f32, 1.63f32, 2.57f32, 3.39f32, 3.6f32, 2.45f32, 0f32,
        0.14f32, 2.84f32, 0.29f32, 0.61f32, 2.62f32, 2.95f32, 2.14f32, 1.72f32, 0.86f32, 0.85f32,
        2.76f32, 2.62f32, 3.49f32, 4.1f32, 1.65f32, 2.7f32, 3.53f32, 3.67f32, 2.59f32, 0.14f32,
        0f32, 2.98f32, 0.41f32, 0.63f32, 2.7f32, 3.04f32, 2.25f32, 1.73f32, 0.94f32, 0.91f32,
        2.96f32, 0.4f32, 1.84f32, 2.05f32, 3.27f32, 1.06f32, 1.14f32, 3.54f32, 0.79f32, 2.84f32,
        2.98f32, 0f32, 2.63f32, 2.85f32, 2.94f32, 2.71f32, 2.1f32, 3.11f32, 2.42f32, 2.7f32,
        2.42f32, 2.29f32, 3.08f32, 3.69f32, 1.46f32, 2.3f32, 3.13f32, 3.34f32, 2.19f32, 0.29f32,
        0.41f32, 2.63f32, 0f32, 0.82f32, 2.36f32, 2.67f32, 1.86f32, 1.89f32, 0.93f32, 0.62f32,
        3.23f32, 2.47f32, 3.7f32, 4.27f32, 2.24f32, 2.81f32, 3.59f32, 4.14f32, 2.63f32, 0.61f32,
        0.63f32, 2.85f32, 0.82f32, 0f32, 3.17f32, 3.45f32, 2.6f32, 1.11f32, 0.48f32, 1.43f32,
        0.06f32, 2.9f32, 1.8f32, 2.4f32, 1.33f32, 1.92f32, 2.48f32, 0.97f32, 2.15f32, 2.62f32,
        2.7f32, 2.94f32, 2.36f32, 3.17f32, 0f32, 0.56f32, 0.87f32, 4.17f32, 3.12f32, 1.79f32,
        0.51f32, 2.74f32, 1.31f32, 1.87f32, 2.84f32, 1.65f32, 2.06f32, 0.85f32, 1.94f32, 2.95f32,
        3.04f32, 2.71f32, 2.67f32, 3.45f32, 0.56f32, 0f32, 0.89f32, 4.38f32, 3.33f32, 2.15f32,
        0.9f32, 2.03f32, 1.4f32, 2.05f32, 1.45f32, 1.12f32, 1.83f32, 1.7f32, 1.32f32, 2.14f32,
        2.25f32, 2.1f32, 1.86f32, 2.6f32, 0.87f32, 0.89f32, 0f32, 3.5f32, 2.45f32, 1.42f32,
        4.23f32, 2.72f32, 4.39f32, 4.88f32, 3.34f32, 3.42f32, 4.08f32, 5.13f32, 3.16f32, 1.72f32,
        1.73f32, 3.11f32, 1.89f32, 1.11f32, 4.17f32, 4.38f32, 3.5f32, 0f32, 1.06f32, 2.51f32,
        3.18f32, 2.02f32, 3.42f32, 3.95f32, 2.38f32, 2.48f32, 3.22f32, 4.08f32, 2.27f32, 0.86f32,
        0.94f32, 2.42f32, 0.93f32, 0.48f32, 3.12f32, 3.33f32, 2.45f32, 1.06f32, 0f32, 1.52f32,
        1.85f32, 2.43f32, 2.76f32, 3.4f32, 0.86f32, 2.13f32, 2.97f32, 2.76f32, 2.11f32, 0.85f32,
        0.91f32, 2.7f32, 0.62f32, 1.43f32, 1.79f32, 2.15f32, 1.42f32, 2.51f32, 1.52f32, 0f32,
    ];

    /// Returns a reference to the flattened 20×20 distance matrix for the `Miyata` dataset.
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
    /// use variant_forge_lib::datasets::miyata::Miyata;
    ///
    /// let matrix = Miyata::matrix();
    /// let distance = matrix[0 * 20 + 1]; // A → R
    /// ```
    #[must_use]
    pub const fn matrix() -> &'static [f32; 400] {
        &Self::MATRIX
    }
}

impl DistanceMetric for Miyata {
    type Value = f32;

    fn name(&self) -> &'static str {
        "Miyata"
    }

    fn is_symmetric(&self) -> bool {
        true
    }

    fn lookup(&self, a: AminoAcid, b: AminoAcid) -> Option<Self::Value> {
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
