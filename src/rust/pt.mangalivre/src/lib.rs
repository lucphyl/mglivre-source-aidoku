#![no_std]

mod helper;
mod parser;

use aidoku::{
	error::Result,
	helpers::uri::encode_uri_component,
	prelude::*,
	std::{
		net::{HttpMethod, Request},
		String, Vec,
	},
	Chapter, DeepLink, Filter, FilterType, Listing, Manga, MangaPageResult, Page,
};

use helper::*;
use parser::*;

const BASE_URL: &str = "https://mangalivre.to";
const USER_AGENT: &str = "Aidoku";

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();

	for filter in filters {
		if let FilterType::Title = filter.kind {
			if let Ok(value) = filter.value.as_string() {
				query = value.read();
			}
		}
	}

	let url = if query.is_empty() {
		format!("{}/page/{}/?post_type=wp-manga", BASE_URL, page)
	} else {
		let encoded = encode_uri_component(query);
		format!(
			"{}/?s={}&post_type=wp-manga",
			BASE_URL,
			encoded
		)
	};

	let html = Request::new(url, HttpMethod::Get)
		.header("User-Agent", USER_AGENT)
		.html()?;

	Ok(parse_manga_list(html, true))
}

#[get_manga_listing]
fn get_manga_listing(_listing: Listing, page: i32) -> Result<MangaPageResult> {
	let url = format!("{}/page/{}/?m_orderby=latest", BASE_URL, page);

	let html = Request::new(url, HttpMethod::Get)
		.header("User-Agent", USER_AGENT)
		.html()?;

	Ok(parse_latest_manga_list(html))
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
	let url = get_manga_url(&manga_id);

	let html = Request::new(url.clone(), HttpMethod::Get)
		.header("User-Agent", USER_AGENT)
		.html()?;

	Ok(parse_manga_details(html, url))
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let url = get_manga_url(&manga_id);

	let html = Request::new(url, HttpMethod::Get)
		.header("User-Agent", USER_AGENT)
		.html()?;

	Ok(parse_chapter_list(html))
}

#[get_page_list]
fn get_page_list(_manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let chap_url = get_chapter_url(&chapter_id)?;

	let html = Request::new(chap_url, HttpMethod::Get)
		.header("User-Agent", USER_AGENT)
		.html()?;

	Ok(parse_page_list(html))
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	request
		.header("Referer", BASE_URL)
		.header("User-Agent", USER_AGENT);
}

#[handle_url]
fn handle_url(url: String) -> Result<DeepLink> {
	let manga_id = get_manga_id(&url);
	let chapter_id = get_chapter_id(&url);

	Ok(DeepLink {
		manga: get_manga_details(manga_id).ok(),
		chapter: Some(Chapter {
			id: chapter_id,
			..Default::default()
		}),
	})
}
