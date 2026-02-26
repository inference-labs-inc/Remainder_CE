//! Per-thread ID counters for deterministic circuit construction.
//!
//! LayerIds assigned during circuit construction are embedded in the
//! [`GKRCircuitDescription`] and included in the circuit description hash.
//! Prover and verifier must independently build the same circuit and arrive
//! at the same hash.  Using a process-global counter caused the verifier's
//! IDs to diverge from the prover's whenever any IDs were allocated between
//! the two builds (e.g. during proving, or by concurrent circuit builds in
//! tests).
//!
//! The counters are now thread-local and reset to zero at the start of every
//! [`CircuitBuilder::new()`] call, guaranteeing deterministic assignment
//! within each build.

use std::cell::Cell;

thread_local! {
    static NODE_ID: Cell<usize> = const { Cell::new(0) };
    static INPUT_LAYER_ID: Cell<usize> = const { Cell::new(0) };
    static LAYER_ID: Cell<usize> = const { Cell::new(0) };
    static FIAT_SHAMIR_CHALLENGE_LAYER_ID: Cell<usize> = const { Cell::new(0) };
}

/// Provides deterministic, per-thread ID allocation for circuit nodes and
/// layers.  Call [`reset()`](Self::reset) before each independent circuit
/// build (this is done automatically by `CircuitBuilder::new()`).
#[derive(Debug, Default)]
pub struct CircuitBuildingContext;

impl CircuitBuildingContext {
    /// Resets all counters to zero for the current thread.
    pub fn reset() {
        NODE_ID.set(0);
        INPUT_LAYER_ID.set(0);
        LAYER_ID.set(0);
        FIAT_SHAMIR_CHALLENGE_LAYER_ID.set(0);
    }

    pub fn next_node_id() -> usize {
        NODE_ID.with(|c| {
            let v = c.get();
            c.set(v + 1);
            v
        })
    }

    pub fn next_input_layer_id() -> usize {
        INPUT_LAYER_ID.with(|c| {
            let v = c.get();
            c.set(v + 1);
            v
        })
    }

    pub fn next_layer_id() -> usize {
        LAYER_ID.with(|c| {
            let v = c.get();
            c.set(v + 1);
            v
        })
    }

    pub fn next_fiat_shamir_challenge_layer_id() -> usize {
        FIAT_SHAMIR_CHALLENGE_LAYER_ID.with(|c| {
            let v = c.get();
            c.set(v + 1);
            v
        })
    }
}
