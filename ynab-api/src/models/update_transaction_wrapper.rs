/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTransactionWrapper {
    #[serde(rename = "transaction")]
    pub transaction: ::models::UpdateTransaction,
}

impl UpdateTransactionWrapper {
    pub fn new(transaction: ::models::UpdateTransaction) -> UpdateTransactionWrapper {
        UpdateTransactionWrapper {
            transaction: transaction,
        }
    }
}


