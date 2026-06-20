# Lomb-Scargle Periodogram

## Why Use It?

Excellent for detecting periodic signals in unevenly sampled data with built-in statistical significance testing.

## Basic Usage

```rust
let periods: Vec<f64> = (1..500).map(|x| x as f64).collect();
let (power, fap_std, fap_bonf) = cwt.lomb_scargle_with_fap(&t, &signal, &periods);
```

## Output

- `power`: Lomb-Scargle power
- `fap_standard`: Standard false alarm probability
- `fap_bonferroni`: Bonferroni-corrected FAP (more conservative)

## Finding Significant Peaks

```rust
if fap_bonf[i] < 0.01 {
    println!("Significant period: {:.1} years", periods[i]);
}
```