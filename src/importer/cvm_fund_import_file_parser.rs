use super::cvm_fund_importer::{ImportFile, ImporterError};
use chrono::NaiveDateTime;
use scraper::{element_ref::Select, Html, Selector};

impl From<reqwest::Error> for ImporterError {
    fn from(_error: reqwest::Error) -> Self {
        ImporterError::HttpError
    }
}

impl From<chrono::format::ParseError> for ImporterError {
    fn from(_error: chrono::format::ParseError) -> Self {
        ImporterError::ParseError
    }
}

pub async fn parse_import_files(url: &str) -> Result<Vec<ImportFile>, ImporterError> {
    let body = reqwest::get(url).await?.text().await?;
    let fragment = Html::parse_document(&body);

    Ok(parse_import_files_html(fragment)?)
}

fn parse_import_files_html(fragment: Html) -> Result<Vec<ImportFile>, ImporterError> {
    let file_selector = Selector::parse("table#indexlist tr.odd, table#indexlist tr.even").unwrap();
    let name_selector = Selector::parse(".indexcolname a").unwrap();
    let last_modified_selector = Selector::parse(".indexcollastmod").unwrap();

    let mut import_files = vec![];
    for file_row in fragment.select(&file_selector) {
        let name = first_inner_html(file_row.select(&name_selector))?;

        if !is_valid_file(&name) {
            continue;
        }

        let time_str = first_inner_html(file_row.select(&last_modified_selector))?;
        let time = NaiveDateTime::parse_from_str(&time_str.trim(), "%Y-%m-%d %H:%M")?;

        let import_file = ImportFile { name, time };

        import_files.push(import_file);
    }

    Ok(import_files)
}

fn first_inner_html(mut select: Select) -> Result<String, ImporterError> {
    match select.next() {
        Some(node) => Ok(node.inner_html()),
        None => Err(ImporterError::ParseError),
    }
}

fn is_valid_file(name: &str) -> bool {
    name.starts_with("inf_diario_fi_")
}
