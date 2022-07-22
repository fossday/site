use crate::app::utils::file;
use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions, ComrakRenderOptions};
use lazy_static::lazy_static;
use rocket_dyn_templates::Template;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Header {
    title: String,
    description: String,
    date: String,
}

#[derive(Deserialize)]
pub struct HomeHeader {
    title: String,
    description: String,
    date: String,
    banner_section: Section,
    about_section: Section,
}

#[derive(Deserialize)]
pub struct Section {
    title: String,
    content: String,
}

lazy_static! {
    static ref OPTIONS: ComrakOptions = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,
            table: true,
            autolink: true,
            tasklist: true,
            description_lists: true,
            front_matter_delimiter: Some("+++".to_owned()),
            ..Default::default()
        },
        render: ComrakRenderOptions {
            unsafe_: true,
            ..Default::default()
        },
        ..Default::default()
    };
}

pub async fn get_home() -> Template {
    let file = "pages/home.md";
    let template = "index";
    let file_content = file::get_content(file).await.unwrap();

    let header: HomeHeader = toml::from_str(&file_content.header).unwrap();
    let content = markdown_to_html(&file_content.content, &OPTIONS);

    let contexts: HashMap<&str, &String> = [
        ("title", &header.title),
        ("description", &header.description),
        ("date", &header.date),
        ("banner_title", &header.banner_section.title),
        ("banner_description", &header.banner_section.content),
        ("about", &header.about_section.title),
        ("about_description", &header.about_section.content),
        ("content", &content),
    ]
    .iter()
    .cloned()
    .collect();

    Template::render(template, contexts)
}
