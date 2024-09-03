# FOV Visualization - Rust (2D)

2D field of view (FOV) visualization for video games, using Rust.

## Key Types

Key types include:
- `FovData`: holds one or more FOV Sets.
- `FovSet`: holds eight FOV octants.
- `FovOctant`:  holds a list of FOV nodes.
- `FovNode`: holds quantized FOV bits for tile `visibility` and obstruction `blockers`.
- `TileMap`: holds in-game tiles.
- `Tile`: holds obstructions that may be present in a tile

## FOV Calculations

### Simple FOV

_Simple_ FOV uses calculates visible Tiles using only the Tile _body_.

Radius and Quantized Bit Pairings:
- R16:  Q16, Q32
- R32:  Q32, Q64
- R64:  Q64, Q128
- R128: Q128, Q256

### Standard FOV

_Standard_ FOV uses calculates visible Tiles using a Tile's _body_, _north wall_ , and _west wall_.

Radius and Quantized Bit Pairings:
- R16:  Q16, Q32
- R32:  Q32, Q64
- R64:  Q64, Q128
- R128: Q128, Q256
