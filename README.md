# augurymorlet

**Morlet Continuous Wavelet Transform + Lomb-Scargle Periodogram** with bootstrap confidence intervals, beautiful SVG visualization, and color themes.

Designed especially for solar cycle analysis (Schwabe, Gleissberg, Suess-de Vries, Rieger periodicities) and time-series visualization.

## Features

- Morlet CWT with customizable ω0
- Lomb-Scargle periodogram with standard + Bonferroni FAP
- Parallel bootstrap confidence intervals with progress bar
- SVG export for scalograms, PSD, and periodograms
- Multiple beautiful color themes (`DARK`, `SOLAR`, `PLASMA`, etc.)
- Time-evolving animation frame generation
- JSON export of significant peaks with CI

## Documentation

Extensive per-topic documentation is available in the [`docs/`](docs/) folder:

- [Installation & Getting Started](docs/01-installation.md)
- [ColorMap & Themes](docs/02-colormap-themes.md)
- [Morlet Continuous Wavelet Transform](docs/03-morlet-cwt.md)
- [Lomb-Scargle Periodogram](docs/04-lomb-scargle.md)
- [Conversation History Summary](docs/history/conversation-summary.md)

## Quick Start

```toml
[dependencies]
cwt-morlet = "0.2.0"
```

```rust
use cwt_morlet::{MorletCWT, ColorMap};
use ndarray::Array1;

fn main() {
    let t = Array1::linspace(0.0, 1000.0, 4000);
    let signal = MorletCWT::example_signal(&t);
    let cwt = MorletCWT::default();

    let cmap = ColorMap::DARK;

    cwt.save_svg_visuals_with_cmap(&t, &signal, "output", &cmap).unwrap();

    let peaks = cwt.export_significant_peaks_bootstrap_parallel(
        &t, &signal, 0.01, 2000, "peaks.json", Some(ColorMap::PROGRESS_DARK)
    ).unwrap();
}
```

## License

MIT or Apache-2.0

## Contributing

Pull requests welcome! Especially for more themes, better bootstrap performance, or real helioseismic data loaders.