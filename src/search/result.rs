
pub type MediaType = String;


use once_cell::sync::Lazy;

pub static SERIES: Lazy<MediaType> = Lazy::new(|| String::from("series"));
pub static MOVIE: Lazy<MediaType> = Lazy::new(|| String::from("movie"));

pub struct MediaResult{
    pub title: String,
    pub year: String,
    pub media_type: MediaType,
    pub is_duplicate: bool,
    pub accuracy: i64,
}

impl MediaResult {
    pub fn new(title: String, year: String, media_type: MediaType, is_duplicate: bool, accuracy: i64) -> MediaResult {
      MediaResult {
            title,
            year,
            media_type,
            is_duplicate,
            accuracy,
        }
    }

    pub fn string(&self) -> String {
        format!("{} ({})", self.title, self.year)
    }
}