//! Chromatic tensor representation and color utilities.
//!
//! Specification references:
//! - `cognitive-research-hub/spec.md`
//! - `cognitive-research-hub/core/spec.md`
//! - `cognitive-research-hub/core/src/tensor/spec.md`

use std::f32::consts::PI;

use super::{channel_offset, Shape2D, Stride2D};
use crate::Fx;

/// RGB chromatic tensor with optional per-cell coherence metadata.
#[derive(Clone, Debug)]
pub struct ChromaticTensor {
    pub shape: Shape2D,
    pub stride: Stride2D,
    pub rgb: Vec<Fx>,
    pub coh: Option<Vec<Fx>>,
}

impl ChromaticTensor {
    /// Creates a chromatic tensor ensuring the supplied buffers match the layout.
    pub fn new(shape: Shape2D, rgb: Vec<Fx>, coh: Option<Vec<Fx>>) -> Self {
        let stride = Stride2D::new(shape);
        let expected_len = shape.rgb_len();
        assert_eq!(rgb.len(), expected_len, "rgb buffer length mismatch");
        if let Some(ref coh_buf) = coh {
            assert_eq!(
                coh_buf.len(),
                shape.cell_count(),
                "coherence buffer mismatch"
            );
        }
        Self {
            shape,
            stride,
            rgb,
            coh,
        }
    }

    /// Returns immutable access to an RGB triplet at the specified coordinates.
    pub fn rgb_at(&self, row: usize, col: usize) -> [Fx; 3] {
        assert!(
            row < self.shape.h && col < self.shape.w,
            "index out of bounds"
        );
        let offset = channel_offset(self.stride, row, col, 0);
        [self.rgb[offset], self.rgb[offset + 1], self.rgb[offset + 2]]
    }

    /// Writes an RGB triplet at the specified coordinates.
    pub fn set_rgb(&mut self, row: usize, col: usize, values: [Fx; 3]) {
        assert!(
            row < self.shape.h && col < self.shape.w,
            "index out of bounds"
        );
        let offset = channel_offset(self.stride, row, col, 0);
        self.rgb[offset] = values[0];
        self.rgb[offset + 1] = values[1];
        self.rgb[offset + 2] = values[2];
    }
}

/// Wraps an angle to the [0, 2π) interval deterministically.
pub fn normalize_hue(h: Fx) -> Fx {
    let two_pi = 2.0 * PI;
    let mut value = h % two_pi;
    if value < 0.0 {
        value += two_pi;
    }
    value
}

/// Converts an RGB triplet in [0,1] to HSL (H in [0, 2π), S/L in [0,1]).
pub fn rgb_to_hsl(r: Fx, g: Fx, b: Fx) -> (Fx, Fx, Fx) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) * 0.5;

    if (max - min).abs() < f32::EPSILON {
        return (0.0, 0.0, l);
    }

    let d = max - min;
    let s = if l > 0.5 {
        d / (2.0 - max - min)
    } else {
        d / (max + min)
    };
    let mut h = if (max - r).abs() < f32::EPSILON {
        (g - b) / d + if g < b { 6.0 } else { 0.0 }
    } else if (max - g).abs() < f32::EPSILON {
        (b - r) / d + 2.0
    } else {
        (r - g) / d + 4.0
    };
    h /= 6.0;
    (normalize_hue(h * 2.0 * PI), s, l)
}

fn hue_to_rgb(p: Fx, q: Fx, mut t: Fx) -> Fx {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 0.5 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

/// Converts an HSL triple to RGB values in [0, 1].
pub fn hsl_to_rgb(h: Fx, s: Fx, l: Fx) -> (Fx, Fx, Fx) {
    if s <= f32::EPSILON {
        let v = l;
        return (v, v, v);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let h_norm = normalize_hue(h) / (2.0 * PI);
    let r = hue_to_rgb(p, q, h_norm + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h_norm);
    let b = hue_to_rgb(p, q, h_norm - 1.0 / 3.0);
    (r, g, b)
}

/// Computes seam-aware ΔHSL values for two HSL triples.
pub fn delta_hsl(a: (Fx, Fx, Fx), b: (Fx, Fx, Fx)) -> (Fx, Fx, Fx) {
    let (h1, s1, l1) = a;
    let (h2, s2, l2) = b;
    let dh = (h2 - h1).sin().atan2((h2 - h1).cos());
    (dh, s2 - s1, l2 - l1)
}
