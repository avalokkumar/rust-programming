pub struct Song {
    pub title: String,
    pub artist: String,
    pub duration: u32,
}

impl Song {
    pub fn new(title: String, artist: String, duration: u32) -> Self {
        Song {
            title,
            artist,
            duration,
        }
    }
}