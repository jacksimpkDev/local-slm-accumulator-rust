# High-Throughput Clinical-Legal Ingestion Pipeline: The Two-Pass Accumulator Pattern in Rust

A high-performance, local-first processing pipeline that decouples **stateless Small Language Model (SLM) extraction** from **stateful sequence assembly**. 

Built in Rust, this architectural pattern solves the "cognitive overload trap" of running local LLMs (like Llama 3.2 3B) on multi-thousand-page clinical document dumps.

---

## 🚀 The Core Problem & Architectural Shift

In clinical-legal document triage (e.g., forensic medical review, insurance claims, or litigation preparation), a single PDF upload often contains years of back-to-back medical encounters. Conversely, a single physical report can span dozens of pages.

Most automated architectures attempt to use a stateless LLM to classify page boundaries on the fly (e.g., asking the model to guess if a page is a "start" or "continuation" in isolation). Under resource-constrained local compute profiles (e.g., < 2.5 GB RAM), this approach fails catastrophically due to context-window exhaustion and hallucinations (such as extracting the patient's name as the medical provider).

### The Solution: The Decoupled Two-Pass Accumulator
This architecture splits the ingestion pipeline into two highly specialized passes:

1. **Pass 1 (Stateless SLM Extraction):** The local SLM acts strictly as a raw, stateless key-value parser. It evaluates each page in absolute isolation, extracting only visible, literal facts. If a field is missing, it returns `null`. It carries zero state and does no boundary reasoning.
2. **Pass 2 (Stateful Rust Assembly):** A deterministic Rust state machine loops through the sequential page-level metadata. It evaluates temporal shifts, detects structural template headers, and dynamically clusters pages into discrete logical "Encounters," merging sparse/missing metadata forward.

PASS 1: STATELESS EXTRACTION
                   ┌──────────────────────────────────┐
                   │       Page 1 OCR Text            │
                   └────────────────┬─────────────────┘
                                    │
                                    ▼
                   ┌──────────────────────────────────┐
                   │   Local SLM (e.g., Llama 3.2 3B) │
                   │    - Simple Key-Value Extraction │
                   │    - Zero logical reasoning      │
                   └────────────────┬─────────────────┘
                                    │
                                    ▼
                   ┌──────────────────────────────────┐
                   │     JSON Page Extraction         │
                   │ {date: "2020-05-22", prov: null} │
                   └────────────────┬─────────────────┘
                                    │
                                    │ (Deliver sequence to Rust)
                                    ▼
                        PASS 2: STATEFUL ASSEMBLY
                   ┌──────────────────────────────────┐
                   │   Rust Deterministic Loop        │
                   │   - Evaluates Date Shifts        │
                   │   - Detects Structural Headers   │
                   │   - Sweeps out Patient Names     │
                   │   - Dynamically merges metadata  │
                   └────────────────┬─────────────────┘
                                    │
                                    ▼
                   ┌──────────────────────────────────┐
                   │   Clean Assembled Encounters     │
                   │   [Pages 1-4], [Pages 5-8]       │
                   └──────────────────────────────────┘

---

## 🛠️ Key Architectural Features

### 1. The "Granularity Shield" (Anti-Minutia Logic)
Prevents the common "OCR noise trap" (experienced in platforms like Casefleet) where users are swamped in thousands of redundant billing or administrative nodes. This pipeline groups pages logically into macroscopic "Encounters" before extracting granular "Facts."

### 2. Late-Binding Bates Mapping
Allows a medical expert to analyze a case from day one using raw physical page offsets. When legal counsel eventually applies official Bates stamps to the discovery packet weeks later, the system dynamically calculates and binds the legal citations on the fly without requiring the user to rewrite a single fact.

$$\text{Bates Citation} = \text{Bates Prefix} + (\text{Start Number} + \text{Page Offset})$$

---

## 📂 Codebase Architecture

This repository is structured as a self-contained console simulation showcasing the logical pipeline without requiring heavy local GPU or LLM dependencies:

* **`src/accumulator.rs`**: Houses the core stateful grouping and metadata merging algorithm. It implements the state-machine loop that evaluates page transitions and sweeps missing values forward.
* **`src/bates.rs`**: Implements the late-binding calculation engine that dynamically switches between local physical page fallback references and real-world serial Bates indices.
* **`src/main.rs`**: Coordinates the runner, loading an 18-page mock scenario representing sparse OCR/SLM outputs to run through the pipeline phases.

---

## ⚡ Quick Start & Verification

This project runs as a zero-dependency local simulation.

### Prerequisite
Ensure you have Rust and Cargo installed ([rustup.rs](https://rustup.rs/)).

### Run the Simulation
```bash
# Clone the repository
git clone [https://github.com/jacksimpkDev/local-slm-accumulator-rust.git](https://github.com/jacksimpkDev/local-slm-accumulator-rust.git)
cd local-slm-accumulator-rust

# Run the pipeline console demonstration
cargo run

# Execute the deterministic integration and unit tests
cargo test

## Simulation Output Guide

When executing `cargo run`, the program coordinates through four distinct operational phases:

### Phase 1: Raw Stateless Page Extractions
Displays the simulated raw JSON extractions received from the local SLM. Note the sparse data; because the model runs statelessly page-by-page, details like "Provider" and "Facility" are often missing (`None`) on continuation pages where a header was not physically re-printed.

### Phase 2: Stateful Assembly
Shows the Rust state machine processing the page array. It successfully identifies boundaries based on date shifts and template flags, groups the physical pages into logical encounters, and merges metadata forward so that no page is left orphaned.

### Phase 3: Physical Page Citation Fallback
Demonstrates the active "Late-Binding" timeline before a Bates profile is applied. Events are cited strictly using physical file page offsets (e.g., `Physical Page 1`).

### Phase 4: Applied Bates Mapping
Demonstrates the late-binding switch. After assigning a Bates profile (Prefix: `CHAMBERS`, Start Index: `100`), the entire timeline is instantly recalculated. Physical Page 1 dynamically references `CHAMBERS_000100` and Physical Page 15 maps to `CHAMBERS_000114` without modifying the underlying analyzed facts.

