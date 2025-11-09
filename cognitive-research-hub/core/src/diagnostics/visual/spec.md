# Module: core/src/diagnostics/visual/
# Spec Version: 1.1 (Aligned with canonical roadmap)
# Purpose

The Visual Diagnostics Module provides deterministic, reproducible visualizations of internal system states for human inspection and automated validation.
It transforms numerical metrics from diagnostics/metrics into images, plots, and interactive maps representing the stability and coherence of the Chromatic Core.

It is designed for visual continuity verification, training analysis, and phase integrity review during system development.

# Scope
Layer	Role
Rendering Engine	Converts metrics into raster or vector graphics for inspection.
Spiral Visualization	Displays chromatic trajectory in hue–saturation–coherence space.
Spectral Drift Analyzer	Shows frequency distribution changes across cycles.
Coherence Field Mapper	Provides heatmaps of spectral stability and energy conservation.
Output Interface	Generates static images (PNG/SVG) and streams frames for real-time monitoring.
Data Inputs

From Metrics Engine (metrics/)

* ChromaticDelta
* SpectralStats
* ContinuityMetrics

From Chronicle (meta/chronicle)

* CycleRecord series for long-term temporal visualizations.

From Tensor Modules (tensor/)

* For raw field-to-image rendering and tensor map overlays.

# Core Functions
Function	Signature	Description
plot_chromatic_spiral()	(history: &[ChromaticTensor]) -> ImageBuffer	Renders 2D spiral showing hue and coherence evolution across dream cycles.
render_energy_drift_plot()	(drift_data: &[SpectralStats]) -> SvgDocument	Plots total spectral energy over epochs to visualize conservation.
generate_coherence_heatmap()	(stats: &[SpectralStats]) -> ImageBuffer	Builds color-coded stability map of spectral coherence vs time.
compose_diagnostics_dashboard()	(metrics: &DiagnosticsSnapshot) -> DiagnosticsPanel	Creates unified panel with all key indicators for report export.
export_visual_report()	(path: &str, panel: &DiagnosticsPanel) -> Result<(), IOError>	Exports rendered graphics to disk in PNG or SVG format.
Mathematical & Visual Design Principles

Chromatic Spiral Plot

Coordinates:

$x = S \cdot \cos(H), y = S \cdot \sin(H)$

Hue (H) controls angular rotation, Saturation (S) sets radius.

Spiral’s radial displacement shows coherence over time:

$r_t = S_t \cdot C_t$

where $C_t$ = Coherence metric.

Spectral Drift Curve

Derived from average spectral centroid drift:

$\Delta f_t = \frac{f_{centroid,t} - f_{centroid,0}}{f_{centroid,0}}$

Color-coded trace indicates deviation from baseline.

Coherence Heatmap

2D grid with time vs frequency axes, filled by normalized coherence:

$\text{Color}(t, f) = \text{map}(C_t(f)) \rightarrow RGB$

Uses perceptually uniform color mapping (CET or Viridis LUT).

Deterministic Constraints
Concern	Solution
Floating-point raster variance	Fixed-point quantization for pixel values
Rendering order	Locked frame ordering by chronological index
Randomized color mapping	LUT-based mapping from static table (lut/color_palette.tbl)
Export consistency	Identical compression settings and metadata hash
Integration Points
Module	Direction	Purpose
diagnostics/metrics	Input	Source of numerical data
meta/chronicle	Input	Temporal sequence data
core/scripts/	Output	For embedding visual reports
docs/	Output	Automatically exported visuals for reports and papers
Validation Tests
Test	Description	Criterion
test_spiral_continuity	Verify smooth trajectory rendering	<1 px discontinuity at hue seam
test_drift_plot_accuracy	FFT drift curve vs raw data	±0.1% RMS error
test_heatmap_stability	Repeated render with same data	Identical hash output
test_dashboard_export	PNG + SVG file integrity	Valid format, <1% size variance
File Layout
visual/
├─ spec.md                       ← this document
├─ renderer.rs                   ← image/signal plotting engine
├─ chromatic_spiral.rs           ← hue–saturation spiral rendering
├─ spectral_drift.rs             ← spectral energy drift plots
├─ coherence_heatmap.rs          ← spectral stability visualizations
├─ dashboard.rs                  ← combined panels and export logic
├─ lut/
│   ├─ color_palette.tbl
│   └─ coherence_gradient.tbl
└─ tests/
    ├─ test_spiral_render.rs
    ├─ test_drift_plot.rs
    ├─ test_heatmap_render.rs

Status
Field	Value
Spec Version	1.1
**Phase Alignment**	**Phase 3**
Determinism Level	Bit-Exact Rendering
Dependencies	metrics, tensor, chronicle
**Next Module**	**Phase 4 - Dream Subsystem**
Readiness	✅ Ready for implementation