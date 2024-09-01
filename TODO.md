# FOV Visualization - Rust (2D) To-Do List

## Command Line (CLI)

Usage:

```text
fov2d.exe <SUBCOMMAND> <TYPE> <RADIUS> [OPTIONS]

SUBCOMMAND:
  build
    TYPE:
      simple
      standard
    RADIUS:
      16, 32, 64, or 128
    OPTIONS:
      --qbasic -q: use Q-value equal to radius. By default, Q-value is 2x radius.

  view
    TYPE:
      simple
      standard
    RADIUS:
      16, 32, 64, or 128
    OPTIONS:
      --qbasic -q: use Q-value equal to radius. By default, Q-value is 2x radius.
```

## Performance Tweaks

- Try _multithreading_ (by octant?) and compare vs ST
  - `rayon`
  - `crossbeam` (`utils` feature for `crossbeam::thread`)
- Try _SIMD_ and compare vs scalar
  - `std::simd`

## FOV Generator

Generates FOV data that can be saved to file for easy reloading.

### Simple FOV

- R/Q Pairings:

  - R16: Q16, Q32
  - R32: Q32, Q64
  - R64: Q64, Q128
  - R128: Q128, Q256

- Benchmarks for various levels of randomly-placed obstructions

### Standard FOV

- R/Q Pairings:

  - R16: Q16, Q32
  - R32: Q32, Q64
  - R64: Q64, Q128
  - R128: Q128, Q256

- Benchmarks for various levels of randomly-placed obstructions

### Exploded FOV

_Note: this requires a different format for FOV Nodes and the TileMap!_

Quantized bits for the following FOV radii:

- 31: Q32, Q64
- 63: Q64, Q128
- 127: Q128, Q256

- `FovData`: holds FovNodes
- `FovNode`: holds quantized FOV bits for tile `visibility` and obstruction `blockers`
- `FovMap`: holds MapTiles in a representation of the FOV-related aspects of a real game map
- `MapTile`: holds obstructions that may be present in a tile
- Benchmarks for various levels of randomly-placed obstructions

## FOV Viewer

Calculates and displays FOV and LOS data.

### Calculations

- `fov_calc` to gather visible tiles for each move
- `los_calc` to gather visible tiles for each move

### Display

- Choose display technology: `macroquad`, `egui`, or perhaps both
- Show `source` observer unit
- Cursor tracking
- `FovLine` display
- Show all Obstructions
- Show FOV
- Expand and contract radius
- `draw_line` display func
- `draw_tile` display func
- `draw_map` display func
- `draw_unit` display func for observer and non-observer unit types
