use reqwest::{blocking::Response, Error};
use serde_json::Value;
use crate::search::state::SearchState;

pub fn get_search_results(search_state:&SearchState, url:String, search_params: Value) -> Result<Response, Error> {
    search_state.client
        .get(url)
        .header("X-TYPESENSE-API-KEY", &search_state.typesense_api_key)
        .query(&search_params)
        .send()
}

pub fn create_collection_schema(search_state:&SearchState, url:String, body: Value) -> Result<Response, Error> {
    search_state.client
        .post(url)
        .header("X-TYPESENSE-API-KEY", &search_state.typesense_api_key)
        .json(&body)
        .send()
}

pub fn delete_collection_schema(search_state:&SearchState, url:String) -> Result<Response, Error> {
    search_state.client
        .delete(url)
        .header("X-TYPESENSE-API-KEY", &search_state.typesense_api_key)
        .send()
}

pub fn insert_single_doc(search_state:&SearchState, url:String, mut body: Value) -> Result<Response, Error> {
    stringify_json_ids(&mut body);
    
    search_state.client
        .post(url)
        .header("X-TYPESENSE-API-KEY", &search_state.typesense_api_key)
        .json(&body)
        .send()
}

pub fn insert_batch_docs(search_state:&SearchState, url:String, mut batch_body: Value) -> Result<Response, Error> {
    
    if let Some(array) = batch_body.as_array_mut() {
        for doc_item in array {
            stringify_json_ids(doc_item);
        }
    }
    
    let jsonl_batch_body = json_array_to_jsonl(&batch_body);
 
    search_state.client
        .post(url)
        .header("X-TYPESENSE-API-KEY", &search_state.typesense_api_key)
        .body(jsonl_batch_body)
        .send()
}


fn json_array_to_jsonl(json_array: &Value) -> String {
    json_array
        .as_array()
        .expect("Expected a JSON array")
        .iter()
        .map(|val| val.to_string())
        .collect::<Vec<String>>()
        .join("\n")
    
}

// Convert postgresSql integer ids to typesense string ids for typsense compatibility
fn stringify_json_ids(doc_item: &mut Value){
    if let Some(id_value) = doc_item.get_mut("id") {
        let string_id = id_value.to_string();
        *id_value = Value::String(string_id);
    }
}


#[cfg(test)]
mod tests {
    use std::env;
    use reqwest::blocking::Client;
    use serde::Deserialize;
    use serde_json::{json, Value};
    use crate::{search::state::SearchState, services::search_service::{create_collection_schema, delete_collection_schema, insert_single_doc, insert_batch_docs, get_search_results}};

    #[test]
    fn test_create_delete_schema_success() {
        #[derive(Debug, Deserialize)]
        struct ResponseStruct {
            pub name: String,
        }

        let typesense_url = "http://localhost:8108/collections/".to_string();
        let client = Client::new();
        let typesense_api_key = env::var("TYPESENSE_API_KEY").expect("TYPESENSE_API_KEY must be set");
        let search_state = SearchState { client, typesense_url, typesense_api_key};
        let schema_name = "test1";
        let schema = json!({
            "name": schema_name,
            "fields": [
                {"name": ".*", "type": "auto" }
            ]
        });
        
        let result = create_collection_schema(&search_state.clone(), search_state.clone().typesense_url, schema);
        assert!(
            result.is_ok(),
            "Schema creation failed when it should have succeeded"
        );
        let response :ResponseStruct= result.expect("Schema creation failed").json().expect("Parsing failed");
        assert_eq!(response.name, schema_name);

        let mut deletion_url = "http://localhost:8108/collections/".to_string();
        deletion_url.push_str(schema_name);
        let deletion_result = delete_collection_schema(&search_state.clone(), deletion_url);
        assert!(
            deletion_result.is_ok(),
            "Schema deletion failed when it should have succeeded"
        );
        let deletion_response :ResponseStruct= deletion_result.expect("Schema deletion failed").json().expect("Parsing failed");
        assert_eq!(deletion_response.name, schema_name);
    }

