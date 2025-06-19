// SPDX-FileCopyrightText: 2025 Ali Sajid Imami
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Auto-generated from `data/processed/exchangability.csv`
//! Matrix: `Exchangability`
//! Symmetric: false

use crate::models::{
    AminoAcid,
    DistanceMetric,
};

/// # Experimental Exchangability Matrix
///
///  This matrix is based on experimental measurements of amino acid exchangability,
///  where the ease of substitution is quantified based on observed changes in protein
///  function and stability. The data were derived from empirical mutation studies
///  and reflect functional tolerance rather than just physical similarity.
///
///  **Citation**:
///  Yampolsky, L. Y., & Stoltzfus, A. (2005). The exchangability of amino acids in proteins.\
///  *Genetics*, 170(4), 1459–1472. <https://doi.org/10.1534/genetics.104.039115>
///
///  **Value type**: `f32`
///
///  **Symmetry**: No (asymmetric)
///
///  **Missing values**: Represented as `f32::NAN` or skipped with `"."` in the CSV
///
///  **Amino Acid Row/Column Order**:
///  A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
#[derive(Debug, Copy, Clone, Default)]
pub struct Exchangability;

impl Exchangability {
    const MATRIX: [f64; 400] = [
        0f64, 295f64, 430f64, 193f64, 393f64, 320f64, 275f64, 387f64, 301f64, 245f64, 313f64,
        225f64, 549f64, 305f64, 243f64, 384f64, 312f64, 165f64, 286f64, 319f64, 459f64, 0f64,
        67f64, 124f64, 225f64, 288f64, 250f64, 251f64, 263f64, 139f64, 242f64, 306f64, 68f64,
        213f64, 145f64, 270f64, 199f64, 63f64, 272f64, 189f64, 400f64, 252f64, 0f64, 208f64,
        234f64, 298f64, 257f64, 391f64, 248f64, 184f64, 233f64, 183f64, 236f64, 210f64, 275f64,
        355f64, 329f64, 120f64, 251f64, 233f64, 293f64, 252f64, 201f64, 0f64, 285f64, 263f64,
        344f64, 264f64, 298f64, 299f64, 236f64, 208f64, 245f64, 233f64, 220f64, 275f64, 245f64,
        103f64, 227f64, 175f64, 334f64, 306f64, 109f64, 109f64, 0f64, 383f64, 270f64, 288f64,
        258f64, 109f64, 347f64, 252f64, 169f64, 349f64, 201f64, 258f64, 121f64, 139f64, 349f64,
        89f64, 499f64, 366f64, 338f64, 68f64, 383f64, 0f64, 439f64, 406f64, 396f64, 467f64, 391f64,
        354f64, 504f64, 383f64, 212f64, 443f64, 361f64, 159f64, 361f64, 603f64, 520f64, 279f64,
        258f64, 533f64, 332f64, 341f64, 0f64, 407f64, 380f64, 450f64, 321f64, 323f64, 219f64,
        342f64, 216f64, 355f64, 292f64, 145f64, 348f64, 351f64, 369f64, 178f64, 210f64, 188f64,
        267f64, 272f64, 206f64, 0f64, 235f64, 110f64, 193f64, 219f64, 197f64, 168f64, 140f64,
        304f64, 187f64, 173f64, 188f64, 208f64, 462f64, 275f64, 225f64, 141f64, 331f64, 301f64,
        319f64, 370f64, 0f64, 205f64, 364f64, 332f64, 315f64, 328f64, 220f64, 365f64, 205f64,
        72f64, 260f64, 255f64, 326f64, 124f64, 172f64, 27f64, 362f64, 191f64, 197f64, 160f64,
        221f64, 0f64, 417f64, 121f64, 279f64, 331f64, 145f64, 196f64, 193f64, 73f64, 323f64,
        494f64, 343f64, 185f64, 162f64, 112f64, 366f64, 250f64, 199f64, 201f64, 288f64, 301f64,
        0f64, 171f64, 367f64, 336f64, 146f64, 212f64, 165f64, 152f64, 295f64, 275f64, 600f64,
        440f64, 457f64, 465f64, 331f64, 441f64, 272f64, 492f64, 362f64, 491f64, 301f64, 0f64,
        414f64, 360f64, 252f64, 376f64, 476f64, 218f64, 343f64, 487f64, 357f64, 112f64, 544f64,
        392f64, 347f64, 394f64, 287f64, 218f64, 278f64, 612f64, 513f64, 135f64, 0f64, 330f64,
        85f64, 353f64, 261f64, 633f64, 308f64, 354f64, 236f64, 122f64, 136f64, 90f64, 176f64,
        216f64, 62f64, 94f64, 237f64, 181f64, 296f64, 85f64, 255f64, 0f64, 112f64, 152f64, 257f64,
        232f64, 332f64, 291f64, 454f64, 254f64, 352f64, 254f64, 345f64, 384f64, 346f64, 404f64,
        369f64, 204f64, 258f64, 231f64, 257f64, 339f64, 0f64, 392f64, 286f64, 305f64, 298f64,
        421f64, 490f64, 363f64, 390f64, 314f64, 373f64, 352f64, 343f64, 418f64, 353f64, 270f64,
        295f64, 275f64, 321f64, 334f64, 249f64, 0f64, 481f64, 160f64, 294f64, 358f64, 402f64,
        299f64, 240f64, 190f64, 325f64, 308f64, 212f64, 332f64, 246f64, 198f64, 271f64, 256f64,
        152f64, 273f64, 164f64, 408f64, 0f64, 66f64, 260f64, 362f64, 63f64, 103f64, 65535f64,
        65535f64, 137f64, 61f64, 65f64, 162f64, 239f64, 65535f64, 177f64, 54f64, 110f64, 364f64,
        66f64, 92f64, 17f64, 65535f64, 281f64, 110f64, 402f64, 340f64, 129f64, 87f64, 142f64,
        369f64, 176f64, 357f64, 197f64, 65535f64, 362f64, 171f64, 392f64, 360f64, 194f64, 173f64,
        65535f64, 303f64, 0f64, 65535f64, 389f64, 190f64, 108f64, 228f64, 382f64, 280f64, 192f64,
        269f64, 253f64, 537f64, 333f64, 197f64, 562f64, 207f64, 201f64, 326f64, 398f64, 286f64,
        209f64, 0f64,
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
    /// use variant_forge_lib::datasets::Exchangability;
    ///
    /// let matrix = Exchangability::matrix();
    /// let distance = matrix[0 * 20 + 1]; // A → R
    /// assert!(distance >= 0.0);
    /// ```
    #[must_use]
    pub const fn matrix() -> &'static [f64; 400] {
        &Self::MATRIX
    }
}

impl DistanceMetric for Exchangability {
    fn name(&self) -> &'static str {
        "Exchangability"
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
        let metric = Exchangability;
        let d = metric.lookup(AminoAcid::A, AminoAcid::V);
        assert!(d.is_some());
        assert!(d.unwrap() > 0.0);
    }
}
