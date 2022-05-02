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
pub struct CategoriesResponseData {
    #[serde(rename = "category_groups")]
    pub category_groups: Vec<crate::models::CategoryGroupWithCategories>,
    /// The knowledge of the server
    #[serde(rename = "server_knowledge")]
    pub server_knowledge: i64,
}

impl CategoriesResponseData {
    pub fn new(category_groups: Vec<crate::models::CategoryGroupWithCategories>, server_knowledge: i64) -> CategoriesResponseData {
        CategoriesResponseData {
            category_groups,
            server_knowledge,
        }
    }
}