    #[test]
    fn test_single_doc_success() {
        #[derive(Debug, Deserialize)]
        struct ResponseStruct {
            pub id: String,
        }
        
        let schema_name = "test2";
        let search_state = setup_test_environment(schema_name);

        let mut insertion_url = "http://localhost:8108/collections/".to_string();
        insertion_url.push_str(schema_name);
        insertion_url.push_str("/documents");
        let body = json!({
                "id": 12,
                "company_name": "Stark Industries",
                "num_employees": 5215
        });
        let insertion_result  = insert_single_doc(&search_state.clone(),insertion_url, body.clone());
        assert!(
            insertion_result.is_ok(),
            "Schema deletion failed when it should have succeeded"
        );
        let insertion_response :ResponseStruct= insertion_result.expect("Doc insertion failed").json().expect("Parsing failed");
        assert_eq!(insertion_response.id, body.get("id").unwrap().as_i64().unwrap().to_string());

        clear_test_environment(search_state, schema_name);
    }

    #[test]
    fn test_batch_success() {
        let schema_name = "test3";
        let search_state = setup_test_environment(schema_name);
        
        let mut insertion_url = "http://localhost:8108/collections/".to_string();
        insertion_url.push_str(schema_name);
        insertion_url.push_str("/documents");
        let body = json!([{
                "id": 12,
                "company_name": "Stark Industries",
                "num_employees": 5215
        }]);
        let batch_insertion_result  = insert_batch_docs(&search_state.clone(),insertion_url, body.clone());
        assert!(
            batch_insertion_result.is_ok(),
            "Schema deletion failed when it should have succeeded"
        );

        clear_test_environment(search_state, schema_name);
    }

    #[test]
    fn test_search_results_success() {#[derive(Debug, Deserialize)]
        struct SearchResult {
            pub found: usize,
            pub hits: Vec<Hit>,
        }
        
        #[derive(Debug, Deserialize)]
        struct Hit {
            document: Value
        }
        
        let schema_name = "test4";
        let search_state = setup_test_environment(schema_name);

        let mut insertion_url = "http://localhost:8108/collections/".to_string();
        insertion_url.push_str(schema_name);
        insertion_url.push_str("/documents");
        let body = json!({
                "id": 12,
                "company_name": "Stark Industries",
                "num_employees": 5215
        });
        let _  = insert_single_doc(&search_state.clone(), insertion_url.clone(), body.clone());
        
        insertion_url.push_str("/search");
        let search_query = serde_json::json!({
            "q": "Stark Industries",
            "query_by": "company_name"
        });
        let search_result = get_search_results(&search_state, insertion_url, search_query);
        assert!(
            search_result.is_ok(),
            "Schema deletion failed when it should have succeeded"
        );
        let search_response :SearchResult= search_result.expect("Doc insertion failed").json().expect("Parsing failed");
        assert_eq!(
            search_response.hits.get(0).unwrap().document.get("id").unwrap().as_str().unwrap(),
            body.get("id").unwrap().as_i64().unwrap().to_string()
        );
        clear_test_environment(search_state, schema_name);
    }


    fn setup_test_environment(schema_name: &str) -> SearchState {
        let typesense_url = "http://localhost:8108/collections/".to_string();
        let client = Client::new();
        let typesense_api_key = env::var("TYPESENSE_API_KEY").expect("TYPESENSE_API_KEY must be set");
        let search_state = SearchState { client, typesense_url, typesense_api_key};

        let schema = json!({
            "name": schema_name,
            "fields": [
                {"name": ".*", "type": "auto" }
            ]
        });
        let _ = create_collection_schema(&search_state.clone(), search_state.clone().typesense_url, schema);

        search_state
    }

    fn clear_test_environment(search_state: SearchState, schema_name: &str) {
        let mut deletion_url = "http://localhost:8108/collections/".to_string();
        deletion_url.push_str(schema_name);
        let _ = delete_collection_schema(&search_state.clone(), deletion_url);
    }
}
