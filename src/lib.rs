#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate url;

mod audio;
mod image;
mod object;
mod video;

pub mod error;
pub mod scraper;

pub use audio::Audio;
pub use error::Error;
pub use image::Image;
pub use object::Object;
pub use scraper::extract;
pub use scraper::{extract_audio, extract_image, extract_open_graph_from_meta_tag, extract_video};
pub use url::Url;
pub use video::Video;
