use crate::song::Song;

pub struct Player {
    playlist: Vec<Song>,
    current_index: usize,
}

impl Player {
    pub fn new() -> Self {
        Player {
            playlist: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_song(&mut self, song: Song) {
        self.playlist.push(song);
    }

    pub fn play(&self) {
        if let Some(song) = self.playlist.get(self.current_index) {
            println!("Playing: {} by {}", song.title, song.artist);
        } else {
            println!("No songs in the playlist.");
        }
    }

    pub fn pause(&self) {
        println!("Paused.");
    }

    pub fn stop(&mut self) {
        self.current_index = 0;
        println!("Stopped.");
    }
}