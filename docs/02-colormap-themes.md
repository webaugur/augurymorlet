# ColorMap & Themes

The `ColorMap` struct controls colors for all visualizations.

## Available Presets

| Name     | Style                    | Best For             |
|----------|--------------------------|----------------------|
| `DARK`   | Deep space blue-purple   | **Clocks & UIs**     |
| `SOLAR`  | Warm orange/red          | Energetic themes     |
| `PLASMA` | Scientific plasma        | Data analysis        |
| `MINIMAL`| Clean grayscale          | Minimalist           |
| `NEON`   | Bright cyan/magenta      | Modern / cyberpunk   |

## Usage

```rust
let cmap = ColorMap::DARK;
cwt.save_svg_visuals_with_cmap(&t, &signal, "output", &cmap)?;
```

## Custom Theme

```rust
let custom = ColorMap {
    low: RGBColor(20, 20, 80),
    mid: RGBColor(100, 200, 255),
    high: RGBColor(255, 100, 50),
    background: RGBColor(15, 15, 25),
};
```