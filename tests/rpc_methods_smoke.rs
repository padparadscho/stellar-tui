//! Smoke checks that key RPC method specs are present in the registered method list.

use stellar_tui::app::methods::method_specs;

#[test]
fn method_specs_include_core_entries() {
    let methods = method_specs();
    assert!(methods.iter().any(|method| method.name == "getHealth"));
    assert!(methods
        .iter()
        .any(|method| method.name == "getLatestLedger"));
    assert!(methods
        .iter()
        .any(|method| method.name == "simulateTransaction"));
}
