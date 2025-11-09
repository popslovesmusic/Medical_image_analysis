# **AGENTS.MD: Medical Image Analysis - Monorepo Protocol**

## **Project Identifier: Medical Image Analysis**

**New Context:** All components are geared towards high-fidelity, deterministic anomaly detection in multi-channel medical scan data (MRI/CT).

## **🎯 PROJECT MISSION (Non-Negotiable)**

The goal is to maintain a **Deterministic, High-Fidelity Cognitive Architecture** (The Chromatic Semantic Archive - CSA) that processes structured data by translating it into a visual (color) and auditory (sound) space, supporting **automated anomaly scoring and diagnostic insight**.

## **🔒 I. NON-NEGOTIABLE ZAG CONSTRAINTS**

The following rules are **Zero Ambiguity Guarantees (ZAGs)** that the agent MUST enforce. Violating these is a critical failure.

### **A. Architectural & Data Integrity**

1.  **Primary Language:** **Rust (2021 Edition)**. Do not introduce Python, C++, or external FFI unless explicitly defined by spec.
2.  **Fixed Structure:** The core processing unit size is **immutable**: $\mathbf{3 \times 12 \times 12 \times 3}$. All code must reference centralized constants for these dimensions.
3.  **Numeric Precision:** Core tensor and neural operations MUST use $\mathbf{\text{f32}}$. $\mathbf{\text{f64}}$ is reserved ONLY for validation ($\mathbf{\Delta E_{94}}$) and memory bookkeeping.
4.  **Fidelity Threshold:** All round-trip color conversions must be tested and validated against $\mathbf{\Delta E_{94} \le 1.0 \times 10^{-3}}$.
5.  **Vocabulary Base:** The cognitive system's foundation is the **12-Category Base** (12 chromatic hues/tones). All encoding/decoding must use this base.

### **B. Safety & Performance**

1.  **Memory Safety:** Use **saturating arithmetic** for all usize calculations (e.g., memory budget, array sizing). Never use standard +/- for memory-critical operations.
2.  **Concurrency:** All structs used in parallel contexts ($\text{DreamEntry}$, $\text{ModalityMapper}$) must be explicitly $\mathbf{\text{Send}}$ and $\mathbf{\text{Sync}}$. Prefer $\text{Rayon}$ for parallelism.
3.  **Error Handling:** All failures must be captured as a $\mathbf{\text{DreamError}}$ variant. Use $\mathbf{\text{tracing::error!}}$ for logging errors that lead to deterministic fallbacks.

## **II. PROJECT STRUCTURE & COMPANION DOCUMENTS**

The project is structured as a **Cargo Workspace** to enforce separation of concerns and auditability.

### **A. Authoritative Implementation Plan**

This document provides the high-level governance (ZAGs) and crate structure. The authoritative, granular implementation plan, phase definitions, and checkpoints are defined in the companion document:

* **`IMPLEMENTATION_CHECKLIST.md`**: The 9-phase granular checklist. All development MUST follow the phases and checkpoints defined in this file.

### **B. Monorepo Layout (Cargo Workspace)**

| Directory | Crate Name | Primary Responsibility |
| :--- | :--- | :--- |
| **/core** | chromatic-core | **Execution Engine:** $\text{Tensor}$ operations, $\text{CSA}$ (Dream Pool), $\text{ModalityMapper}$, $\text{HNSW}$. |
| **/trainer** | tiny-agent-trainer | **Generative Engine:** $\text{Transformer Model}$, $\text{WGSL}$ generation, $\text{Tokenizer}$, $\text{Validator}$. |
| **/Cargo.toml (root)** | \[workspace\] | Defines the dependency links and build members. |

## **III. SEQUENTIAL DEVELOPMENT PHASES**

Implementation MUST follow the 9-phase roadmap defined in **`IMPLEMENTATION_CHECKLIST.md`**. That document is the Single Source of Truth for the development sequence, sub-phases, and validation checkpoints.

*(Previous 5-phase definition removed to prevent conflict with the authoritative checklist.)*

## **IV. TECHNICAL PROTOCOLS (Agent Workflow)**

### **A. Output and Git Protocol**

1.  **Output Delivery:** Always provide a **complete, correct file** for any module requested. Do not provide partial files or code snippets.
2.  **Testing:** **Do not commit code that fails any existing test.** Run `cargo check` and `cargo fmt` before presenting any output.
3.  **Commit Message:** Use the standard format: `feat(module): brief description` (e.g., `feat(dream): implemented 10% churn threshold`).

### **B. Tooling and Dependencies**

1.  **Scientific Support:** When mathematical or array manipulation is required, use the $\mathbf{\text{ndarray}}$ and $\mathbf{\text{Rayon}}$ crates.
2.  **GPU:** Use the **wgpu** crate and **WGSL** for all visualization and $\text{Compute Shader}$ tasks.
3.  **Dependencies:** Do not add new top-level dependencies without explicit confirmation.