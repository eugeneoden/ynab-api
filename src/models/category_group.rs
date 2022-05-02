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
pub struct CategoryGroup {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    /// Whether or not the category group is hidden
    #[serde(rename = "hidden")]
    pub hidden: bool,
    /// Whether or not the category group has been deleted.  Deleted category groups will only be included in delta requests.
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl CategoryGroup {
    pub fn new(id: String, name: String, hidden: bool, deleted: bool) -> CategoryGroup {
        CategoryGroup {
            id,
            name,
            hidden,
            deleted,
        }
    }
}


