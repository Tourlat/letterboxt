// film_scraper.rs


use crate::film_error::FilmError;
use scraper::{Html, Selector};
use reqwest::StatusCode;

#[derive(Debug)]
pub struct FilmMetaData {
    pub rating: f32,
    pub director: String,
    pub release_year: String,
    pub synopsis: String,
    pub genres: Vec<String>,
}

async fn get_html_content(url: &str) -> Result<String, FilmError> {
    let response = reqwest::get(url).await.map_err(FilmError::ReqwestError)?;
    let status = response.status();
    let text = response.text().await.map_err(FilmError::ReqwestError)?;
    if status == StatusCode::NOT_FOUND || text.contains("<title>Letterboxd - Not Found</title>") {
        return Err(FilmError::NotFoundError);
    }
    Ok(text)    
}

fn build_film_url(film_name: &str) -> String {
    let film_name = film_name.to_lowercase().replace(":","").replace(" ", "-");
    return format!("https://letterboxd.com/film/{}", film_name);
}

pub async fn fetch_and_parse_html(film_name: &str) -> Result<Html, FilmError> {
    let film_url = build_film_url(film_name);
    println!("Fetching {}", film_url);
    let html_content = get_html_content(&film_url).await?;
    let document = Html::parse_document(&html_content);
    Ok(document)
}

fn get_film_rating(document: &Html, selector: &Selector) -> Result<f32, FilmError> {
    let element = document
        .select(selector)
        .next()
        .ok_or(FilmError::SelectorError)?;

    let content = element
        .value()
        .attr("content")
        .ok_or(FilmError::AttributeError)?;

    let rating: f32 = content
        .split_whitespace()
        .next()
        .ok_or(FilmError::ParseError)?
        .parse()
        .map_err(|_| FilmError::ParseError)?;

    Ok(rating)
}

fn get_film_single_info(document: &Html, selector: &Selector) -> Result<String, FilmError> {
    let info = document
        .select(selector)
        .next()
        .ok_or(FilmError::SelectorError)?
        .text()
        .collect::<String>();
    Ok(info)
}

fn get_film_multiples_infos(
    document: &Html,
    selector: &Selector,
) -> Result<Vec<String>, FilmError> {
    let infos = document.select(selector);
    let mut infos_vec = Vec::new();

    for info in infos {
        infos_vec.push(info.text().collect::<String>());
    }

    Ok(infos_vec)
}

fn extract_release_year(document: &Html) -> Result<String, FilmError> {
    let selector = Selector::parse("div.releaseyear a").unwrap();
    get_film_single_info(document, &selector)
}

fn extract_director(document: &Html) -> Result<String, FilmError> {
    let selector = Selector::parse("span.directorlist a span.prettify").unwrap();
    get_film_single_info(document, &selector)
}

fn extract_synopsis(document: &Html) -> Result<String, FilmError> {
    let selector = Selector::parse("div.truncate p").unwrap();
    get_film_single_info(document, &selector)
}
fn extract_rating(document: &Html) -> Result<f32, FilmError> {
    let selector = Selector::parse(r#"meta[name="twitter:data2"]"#).unwrap();
    get_film_rating(document, &selector)
}

fn extract_genres(document: &Html) -> Result<Vec<String>, FilmError> {
    let selector = Selector::parse("div.text-sluglist.capitalize a.text-slug").unwrap();
    get_film_multiples_infos(document, &selector)
}

pub async fn extract_film_meta_datas(film_name: &str) -> Result<FilmMetaData, FilmError> {
    let document = fetch_and_parse_html(film_name).await?;

    extract_datas_from_doc(&document)
}

pub async fn extract_film_meta_datas_from_url(url: &str) -> Result<FilmMetaData, FilmError> {
    let html_content = get_html_content(&url).await?;
    let document = Html::parse_document(&html_content);

    extract_datas_from_doc(&document)
}

fn extract_datas_from_doc(document: &Html) -> Result<FilmMetaData, FilmError> {
    let release_year = extract_release_year(&document)?;
    let director = extract_director(&document)?;
    let synopsis = extract_synopsis(&document)?;
    let rating = extract_rating(&document)?;
    let genres = extract_genres(&document)?;

    Ok(FilmMetaData {
        rating,
        director,
        release_year,
        synopsis,
        genres,
    })
}

