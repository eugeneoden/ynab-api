/*
 * YNAB API Endpoints
 *
 * Our API uses a REST based design, leverages the JSON data format, and relies upon HTTPS for transport. We respond with meaningful HTTP response codes and if an error occurs, we include error details in the response body.  API Documentation is at https://api.youneedabudget.com
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;

use super::{Error, configuration};

pub struct CategoriesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl CategoriesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> CategoriesApiClient {
        CategoriesApiClient {
            configuration: configuration,
        }
    }
}

pub trait CategoriesApi {
    fn get_categories(&self, budget_id: &str, last_knowledge_of_server: i64) -> Result<crate::models::CategoriesResponse, Error>;
    fn get_category_by_id(&self, budget_id: &str, category_id: &str) -> Result<crate::models::CategoryResponse, Error>;
    fn get_month_category_by_id(&self, budget_id: &str, month: String, category_id: &str) -> Result<crate::models::CategoryResponse, Error>;
    fn update_month_category(&self, budget_id: &str, month: String, category_id: &str, data: crate::models::SaveMonthCategoryWrapper) -> Result<crate::models::SaveCategoryResponse, Error>;
}

impl CategoriesApi for CategoriesApiClient {
    fn get_categories(&self, budget_id: &str, last_knowledge_of_server: i64) -> Result<crate::models::CategoriesResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/budgets/{budget_id}/categories", configuration.base_path, budget_id=crate::apis::urlencode(budget_id));
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("last_knowledge_of_server", &last_knowledge_of_server.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_category_by_id(&self, budget_id: &str, category_id: &str) -> Result<crate::models::CategoryResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/budgets/{budget_id}/categories/{category_id}", configuration.base_path, budget_id=crate::apis::urlencode(budget_id), category_id=crate::apis::urlencode(category_id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_month_category_by_id(&self, budget_id: &str, month: String, category_id: &str) -> Result<crate::models::CategoryResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/budgets/{budget_id}/months/{month}/categories/{category_id}", configuration.base_path, budget_id=crate::apis::urlencode(budget_id), month=month, category_id=crate::apis::urlencode(category_id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn update_month_category(&self, budget_id: &str, month: String, category_id: &str, data: crate::models::SaveMonthCategoryWrapper) -> Result<crate::models::SaveCategoryResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/budgets/{budget_id}/months/{month}/categories/{category_id}", configuration.base_path, budget_id=crate::apis::urlencode(budget_id), month=month, category_id=crate::apis::urlencode(category_id));
        let mut req_builder = client.patch(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.header("Authorization", val);
        };
        req_builder = req_builder.json(&data);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
