cognitive-research-hub/trainer/src/model/model-spec.md
Purpose

The model layer provides modular, deterministic architectures used by the Trainer:

Learner (classifier / MLP, optional conv front-end)

Bridge (Chromatic ↔ Sonic encoder/decoder heads over fixed tensors)

Interfaces (traits + adapters) so training code is agnostic to the concrete net

Deterministic initialization and shape contracts bound to trainer/config

All models must produce identical outputs for identical inputs, seeds, and configs across OS.

Directory Layout
trainer/src/model/
├─ model-spec.md                 ← this spec
├─ mod.rs                        ← re-exports & factory
├─ traits.rs                     ← Model, Classifier, AutoEncoder traits
├─ init.rs                       ← deterministic weight init (xavier, kaiming)
├─ layers/
│  ├─ mlp.rs                     ← linear + activation + dropout (seeded)
│  ├─ conv2d.rs                  ← minimal conv (optional feature "conv")
│  └─ norm.rs                    ← layer/batch norm (seeded epsilon)
├─ learner/
│  ├─ classifier.rs              ← MLP classifier (3072→256→C)
│  └─ head.rs                    ← logits→loss/metrics adapter
├─ bridge/
│  ├─ chroma_encoder.rs          ← ChromaticTensor → UMS (HSL block)
│  ├─ spectral_encoder.rs        ← SpectralTensor → UMS (bands block)
│  └─ ums_decoder.rs             ← UMS → (chromatic, spectral) heads
├─ utils/
│  ├─ activations.rs             ← ReLU, GELU, Tanh (pure, exact)
│  ├─ losses.rs                  ← CE, MSE, ΔColor regularizer
│  └─ metrics.rs                 ← accuracy, ΔHSL, coherence proxy
└─ tests/
   ├─ test_init.rs
   ├─ test_forward_shapes.rs
   ├─ test_loss_determinism.rs
   └─ test_factory.rs

Configuration Mapping

All models are built from trainer/config/model/*.toml.

Required keys
[model]
type = "MLP"            # "MLP" | "BridgeAE"
input_dim = 3072
hidden_dim = 256
output_dim = 10
activation = "ReLU"     # "ReLU" | "GELU" | "Tanh"
init = "xavier"         # "xavier" | "kaiming"
dropout = 0.05

[optimizer]
type = "SGD"            # defined in trainer/src/training/optimizer.rs
learning_rate = 0.01
decay = 0.95
momentum = 0.9


Bridge models also require:

[bridge]
ums_dim = 512
hsl_offset = 256         # start channel for H/S/L
bands = 256              # spectral bands
gamma = 1.0              # A = S^gamma (appendix A)

Core Traits
// traits.rs
pub trait Model {
    /// Forward pass; deterministic for given input + state.
    fn forward(&mut self, x: &[f32]) -> Vec<f32>;

    /// Parameter access for training (flattened view).
    fn parameters(&mut self) -> Vec<&mut [f32]>;

    /// Apply in-place weight update (from optimizer).
    fn apply_update(&mut self, delta: &[f32]);

    /// Reproducible reset using global seed.
    fn reset_with_seed(&mut self, seed: u64);
}

pub trait Classifier: Model {
    /// Returns logits; loss computed via utils::losses.
    fn logits(&mut self, x: &[f32]) -> Vec<f32>;
}

pub trait AutoEncoder: Model {
    /// Encode → latent UMS; Decode → target modality heads.
    fn encode(&mut self, x: &[f32]) -> Vec<f32>;
    fn decode(&mut self, z: &[f32]) -> Vec<f32>;
}

Deterministic Initialization
// init.rs
pub enum InitKind { Xavier, Kaiming }

pub fn init_weights(buf: &mut [f32], fan_in: usize, fan_out: usize, kind: InitKind, seed: u64) {
    // Box–Muller PRNG from seed; fixed ordering; no SIMD reordering
    // Stddev computed per kind; write sequentially row-major.
}


Rules

Single stream PRNG seeded from seeds.global

Fixed write order (row-major by layer, then parameter index)

Dropout uses precomputed Bernoulli mask from the same seed per epoch

Layers
layers/mlp.rs
pub struct Linear {
    pub w: Vec<f32>, // row-major: out × in
    pub b: Vec<f32>, // length out
    pub in_dim: usize,
    pub out_dim: usize,
}

impl Linear {
    pub fn new(in_dim: usize, out_dim: usize, init: InitKind, seed: u64) -> Self;
    pub fn forward(&self, x: &[f32]) -> Vec<f32>;
}

pub struct Mlp {
    pub l1: Linear,
    pub act: Activation,  // enum
    pub l2: Linear,
    pub p_drop: f32,      // deterministic dropout prob
    pub drop_mask: Vec<bool>,
}

impl Mlp {
    pub fn new(in_dim: usize, hidden: usize, out_dim: usize, act: Activation, init: InitKind, seed: u64) -> Self;
    pub fn forward(&mut self, x: &[f32], epoch_seed: u64) -> Vec<f32>; // refresh mask from epoch_seed
}

(Optional) layers/conv2d.rs

Minimal NHWC single-stride conv for future image tokens (feature "conv").

Deterministic im2col; no parallelism in tests.

Learner
learner/classifier.rs
pub struct ColorClassifier {
    pub mlp: Mlp,
    pub num_classes: usize,
}

impl ColorClassifier {
    pub fn new(cfg: &ModelConfig, seed: u64) -> Self;
}

impl Model for ColorClassifier { /* forward → logits; params; apply_update; reset */ }
impl Classifier for ColorClassifier {
    fn logits(&mut self, x: &[f32]) -> Vec<f32> {
        self.mlp.forward(x, /*epoch_seed*/ 0)
    }
}

