# Morlet Continuous Wavelet Transform

## Overview

The Morlet wavelet provides excellent time-frequency resolution, ideal for non-stationary signals like solar activity.

## Basic Usage

```rust
let cwt = MorletCWT::default();
let widths: Vec<f64> = (5..500).step_by(4).map(|x| x as f64).collect();

let scalogram = cwt.compute_cwt(&t, &signal, &widths);
```

## Key Parameter

- `w0` (central frequency): Default = 6.0
  - Higher → better frequency resolution
  - Lower → better time resolution

## Common Applications

- Detecting Rieger-type periodicities
- Visualizing evolution of the Schwabe cycle
- Analyzing longer cycle modulation

## Visualization

```rust
cwt.save_svg_visuals_with_cmap(&t, &signal, "output", &ColorMap::DARK)?;
```