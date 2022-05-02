/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BudgetDetailAllOf {
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<crate::models::Account>>,
    #[serde(rename = "payees", skip_serializing_if = "Option::is_none")]
    pub payees: Option<Vec<crate::models::Payee>>,
    #[serde(rename = "payee_locations", skip_serializing_if = "Option::is_none")]
    pub payee_locations: Option<Vec<crate::models::PayeeLocation>>,
    #[serde(rename = "category_groups", skip_serializing_if = "Option::is_none")]
    pub category_groups: Option<Vec<crate::models::CategoryGroup>>,
    #[serde(rename = "categories", skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<crate::models::Category>>,
    #[serde(rename = "months", skip_serializing_if = "Option::is_none")]
    pub months: Option<Vec<crate::models::MonthDetail>>,
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<crate::models::TransactionSummary>>,
    #[serde(rename = "subtransactions", skip_serializing_if = "Option::is_none")]
    pub subtransactions: Option<Vec<crate::models::SubTransaction>>,
    #[serde(rename = "scheduled_transactions", skip_serializing_if = "Option::is_none")]
    pub scheduled_transactions: Option<Vec<crate::models::ScheduledTransactionSummary>>,
    #[serde(rename = "scheduled_subtransactions", skip_serializing_if = "Option::is_none")]
    pub scheduled_subtransactions: Option<Vec<crate::models::ScheduledSubTransaction>>,
}

impl BudgetDetailAllOf {
    pub fn new() -> BudgetDetailAllOf {
        BudgetDetailAllOf {
            accounts: None,
            payees: None,
            payee_locations: None,
            category_groups: None,
            categories: None,
            months: None,
            transactions: None,
            subtransactions: None,
            scheduled_transactions: None,
            scheduled_subtransactions: None,
        }
    }
}


