use super::{
    method::{
        get_events, get_fee_stats, get_health, get_latest_ledger, get_ledger_entries, get_ledgers,
        get_network, get_transaction, get_transactions, get_version_info, send_transaction,
        simulate_transaction,
    },
    MethodSpec,
};

/// Returns the complete, ordered list of supported RPC method specs
pub fn method_specs() -> Vec<MethodSpec> {
    // Order here defines method list order in the UI
    vec![
        get_events::spec(),
        get_fee_stats::spec(),
        get_health::spec(),
        get_latest_ledger::spec(),
        get_ledger_entries::spec(),
        get_ledgers::spec(),
        get_network::spec(),
        get_transaction::spec(),
        get_transactions::spec(),
        get_version_info::spec(),
        send_transaction::spec(),
        simulate_transaction::spec(),
    ]
}
