# FOV Visualization - Rust (2D) To-Do List

Use two separate binaries: `FOV Generator` and `FOV Viewer`.

## FOV Generator

Generates FOV data that can be saved to file for easy reloading.

### General

- Geometry objects: 2D lines, rays, rects, intersections

### Simple FOV

Quantized bits for the following FOV radii:
- 31:  Q32, Q64
- 63:  Q64, Q128
- 127: Q128, Q256

- `FovData`: holds FovTiles
- `FovTile`: holds quantized FOV bits for tile `visibility` and obstruction `blockers`
- `FovMap`:  holds MapTiles in a representation of the FOV-related aspects of a real game map
- `MapTile`: holds obstructions that may be present in a tile
- Benchmarks for various levels of randomly-placed obstructions

### Exploded FOV

Quantized bits for the following FOV radii:
- 31:  Q32, Q64
- 63:  Q64, Q128
- 127: Q128, Q256

- `FovData`: holds FovTiles
- `FovTile`: holds quantized FOV bits for tile `visibility` and obstruction `blockers`
- `FovMap`:  holds MapTiles in a representation of the FOV-related aspects of a real game map
- `MapTile`: holds obstructions that may be present in a tile
- Benchmarks for various levels of randomly-placed obstructions

## FOV Viewer

Calculates and displays FOV and LOS data.

### Calculation

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
