# Miyata Distance Matrix

The Miyata matrix uses a continuous float-based approach to encode physicochemical differences
between amino acids, accounting for both volume and polarity differences.
This matrix is symmetric and reflects molecular dissimilarity.

**Citation**:
Miyata, T., Miyazawa, S., & Yasunaga, T. (1979). Two types of amino acid substitutions in protein evolution. *Journal of Molecular Evolution*, 12(3), 219–236. <https://doi.org/10.1007/BF01732340>

**Value type**: `f32`

**Symmetry**: Yes (symmetric)

**Missing values**: Represented as `f32::NAN` or skipped with `"."` in the source

**Amino Acid Row/Column Order**:
A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
