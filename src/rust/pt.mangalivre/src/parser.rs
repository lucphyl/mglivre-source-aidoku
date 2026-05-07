use aidoku::{
	prelude::format,
	std::{html::Node, String, Vec},
	Chapter, Manga, MangaContentRating, MangaPageResult, MangaStatus, MangaViewer, Page,
};

use crate::helper::*;
use crate::BASE_URL;

pub fn parse_latest_manga_list(html: Node) -> MangaPageResult {
	let mut manga: Vec<Manga> = Vec::new();

	for node in html.select(".page-item-detail").array() {
		let node = node.as_node().expect("Failed to get node");

		let link_node = node.select("a").first();

		let raw_url = link_node.attr("href").read();

		if !raw_url.contains("/manga/") {
			continue;
		}

		let id = get_manga_id(&raw_url);
		let url = get_manga_url(&id);

		let title = node
			.select(".post-title")
			.first()
			.text()
			.read();

		let cover = node
			.select("img")
			.first()
			.attr("abs:src")
			.read();

		manga.push(Manga {
			id,
			cover,
			title,
			url,
			..Default::default()
		});
	}

	let has_more = !manga.is_empty();

	MangaPageResult { manga, has_more }
}

pub fn parse_manga_list(html: Node, _searching: bool) -> MangaPageResult {
	let mut manga: Vec<Manga> = Vec::new();

	for node in html.select(".c-tabs-item__content").array() {
		let node = node.as_node().expect("Failed to get node");

		let link_node = node.select(".post-title a").first();

		let raw_url = link_node.attr("href").read();

		if !raw_url.contains("/manga/") {
			continue;
		}

		let title = link_node.text().read();

		let id = get_manga_id(&raw_url);
		let url = get_manga_url(&id);

		let cover = node
			.select("img")
			.first()
			.attr("abs:src")
			.read();

		manga.push(Manga {
			id,
			cover,
			title,
			url,
			..Default::default()
		});
	}

	let has_more = !manga.is_empty();

	MangaPageResult { manga, has_more }
}

pub fn parse_manga_details(html: Node, manga_url: String) -> Manga {
	let id = get_manga_id(&manga_url);

	let title = html
		.select(".post-title h1")
		.first()
		.text()
		.read();

	let mut categories = Vec::new();

	for node in html.select(".genres-content a").array() {
		let node = node.as_node().expect("Failed to get genre node");
		let genre = node.text().read();
		categories.push(genre);
	}

	let description = html
		.select(".summary__content")
		.first()
		.text()
		.read();

	let mut author = String::from("");
	let mut artist = String::from("");
	let mut status = String::from("");

	for node in html.select(".post-content_item").array() {
		let node = node.as_node().expect("Failed to get info node");

		let label = node
			.select(".summary-heading")
			.first()
			.text()
			.read();

		let value = node
			.select(".summary-content")
			.first()
			.text()
			.read();

		match label.to_lowercase().trim() {
			"author(s)" => author = value,
			"artist(s)" => artist = value,
			"status" => status = value,
			_ => {}
		}
	}

	let status = match status.to_lowercase().trim() {
		"ongoing" => MangaStatus::Ongoing,
		"completed" => MangaStatus::Completed,
		"cancelled" => MangaStatus::Cancelled,
		"hiatus" => MangaStatus::Hiatus,
		_ => MangaStatus::Unknown,
	};

	let nsfw = {
		let mut rating = MangaContentRating::Safe;

		for genre in categories.iter() {
			match genre.to_lowercase().trim() {
				"ecchi" | "mature" => {
					rating = MangaContentRating::Suggestive
				}
				"hentai" | "smut" => {
					rating = MangaContentRating::Nsfw;
					break;
				}
				_ => {}
			}
		}

		rating
	};

	let cover = html
		.select(".summary_image img")
		.first()
		.attr("abs:src")
		.read();

	let viewer = MangaViewer::Scroll;

	Manga {
		id,
		title,
		categories,
		description,
		author,
		artist,
		status,
		nsfw,
		cover,
		viewer,
		url: manga_url,
	}
}

pub fn parse_chapter_list(html: Node) -> Vec<Chapter> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for node in html.select(".wp-manga-chapter").array() {
		let node = node.as_node().expect("Failed to get chapter node");

		let link_node = node.select("a").first();

		let raw_url = link_node.attr("href").read();

		let url = raw_url.clone();

		let id = get_chapter_id(&url);

		let title = link_node.text().read();

		let chapter = {
			let parts: Vec<&str> = title.split(" ").collect();
			let last = parts.last().unwrap_or(&"1");
			last.parse::<f32>().unwrap_or(-1.0)
		};

		let lang = String::from("pt-br");

		chapters.push(Chapter {
			id,
			title,
			lang,
			chapter,
			url,
			..Default::default()
		});
	}

	chapters
}

pub fn parse_page_list(html: Node) -> Vec<Page> {
	let mut pages = Vec::new();

	for (index, node) in html.select(".reading-content img").array().enumerate() {
		let node = node.as_node().expect("Failed to get image node");

		let url = node.attr("abs:src").read();

		if url.is_empty() {
			continue;
		}

		let index: i32 = index.try_into().unwrap();

		pages.push(Page {
			index,
			url,
			..Default::default()
		});
	}

	pages
}
