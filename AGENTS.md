# **AGENTS.MD: Medical Image Analysis \- Monorepo Protocol**

## **Project Identifier: Medical Image Analysis**

**New Context:** All components are geared towards high-fidelity, deterministic anomaly detection in multi-channel medical scan data (MRI/CT).

## **🎯 PROJECT MISSION (Non-Negotiable)**

The goal is to maintain a **Deterministic, High-Fidelity Cognitive Architecture** (The Chromatic Semantic Archive \- CSA) that processes structured data by translating it into a visual (color) and auditory (sound) space, supporting **automated anomaly scoring and diagnostic insight**.

## **🔒 I. NON-NEGOTIABLE ZAG CONSTRAINTS**

The following rules are **Zero Ambiguity Guarantees (ZAGs)** that the agent MUST enforce. Violating these is a critical failure.

### **A. Architectural & Data Integrity**

1. **Primary Language:** **Rust (2021 Edition)**. Do not introduce Python, C++, or external FFI unless explicitly defined by spec.  
2. **Fixed Structure:** The core processing unit size is **immutable**: $\\mathbf{3 \\times 12 \\times 12 \\times 3}$. All code must reference centralized constants for these dimensions.  
3. **Numeric Precision:** Core tensor and neural operations MUST use $\\mathbf{\\text{f32}}$. $\\mathbf{\\text{f64}}$ is reserved ONLY for validation ($\\mathbf{\\Delta E\_{94}}$) and memory bookkeeping.  
4. **Fidelity Threshold:** All round-trip color conversions must be tested and validated against $\\mathbf{\\Delta E\_{94} \\le 1.0 \\times 10^{-3}}$.  
5. **Vocabulary Base:** The cognitive system's foundation is the **12-Category Base** (12 chromatic hues/tones). All encoding/decoding must use this base.

### **B. Safety & Performance**

1. **Memory Safety:** Use **saturating arithmetic** for all usize calculations (e.g., memory budget, array sizing). Never use standard \+/\- for memory-critical operations.  
2. **Concurrency:** All structs used in parallel contexts ($\\text{DreamEntry}$, $\\text{ModalityMapper}$) must be explicitly $\\mathbf{\\text{Send}}$ and $\\mathbf{\\text{Sync}}$. Prefer $\\text{Rayon}$ for parallelism.  
3. **Error Handling:** All failures must be captured as a $\\mathbf{\\text{DreamError}}$ variant. Use $\\mathbf{\\text{tracing::error\!}}$ for logging errors that lead to deterministic fallbacks.

## **II. PROJECT STRUCTURE (Monorepo Layout)**

The project is structured as a **Cargo Workspace** to enforce separation of concerns and auditability.

| Directory | Crate Name | Primary Responsibility |
| :---- | :---- | :---- |
| **/core** | chromatic-core | **Execution Engine:** $\\text{Tensor}$ operations, $\\text{CSA}$ (Dream Pool), $\\text{ModalityMapper}$, $\\text{HNSW}$. |
| **/trainer** | tiny-agent-trainer | **Generative Engine:** $\\text{Transformer Model}$, $\\text{WGSL}$ generation, $\\text{Tokenizer}$, $\\text{Validator}$. |
| **/Cargo.toml (root)** | \[workspace\] | Defines the dependency links and build members. |

## **III. SEQUENTIAL DEVELOPMENT PHASES**

The agent MUST follow the chronological reference to ensure no critical dependencies are missed.

### **1\. Phase 1: Foundational Infrastructure (Memory & Types)**

* **Goal:** Establish the fixed memory and structural environment.  
* **Key Action:** Implement $\\text{ndarray}$ types and memory budget with **saturating arithmetic**.  
* **Reference:** ZAG Constraints I.A and I.B.

### **2\. Phase 2: Cognitive Modality Integration (UMS & Encoding)**

* **Goal:** Implement the deterministic semantic translation protocols.  
* **Subphases:**  
  * **2.A:** Implement the $\\mathbf{12 \\text{-Hue \\ Category \\ Map}}$ (Nearest Neighbor logic).  
  * **2.B:** Implement $\\text{UMS \\ Encoding}$ with $\\mathbf{\\text{f16 \\ Compression}}$ and $\\mathbf{\\mu}/\\mathbf{\\sigma}$ normalization.  
* **Constraint:** Must pass $\\mathbf{\\Delta E\_{94}}$ round-trip fidelity check.

### **3\. Phase 3: Semantic Archive Finalization (Retrieval)**

* **Goal:** Harden the $\\text{CSA}$ retrieval mechanism for production use.  
* **Subphases:**  
  * Implement $\\mathbf{\\text{HNSW \\ Stabilization}}$ (Incremental updates, ghost node management).  
  * Implement $\\text{Auto-Scaling Logic}$ ($\\text{Linear} \\leftrightarrow \\text{HNSW}$ switch at $\\mathbf{\\sim 3000}$ entries).  
  * Implement $\\text{retrieve\\\_hybrid}$ and $\\text{retrieve\\\_semantic}$ functions.  
* **Constraint:** All retrieval functions must correctly use the $\\text{hue\\\_category}$ for partitioning.

### **4\. Phase 4: Generative Engine Implementation (Trainer)**

* **Goal:** Implement the code generation capability.  
* **Subphases:**  
  * Implement the **Transformer Encoder-Decoder Model** ($\\mathbf{d\\\_{model}=512}$ stack).  
  * Implement $\\text{Training}$ **and** $\\text{Evaluation}$ **Pipelines** ($\\text{Cross-Entropy}$ loss, $\\text{AdamW}$ optimizer).  
  * Implement $\\text{WGSL \\ Validator}$ and $\\text{Tokenizer}$.  
* **Constraint:** The model MUST be serializable via the $\\text{Checkpointing \\ API}$.

### **5\. Phase 5: Production & Application Pivot**

* **Goal:** Finalize external application hooks and deployment readiness.  
* **Subphases:**  
  * Implement $\\text{Auditory \\ Processing \\ Module (APM)}$ for external sensor/medical data intake.  
  * Implement $\\text{CSI}$ $\\text{WGSL/wgpu}$ pipeline for visualization.  
  * Finalize $\\text{Build \\ Artifacts}$ (statically linked binary, $\\text{FINAL\\\_BASELINE}$ report).

## **IV. TECHNICAL PROTOCOLS (Agent Workflow)**

### **A. Output and Git Protocol**

1. **Output Delivery:** Always provide a **complete, correct file** for any module requested. Do not provide partial files or code snippets.  
2. **Testing:** **Do not commit code that fails any existing test.** Run cargo check and cargo fmt before presenting any output.  
3. **Commit Message:** Use the standard format: feat(module): brief description (e.g., feat(dream): implemented 10% churn threshold).

### **B. Tooling and Dependencies**

1. **Scientific Support:** When mathematical or array manipulation is required, use the $\\mathbf{\\text{ndarray}}$ and $\\mathbf{\\text{Rayon}}$ crates.  
2. **GPU:** Use the **wgpu** crate and **WGSL** for all visualization and $\\text{Compute \\ Shader}$ tasks.  
3. **Dependencies:** Do not add new top-level dependencies without explicit confirmation.  
1. 