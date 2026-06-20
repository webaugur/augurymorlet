# Installation & Getting Started

## Adding the Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
cwt-morlet = "0.2.0"
```

## Basic Example

```rust
use cwt_morlet::{MorletCWT, ColorMap};
use ndarray::Array1;

fn main() {
    let t = Array1::linspace(0.0, 1000.0, 4000);
    let signal = MorletCWT::example_signal(&t);
    let cwt = MorletCWT::default();

    let cmap = ColorMap::DARK;

    cwt.save_svg_visuals_with_cmap(&t, &signal, "output/dark", &cmap).unwrap();

    let peaks = cwt.export_significant_peaks_bootstrap_parallel(
        &t, &signal, 0.01, 2000, "peaks.json", Some(ColorMap::PROGRESS_DARK)
    ).unwrap();

    println!("Found {} significant peaks", peaks.len());
}
```

## Requirements

- Rust 1.70 or newer

## Next Steps

- [ColorMap & Themes](02-colormap-themes.md)
- [Morlet CWT](03-morlet-cwt.md)