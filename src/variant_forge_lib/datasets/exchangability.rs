//! Auto-generated from data/processed/exchangability.csv
//! Matrix: Exchangability
//! Symmetric: false
use crate::{
    AminoAcid,
    DistanceMetric,
};

/// # Experimental Exchangeability Matrix
///
/// This matrix is based on experimental measurements of amino acid exchangeability,
/// where the ease of substitution is quantified based on observed changes in protein
/// function and stability. The data were derived from empirical mutation studies
/// and reflect functional tolerance rather than just physical similarity.
///
/// **Citation**:
/// Yampolsky, L. Y., & Stoltzfus, A. (2005). The exchangeability of amino acids in proteins.\
/// *Genetics*, 170(4), 1459–1472. <https://doi.org/10.1534/genetics.104.039115>
///
/// **Value type**: `f32`
///
/// **Symmetry**: Yes (symmetric)
///
/// **Missing values**: Represented as `f32::NAN` or skipped with `"."` in the CSV
///
/// **Amino Acid Row/Column Order**:
/// A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
#[derive(Debug, Copy, Clone)]
pub struct Exchangability;

impl Exchangability {
    const MATRIX: [u16; 400] = [
        0, 295, 430, 193, 393, 320, 275, 387, 301, 245, 313, 225, 549, 305, 243, 384, 312, 165,
        286, 319, 459, 0, 67, 124, 225, 288, 250, 251, 263, 139, 242, 306, 68, 213, 145, 270, 199,
        63, 272, 189, 400, 252, 0, 208, 234, 298, 257, 391, 248, 184, 233, 183, 236, 210, 275, 355,
        329, 120, 251, 233, 293, 252, 201, 0, 285, 263, 344, 264, 298, 299, 236, 208, 245, 233,
        220, 275, 245, 103, 227, 175, 334, 306, 109, 109, 0, 383, 270, 288, 258, 109, 347, 252,
        169, 349, 201, 258, 121, 139, 349, 89, 499, 366, 338, 68, 383, 0, 439, 406, 396, 467, 391,
        354, 504, 383, 212, 443, 361, 159, 361, 603, 520, 279, 258, 533, 332, 341, 0, 407, 380,
        450, 321, 323, 219, 342, 216, 355, 292, 145, 348, 351, 369, 178, 210, 188, 267, 272, 206,
        0, 235, 110, 193, 219, 197, 168, 140, 304, 187, 173, 188, 208, 462, 275, 225, 141, 331,
        301, 319, 370, 0, 205, 364, 332, 315, 328, 220, 365, 205, 72, 260, 255, 326, 124, 172, 27,
        362, 191, 197, 160, 221, 0, 417, 121, 279, 331, 145, 196, 193, 73, 323, 494, 343, 185, 162,
        112, 366, 250, 199, 201, 288, 301, 0, 171, 367, 336, 146, 212, 165, 152, 295, 275, 600,
        440, 457, 465, 331, 441, 272, 492, 362, 491, 301, 0, 414, 360, 252, 376, 476, 218, 343,
        487, 357, 112, 544, 392, 347, 394, 287, 218, 278, 612, 513, 135, 0, 330, 85, 353, 261, 633,
        308, 354, 236, 122, 136, 90, 176, 216, 62, 94, 237, 181, 296, 85, 255, 0, 112, 152, 257,
        232, 332, 291, 454, 254, 352, 254, 345, 384, 346, 404, 369, 204, 258, 231, 257, 339, 0,
        392, 286, 305, 298, 421, 490, 363, 390, 314, 373, 352, 343, 418, 353, 270, 295, 275, 321,
        334, 249, 0, 481, 160, 294, 358, 402, 299, 240, 190, 325, 308, 212, 332, 246, 198, 271,
        256, 152, 273, 164, 408, 0, 66, 260, 362, 63, 103, 65535, 65535, 137, 61, 65, 162, 239,
        65535, 177, 54, 110, 364, 66, 92, 17, 65535, 281, 110, 402, 340, 129, 87, 142, 369, 176,
        357, 197, 65535, 362, 171, 392, 360, 194, 173, 65535, 303, 0, 65535, 389, 190, 108, 228,
        382, 280, 192, 269, 253, 537, 333, 197, 562, 207, 201, 326, 398, 286, 209, 0,
    ];

    /// Returns a reference to the flattened 20×20 distance matrix for the `Exchangability` dataset.
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
    /// use variant_forge_lib::datasets::exchangability::Exchangability;
    ///
    /// let matrix = Exchangability::matrix();
    /// let distance = matrix[0 * 20 + 1]; // A → R
    /// ```
    #[must_use]
    pub const fn matrix() -> &'static [u16; 400] {
        &Self::MATRIX
    }
}

impl DistanceMetric for Exchangability {
    type Value = u16;

    fn name(&self) -> &'static str {
        "Exchangability"
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
