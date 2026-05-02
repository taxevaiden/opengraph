use crate::Audio;
use crate::Image;
use crate::Object;
use crate::Video;
use crate::error::Error;
use std::io::Read;

use scraper::{Html, Selector};

#[derive(Default)]
pub struct Opts {
    pub include_images: bool,
    pub include_audios: bool,
    pub include_videos: bool,
}

pub fn extract<R>(input: &mut R, option: Opts) -> Result<Object, Error>
where
    R: Read,
{
    let mut html = String::new();
    input.read_to_string(&mut html).unwrap();
    let doc = Html::parse_document(&html);

    let mut og_props = Vec::new();
    let mut images = Vec::new();
    let mut audios = Vec::new();
    let mut videos = Vec::new();

    let meta_sel = Selector::parse("meta").unwrap();
    for el in doc.select(&meta_sel) {
        let mut props = extract_open_graph_from_meta_tag(el);
        og_props.append(&mut props);
    }

    if option.include_images {
        let img_sel = Selector::parse("img").unwrap();
        for el in doc.select(&img_sel) {
            if let Some(image) = extract_image(el) {
                images.push(image);
            }
        }
    }

    if option.include_audios {
        let audio_sel = Selector::parse("audio").unwrap();
        for el in doc.select(&audio_sel) {
            if let Some(audio) = extract_audio(el) {
                audios.push(audio);
            }
        }
    }

    if option.include_videos {
        let video_sel = Selector::parse("video").unwrap();
        for el in doc.select(&video_sel) {
            if let Some(video) = extract_video(el) {
                videos.push(video);
            }
        }
    }

    let mut obj = Object::new(&og_props);
    obj.images.append(&mut images);
    obj.audios.append(&mut audios);
    obj.videos.append(&mut videos);
    Ok(obj)
}

pub fn extract_open_graph_from_meta_tag(el: scraper::ElementRef) -> Vec<(String, String)> {
    let mut og_props = vec![];
    if let Some(pair) = extract_open_graph_prop("property", el) {
        og_props.push(pair);
    }
    if let Some(pair) = extract_open_graph_prop("name", el) {
        og_props.push(pair);
    }
    og_props
}

fn extract_open_graph_prop(attr_name: &str, el: scraper::ElementRef) -> Option<(String, String)> {
    el.value().attr(attr_name).and_then(|property| {
        if let Some(stripped) = property.strip_prefix("og:") {
            let key = stripped.to_string();
            el.value()
                .attr("content")
                .map(|content| (key, content.to_string()))
        } else {
            None
        }
    })
}

pub fn extract_image(el: scraper::ElementRef) -> Option<Image> {
    el.value()
        .attr("src")
        .map(|src| Image::new(src.to_string()))
}

pub fn extract_audio(el: scraper::ElementRef) -> Option<Audio> {
    el.value()
        .attr("src")
        .map(|src| Audio::new(src.to_string()))
}

pub fn extract_video(el: scraper::ElementRef) -> Option<Video> {
    el.value()
        .attr("src")
        .map(|src| Video::new(src.to_string()))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::object::ObjectType;
    #[test]
    fn extract_open_graph_object() {
        let x = r#"
<html prefix="og: http://ogp.me/ns#">
<head>
<title>The Rock (1996)</title>
<meta property="og:title" content="The Rock" />
<meta property="og:type" content="video.movie" />
<meta property="og:url" content="http://www.imdb.com/title/tt0117500/" />
<meta property="og:image" content="http://ia.media-imdb.com/images/rock.jpg" />
</head>
</html>
                "#;
        let obj = extract(&mut x.to_string().as_bytes(), Default::default());
        assert!(obj.is_ok());
        let obj = obj.unwrap();
        assert_eq!(&obj.title, "The Rock");
        assert_eq!(obj.obj_type, ObjectType::Movie);
        assert_eq!(&obj.url, "http://www.imdb.com/title/tt0117500/");
        assert_eq!(obj.images.len(), 1);
        assert_eq!(
            &obj.images[0].url,
            "http://ia.media-imdb.com/images/rock.jpg"
        );
    }
}
