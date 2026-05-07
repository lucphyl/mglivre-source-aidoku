use aidoku::{error::Result, prelude::format, std::String};

use crate::BASE_URL;

/// Retorna o ID do mangá a partir da URL.
///
/// Exemplo:
/// https://mangalivre.to/manga/solo-leveling/
/// -> solo-leveling
pub fn get_manga_id(url: &str) -> String {
	let clean = url.trim_end_matches('/');

	match clean.split("/manga/").last() {
		Some(id) => String::from(id),
		None => String::from(""),
	}
}

/// Retorna o ID do capítulo.
///
/// Exemplo:
/// https://mangalivre.to/manga/solo-leveling/capitulo-1/
/// -> /manga/solo-leveling/capitulo-1/
pub fn get_chapter_id(url: &str) -> String {
	let clean = url.trim_end_matches('/');

	match clean.split(BASE_URL).last() {
		Some(path) => format!("{}/", path),
		None => String::from(""),
	}
}

/// Retorna URL completa do mangá.
pub fn get_manga_url(manga_id: &String) -> String {
	format!("{}/manga/{}/", BASE_URL, manga_id)
}

/// Retorna URL completa do capítulo.
pub fn get_chapter_url(chapter_id: &String) -> Result<String> {
	Ok(format!("{}{}", BASE_URL, chapter_id))
}
