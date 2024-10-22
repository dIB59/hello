use reqwest::blocking::Client;


#[derive(Clone)]
pub (crate) struct SearchState  {
    pub client : Client,
    pub typesense_url : String,
    pub typesense_api_key : String,
}