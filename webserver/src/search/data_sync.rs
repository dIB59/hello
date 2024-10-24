use diesel::prelude::*;
use reqwest::{blocking::Response, Error};
use serde::Serialize;
use serde_json::json;
use crate::{search::index::index_docs, services::{job_service, search_service}, state::SearchState};


pub fn init_tpyesense(connection: &mut PgConnection, search_state : SearchState){
    let schema_names = vec!["jobs"];

    for schema_name in schema_names{
        match create_collection(search_state.clone(), schema_name){
            Ok(response) =>
                if response.status().is_success(){
                    fetch_and_index(connection, search_state.clone(), schema_name)
                }else {
                    log::debug!("Typesense collection '{}' already exist", schema_name)
                },
            Err(_) => log::debug!("Typesense connection failed")
        }
    }
}


fn create_collection(search_state : SearchState, schema_name : &str) -> Result<Response,Error>{
    let typesense_collections_url = format!("{}/collections",search_state.typesense_url);
    
    // Create an auto detection schema that's compatible with all database tables
    let schema = json!({
        "name": *schema_name,  
        "fields": [
            {"name": ".*", "type": "auto" }
        ]
      });
    
    search_service::create_collection_schema(&search_state, typesense_collections_url, schema)
}


fn fetch_and_index(connection: &mut PgConnection, search_state : SearchState, schema_name : &str) {
    match schema_name{
        "jobs" => fetch_and_index_docs(job_service::get_jobs(connection) , search_state, schema_name),
        _ => ()
    }
}


fn fetch_and_index_docs<Doc: Serialize>(
    fetch_result : Result<Vec<Doc>,
    diesel::result::Error>,
    search_state : SearchState,
    schema_name : &str
){
    match fetch_result{
        Ok(docs) =>
            if let Err(err) = index_docs(docs, search_state, schema_name) {
                log::debug!("Error indexing {}: {}",schema_name, err);
            },
        Err(err) => 
            log::debug!("Error fetching {}: {}",schema_name, err)
    }
}