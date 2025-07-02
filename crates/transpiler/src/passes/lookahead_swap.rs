// This code is part of Qiskit.
//
// (C) Copyright IBM 2025
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
//
// Any modifications or derivative works of this code must retain this
// copyright notice, and modified files need to carry a notice indicating
// that they have been altered from the originals.

use qiskit_circuit::operations::StandardGate;
use qiskit_circuit::DAGCircuit;

struct system_state {
    layout: Vec<uszie>, // index is physcal qubit, value is logical qubit
    coupling_map: ArrayView2<f64>,
    register: Vec<usize>,
    swaps: Vec<(usize, usize)>,
}

// A step describes one possible step in the lookahead process
struct step {
    state: system_state,
    swaps_added: Vec<StandardGate>,
    gates_mapped: Vec<StandardGate>,
    gates_remaining: Vec<StandardGate>,
}

pub fn lookahead_swap(
    coupling_map: ArrayView2<f64>,
    search_depth: usize,
    search_width: usize,
    fake_run: False,
    dag: DAGCircuit) -> DAGCircuit
{
    // First do validity checks (lines 121-135)

    // Create first state

    // Init mapped gates and gates remaining

    let mut mapped_gates: Vec<> = vec![];
    let mut gates_remaining: Vec<> = vec![];

    let num_qubits: usize = dag.num_qubits();

    while (!gates_remaining.empty()) {
        let best_step: step = search_forward_n_swaps(
            current_state,
            gates_remaining,
            search_depth,
            search_width
        )

    }

}

fn search_forward_n_steps(state: system_state, gates: gates_remaining, depth: usize, width: usize) {

}

pub fn lookahead_swap_mod(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_function!(lookahead_swap))?;
    Ok(())
}
