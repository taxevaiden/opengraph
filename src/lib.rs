#[macro_use]
extern crate serde_derive;
extern crate html5ever;
extern crate serde;
extern crate serde_json;
extern crate url;

mod audio;
mod image;
mod object;
mod video;

pub mod error;

pub use audio::Audio;
pub use error::Error;
pub use image::Image;
pub use object::Object;
pub use url::Url;
pub use video::Video;
