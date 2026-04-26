use serde::Deserialize;

pub struct HtmlContract {
    pub id: i16,
    pub signature_id: String,
    pub contract_url: String,
    pub term_url: String,
}

#[derive(Deserialize)]
pub struct CreatePdf {
    pub file_path: String,
    pub bucket_path: String,
}
