use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub type_field: String,
    pub data: Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    pub created: String,
    pub description: String,
    pub amount: i64,
    pub fees: Fees,
    pub currency: String,
    pub merchant: ::serde_json::Value,
    pub notes: String,
    pub metadata: Metadata,
    pub labels: ::serde_json::Value,
    pub account_balance: i64,
    pub attachments: ::serde_json::Value,
    pub international: ::serde_json::Value,
    pub category: String,
    pub categories: ::serde_json::Value,
    pub is_load: bool,
    pub settled: String,
    pub local_amount: i64,
    pub local_currency: String,
    pub updated: String,
    pub account_id: String,
    pub user_id: String,
    pub counterparty: Counterparty,
    pub scheme: String,
    pub dedupe_id: String,
    pub originator: bool,
    pub include_in_spending: bool,
    pub can_be_excluded_from_breakdown: bool,
    pub can_be_made_subscription: bool,
    pub can_split_the_bill: bool,
    pub can_add_to_tab: bool,
    pub amount_is_pending: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Fees {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Metadata {
    FasterPayment {
        faster_payment: String,
        fps_fpid: String,
        fps_payment_id: String,
        insertion: String,
        notes: String,
        trn: String,
    },
    Pot {
        external_id: String,
        ledger_insertion_id: String,
        pot_account_id: String,
        pot_deposit_id: String,
        pot_id: String,
        trigger: String,
        user_id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Counterparty {
    FasterPayment {
        account_number: String,
        name: String,
        sort_code: String,
        user_id: String,
    },
    Pot {},
}
