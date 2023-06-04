use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
// mod utils;
use wasm_bindgen::prelude::*;
use automaton_evaluator::nfa::NFA;
use automaton_evaluator::traits::Acceptor;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// We only use the NFA implementation here; that should work for DFA's too.
#[wasm_bindgen]
pub struct WasmNFA {
    // we need to put the wasm_bindgen attribute on everything that needs to be exposed to JS
    // i didn't wanna pollute the library with wasm stuff, so I'm just wrapping the library
    // functionality in new methods/structs here that have the wasm_bindgen attribute set
    nfa: NFA
}

#[derive(Deserialize, Serialize)]
pub struct NfaArgs {
    pub states: HashMap<usize, String>,
    pub initial_state: usize,
    pub final_states: HashSet<usize>,
    pub transitions: Vec<Vec<Vec<char>>>
}

#[wasm_bindgen]
impl WasmNFA {
    #[wasm_bindgen(constructor)]
    pub fn new(args: JsValue) -> Self {
        let nfa_args: NfaArgs = serde_wasm_bindgen::from_value(args).unwrap();
        Self {
            nfa: NFA::new(nfa_args.states, nfa_args.initial_state, nfa_args.final_states, nfa_args.transitions)
        }
    }

    #[wasm_bindgen]
    pub fn test_string(&self, s: String) -> bool {
        self.nfa.test_string(s)
    }
}
