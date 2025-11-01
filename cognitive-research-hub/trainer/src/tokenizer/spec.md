Specification: Tokenizer Subsystem

Module Path: cognitive-research-hub/trainer/src/tokenizer/
Parent Spec: trainer/src/spec.md
Related Specs:

trainer/src/model/spec.md — downstream consumer of encoded token sequences

core/tensor/spec.md — provides base tensor interfaces for token data

core/bridge/spec.md — defines chromatic ↔ sonic / multimodal mappings

trainer/src/validator/spec.md — validates encoding–decoding reversibility

I. Mission

The Tokenizer Subsystem ensures that all forms of data — text, image, spectral, or chromatic tensors — are represented in a unified deterministic token space.
It provides bidirectional transformation between raw modalities and model-ready embeddings while preserving exact reversibility (Δ < 1e-6).

It forms the semantic intake layer of the trainer pipeline — converting structured input into tokens the model can process, and decoding model outputs back into interpretable forms.

II. Core Objectives
Goal	Description
Consistency	Identical input → identical token stream across environments
Multimodality	Handle text, numerical, chromatic, and tensor inputs
Reversibility	Full decode–encode roundtrip with no entropy loss
Compression	Map high-dimensional tensors into compact embeddings
Auditability	Maintain token logs and ID–string dictionaries
Safety	Support medical data anonymization during tokenization
III. Directory Layout
tokenizer/
├─ spec.md
├─ text/
│   ├─ text_tokenizer.rs      # Deterministic text encoder/decoder
│   ├─ vocab.json             # Frozen vocabulary
│   ├─ normalization.py       # Unicode + symbol normalization
│
├─ image/
│   ├─ image_tokenizer.rs     # Encodes images to patch tokens
│   ├─ color_tokenizer.rs     # Converts RGB → ChromaticTensor
│   ├─ hsl_utils.rs           # Deterministic hue/saturation mapping
│
├─ multimodal/
│   ├─ chromatic_encoder.rs   # Bridges image + spectral representations
│   ├─ sonic_encoder.rs       # For sound-based augmentations (Phase 7)
│   ├─ embedding_fusion.rs    # Merges cross-modal features into shared space
│
├─ tables/
│   ├─ token_map.json         # Maps tokens to embeddings
│   ├─ reverse_map.json       # Reverse lookups for decoding
│
└─ tests/
    ├─ tokenizer_roundtrip.rs
    ├─ chromatic_invariance.rs

IV. Functional Overview
1. Text Tokenization

Implements deterministic BPE or WordPiece with frozen vocabulary (vocab.json).

Normalizes Unicode and medical abbreviations.

Maps each token to a unique 32-bit integer ID.

Includes a reverse map for complete reconstruction.

2. Image / Chromatic Tokenization

Converts RGB image patches into normalized chromatic vectors:

RGB → HSL → ChromaticTensor (Phase 7a mapping)

Quantization uses 12-category hue partitions (deterministic chromatic alphabet).

Each token encodes:

[HueIndex, Saturation, Luminance, PatchPosX, PatchPosY]


Ensures full reversibility with floating-point clamping (Δ ≤ 1e-6).

3. Spectral / Sonic Tokenization

Converts sound features into tokenized harmonic descriptors using Gaussian kernels.

Maps to the same 12-category spectral alphabet for multimodal learning.

Deterministically aligned with the ChromaticTensor frequency bands.

4. Fusion Encoding

Uses canonical Unified Modality Space (UMS) (512D vector):

UMS = [Spectral(0–255), Chromatic(256–383), Temporal(384–511)]


All tokens are mapped into this continuous embedding via fixed affine projection.

5. Logging & Replay

Every token sequence is hashed and stored under tables/token_map.json.

reverse_map.json provides exact reconstruction for auditing.

Log entries include:

{
  "input_id": "image_20251031_00123",
  "token_hash": "ab56b4...",
  "length": 512,
  "modality": "chromatic"
}

V. Deterministic Constraints
Component	Guarantee	Mechanism
Text Normalization	Unicode-consistent	NFC normalization pipeline
RGB → HSL Conversion	Seamless hue boundary	Weighted seam correction (Appendix A)
Quantization	Fixed bins, no rounding drift	Deterministic clamp, FP32 precision
Token Mapping	Immutable index → embedding	Hash-based frozen vocabulary
Cross-Modal Fusion	Stable projection	Fixed affine weights, no learnable layer
Logging	Reproducible hashes	SHA256 with timestamp offset
VI. Interfaces
Function	Input	Output	Description
encode_text(str)	string	list[int]	Deterministic text → token IDs
decode_text(ids)	list[int]	string	Reverse of above
encode_image(img)	tensor	list[int]	Converts image patches → chromatic tokens
decode_image(tokens)	list[int]	tensor	Reconstructs visual tensor
encode_multimodal(inputs)	dict	list[float]	Fusion into Unified Modality Space
token_hash(tokens)	list[int]	str	Generates reproducible SHA256 signature
VII. Validation Tests
Test	Description	Expected Outcome
Roundtrip Consistency	encode → decode	identical reconstruction
Cross-Platform Determinism	Run on Linux/Windows	identical hashes
Chromatic Boundary Test	hue wrap-around	no color discontinuity
Sonic Alignment Test	frequency–hue mapping	≤ 1e-3 RMS deviation
Vocabulary Drift Test	vocabulary vs saved map	zero mismatched IDs
VIII. Integration Points
Module	Usage
core/tensor	Provides tensor normalization utilities
core/bridge	Supplies hue-frequency mapping constants
trainer/src/model	Consumes embeddings for training
trainer/src/validator	Validates reconstruction fidelity
data/processed	Stores tokenized datasets for reuse
IX. Output Files
File	Description
token_map.json	Frozen forward token dictionary
reverse_map.json	Reverse lookup for decoding
token_stats.json	Histogram of token distribution
tokenizer_log.csv	Sequence-level audit trail
X. Compliance Summary
Field	Specification
Spec Version	1.0
Reversibility	100% (Δ ≤ 1e-6)
Determinism	Cross-platform identical
Audit Agent	Tokenization Integrity Agent
Hash Scheme	SHA256 (config + token sequence)
Status	✅ Verified Tokenizer Spec Complete