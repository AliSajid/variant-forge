# Epstein Distance Matrix

The Epstein distance matrix is an asymmetric matrix encoding substitution preferences
based on experimental mutation frequency data. It captures evolutionary pressures
in a directional manner, reflecting how likely one amino acid is to mutate into another.

**Citation**:
Epstein, C. J. (1967). Non-randomness of Amino Acid Changes in the Evolution of Homologous Proteins. *Nature*, 215(5096), 355–359. <https://doi.org/10.1038/215355a0>

**Value type**: `f32`

**Symmetry**: No (asymmetric)

**Missing values**: Represented as `f32::NAN` or skipped with `"."` in the source

**Amino Acid Row/Column Order**:
A, R, N, D, C, Q, E, G, H, I, L, K, M, F, P, S, T, W, Y, V
