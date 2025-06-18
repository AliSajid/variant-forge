# Experimental Exchangeability Matrix

This matrix is based on experimental measurements of amino acid exchangeability,
where the ease of substitution is quantified based on observed changes in protein
function and stability. The data were derived from empirical mutation studies
and reflect functional tolerance rather than just physical similarity.

**Citation**:
Yampolsky, L. Y., & Stoltzfus, A. (2005). The exchangeability of amino acids in proteins.\
*Genetics*, 170(4), 1459–1472. <https://doi.org/10.1534/genetics.104.039115>

**Value type**: `f32`

**Symmetry**: Yes (symmetric)

**Missing values**: Represented as `f32::NAN` or skipped with `"."` in the CSV

**Amino Acid Row/Column Order**:
A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
