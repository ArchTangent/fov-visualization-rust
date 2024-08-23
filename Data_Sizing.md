# Data Size Analysis for FOV Visualization - Rust (2D)

## General

- `1` or `2` Quantized `FovLines` per unit of FOV radius: if `rFOV=32`, `Qval={32,64}`
  - `FovLines` are spaced _evenly_ along the far edge, furthest away from the observer.
- Some obstructions may block LOS, but _not_ FOV (units crouching behind low walls).
- _circular culling_ is used to "pre-bake" a circular FOV.
  - approximate reduction from rectangular FOV to circular FOV is ~1 in 6 tiles.
- FOV data is broken up into eight _octants_.

## Simple FOV

Assume:
- `1` obstruction per `FovTile`, consisting of a single _block_.
  - Each _block_ can either be entirely empty (no obstruction) or full (fully obstructs sight)

Sizing:

| rFOV | Qval | nTile | nObst | bytes |  per_O |  total  |   size  |
| ---: | ---: | ----: | ----: | ----: | -----: | ------: | ------: |
|   32 |   32 |   468 |   468 |     4 |   1872 |   14976 |  15.0KB |
|   32 |   64 |   468 |   468 |     8 |   3744 |   29552 |  29.6KB |
|   64 |   64 |  1788 |  1788 |     8 |  14304 |  114432 | 114.4KB |
|   64 |  128 |  1788 |  1788 |    16 |  28608 |  228864 | 228.9KB |
|  128 |  128 |  6988 |  6988 |    16 | 111808 |  894464 | 894.5KB |
|  128 |  256 |  6988 |  6988 |    32 | 223616 | 1788928 |   1.8MB |

Where:
- `rFOV`  = FOV radius.
- `Qval`  = number of quantized FOV lines, equal to number of `visible` and `blocking` bits for FOV calc.
- `nTile` = number of tiles per octant.
- `nObst` = number of obstructions per octant, equal to `tiles per octant * obstructions per tile`.
- `bytes` = number of bytes per obstruction, equal to `Qval / 8`, rounded up to nearest integer.
- `per_O` = number of bytes per octant, equal to `bytes * nObst`.
- `total` = number of bytes in total for `FovData`, equal to `per_O * 8`.

## Exploded FOV

Assume:
- A `MapTile` "explodes" into component `FovNode` instances. These nodes include:
  - `body`:   the main tile body
  - `wall_n`: a North-facing wall
  - `wall_w`: a West-facing wall
- `1` obstruction per `FovNode`, consisting of a single _block_.
  - Each _block_ can either be entirely empty (no obstruction) or full (fully obstructs sight)

To preserve space:
- assume half-height objects (blocks or wall) do _not_ block FOV, but may block LOS. This allows for:
  - Tile `body` `FovNodes` to require only one set of Q-values.
  - Tile `wall_n` `FovNodes` to require only one set of Q-values.
  - Tile `wall_w` `FovNodes` to require only one set of Q-values.
- use _circular culling_ as described above