learner/head.rs

Turns logits → loss/metrics using utils::losses::{cross_entropy, accuracy}

Optional regularizers (e.g., L2) are deterministic (fixed order sum)

Bridge (Chromatic ↔ Sonic)
bridge/chroma_encoder.rs

Packs HSL into UMS[H,S,L] using Appendix A seam-safe hue normalization.

pub struct ChromaEncoder {
    pub ums_dim: usize,
    pub hsl_offset: usize,
}

impl ChromaEncoder {
    pub fn encode_hsl(&self, h: f32, s: f32, l: f32) -> Vec<f32>; // writes to fixed slots
}

bridge/spectral_encoder.rs

Writes normalized spectral bands into UMS[0..bands] (deterministic LUT order).

pub struct SpectralEncoder {
    pub bands: usize,
}

impl SpectralEncoder {
    pub fn encode_bands(&self, spectrum: &[f32]) -> Vec<f32>;
}

bridge/ums_decoder.rs

Reads UMS → predicted heads (HSL and Spectral bands) via small MLP(s).

Used in Phase 7 for supervised reconstruction tests.

pub struct UmsDecoder {
    pub h_head: Mlp, // predict H (wrapped angle encoded as sin/cos or direct)
    pub s_head: Mlp,
    pub l_head: Mlp,
    pub bands_head: Mlp,
}

impl AutoEncoder for UmsDecoder {
    fn encode(&mut self, x: &[f32]) -> Vec<f32> { /* identity or small bottleneck */ }
    fn decode(&mut self, z: &[f32]) -> Vec<f32> { /* concat heads deterministically */ }
}

Utilities
utils/activations.rs

relu(x), gelu_exact(x) (no approximate fast-gelu), tanh(x)

Pure functions, no global state.

utils/losses.rs

cross_entropy(logits, target_idx)

mse(a, b)

delta_color_regularizer(hsl_pred, hsl_target, w_h, w_s, w_l) (seam-safe ΔH)

utils/metrics.rs

accuracy(logits, target_idx)

delta_hsl(hsl_pred, hsl_target) (wrap-aware)

coherence_proxy(spec: &[f32]) (sum/centroid-based, deterministic)

Factory
// mod.rs
pub enum ModelKind { MLP, BridgeAE }

pub fn build_from_config(cfg: &ModelConfig, seed: u64) -> Box<dyn Model> {
    match cfg.model_type {
        ModelKind::MLP => Box::new(ColorClassifier::new(cfg, seed)),
        ModelKind::BridgeAE => Box::new(UmsDecoder::new(cfg, seed)),
    }
}

Determinism & Testing

Single-threaded in tests; parallel features gated.

All sums over >8 terms use fixed-point accumulator from core tensor spec, or a Kahan-like compensator with fixed order.

Dropout masks derived from (epoch_seed ⊕ layer_index ⊕ param_index).

Unit/Integration Tests (tests/ here)

test_init.rs — same seed → identical weights (hash equal)

test_forward_shapes.rs — shapes match config

test_loss_determinism.rs — same batch twice → identical loss

test_factory.rs — config → model selection is correct

Interfaces With Other Modules
Uses	Why
trainer/src/training/*	Optimizers call parameters() and apply_update()
core/src/tensor/*	ΔColor regularizer and seam-safe hue ops (Phase 7)
core/src/bridge/*	Consistency checks when training BridgeAE
trainer/src/validator/*	Metrics + report generation
Status
Field	Value
Spec Version	1.0
Phase Alignment	5 (Learner) · 7 (Bridge)
Determinism Level	Bit-stable for identical seeds
Readiness	✅ Implementation ready
Next Spec	trainer/src/tokenizer/ or trainer/src/training/ (your call)
