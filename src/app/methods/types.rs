use serde_json::Value;
use std::collections::HashMap;

use crate::app::core::forms::{FieldSpec, FormState};

use super::{
    method::{
        get_events, get_fee_stats, get_health, get_latest_ledger, get_ledger_entries, get_ledgers,
        get_network, get_transaction, get_transactions, get_version_info, send_transaction,
        simulate_transaction,
    },
    validation,
};

/// Discriminant for each supported Stellar RPC method
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodId {
    GetEvents,
    GetFeeStats,
    GetHealth,
    GetLatestLedger,
    GetLedgerEntries,
    GetLedgers,
    GetNetwork,
    GetTransaction,
    GetTransactions,
    GetVersionInfo,
    SendTransaction,
    SimulateTransaction,
}

/// Full specification for an RPC method: metadata, fields, and help text
#[derive(Debug, Clone)]
pub struct MethodSpec {
    pub id: MethodId,
    pub name: &'static str,
    pub http_method: &'static str,
    pub fields: Vec<FieldSpec>,
    pub help: &'static str,
}

impl MethodSpec {
    /// Builds JSON RPC params from current form values
    pub fn build_params(&self, form: &FormState) -> Result<Value, String> {
        // Per method builders keep RPC specific payload shape close to field definitions
        match self.id {
            MethodId::GetEvents => get_events::build(form),
            MethodId::GetFeeStats => get_fee_stats::build(form),
            MethodId::GetHealth => get_health::build(form),
            MethodId::GetLatestLedger => get_latest_ledger::build(form),
            MethodId::GetLedgerEntries => get_ledger_entries::build(form),
            MethodId::GetLedgers => get_ledgers::build(form),
            MethodId::GetNetwork => get_network::build(form),
            MethodId::GetTransaction => get_transaction::build(form),
            MethodId::GetTransactions => get_transactions::build(form),
            MethodId::GetVersionInfo => get_version_info::build(form),
            MethodId::SendTransaction => send_transaction::build(form),
            MethodId::SimulateTransaction => simulate_transaction::build(form),
        }
    }

    /// Validates form values and returns field errors
    pub fn validate(&self, form: &FormState) -> HashMap<String, String> {
        // Validation remains centralized so UI can reuse one error model for all methods
        validation::validate_request_form(self, form)
    }
}
