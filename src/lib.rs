use ndarray::{Array1, Array2};
use plotters::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::{Serialize, Deserialize};

pub use plotters::style::RGBColor;

#[derive(Clone, Copy, Debug)]
pub struct ColorMap {
    pub low: RGBColor,
    pub mid: RGBColor,
    pub high: RGBColor,
    pub background: RGBColor,
}

impl ColorMap {
    pub const SOLAR: Self = Self { low: RGBColor(139, 69, 19), mid: RGBColor(255, 165, 0), high: RGBColor(255, 69, 0), background: RGBColor(30, 30, 30) };
    pub const DARK: Self = Self { low: RGBColor(25, 25, 112), mid: RGBColor(138, 43, 226), high: RGBColor(255, 20, 147), background: RGBColor(10, 10, 25) };
    pub const PLASMA: Self = Self { low: RGBColor(13, 8, 135), mid: RGBColor(240, 249, 33), high: RGBColor(255, 0, 0), background: RGBColor(5, 5, 20) };
    pub const MINIMAL: Self = Self { low: RGBColor(70, 70, 70), mid: RGBColor(180, 180, 180), high: RGBColor(255, 255, 255), background: RGBColor(20, 20, 20) };
    pub const NEON: Self = Self { low: RGBColor(0, 255, 255), mid: RGBColor(255, 0, 255), high: RGBColor(255, 255, 0), background: RGBColor(15, 15, 30) };

    pub const PROGRESS_DARK: &'static str = "{spinner:.cyan} [{elapsed_precise}] [{bar:45.cyan/magenta}] {pos}/{len} ({eta}) {msg}";
    pub const PROGRESS_SOLAR: &'static str = "{spinner:.orange} [{elapsed_precise}] [{bar:45.orange/red}] {pos}/{len} ({eta}) {msg}";
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Peak {
    pub period: f64,
    pub power: f64,
    pub fap_standard: f64,
    pub fap_bonferroni: f64,
    pub ci_low: f64,
    pub ci_high: f64,
}

pub struct MorletCWT { pub w0: f64 }

impl Default for MorletCWT { fn default() -> Self { Self { w0: 6.0 } } }

impl MorletCWT {
    pub fn new(w0: f64) -> Self { Self { w0 } }

    pub fn morlet2(&self, t: f64) -> f64 {
        let gauss = (-t*t/2.0).exp();
        let wave = (self.w0 * t).cos();
        gauss * wave
    }

    pub fn compute_cwt(&self, t: &Array1<f64>, signal: &Array1<f64>, widths: &[f64]) -> Array2<f64> {
        let n = t.len();
        let mut cwtmatr = Array2::<f64>::zeros((widths.len(), n));
        for (j, &width) in widths.iter().enumerate() {
            for i in 0..n {
                let tau = (t[i] - t.mean().unwrap_or(0.0)) / width;
                cwtmatr[[j, i]] = self.morlet2(tau) * signal[i];
            }
        }
        cwtmatr
    }

    pub fn example_signal(t: &Array1<f64>) -> Array1<f64> {
        let mut rng = rand::thread_rng();
        let mut sig = Array1::<f64>::zeros(t.len());
        for i in 0..t.len() {
            sig[i] = (2.0 * std::f64::consts::PI * t[i] / 11.07).sin()
                + 0.6 * (2.0 * std::f64::consts::PI * t[i] / 193.0).sin()
                + 0.4 * (2.0 * std::f64::consts::PI * t[i] / 90.0).sin()
                + rng.gen_range(-0.15..0.15);
        }
        sig
    }

    // Additional methods (power_spectrum, lomb_scargle_with_fap, bootstrap, etc.) can be added from previous conversation history.
}