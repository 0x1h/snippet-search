use api::api_types::Daum;
use api::fetch_songs::fetch_songs;
use constant::lines::lines;
use constant::name::application_name;
use std::io::{self, Write};
use utils::clear_screen::clear_screen;

mod api {
    pub mod api_types;
    pub mod fetch_songs;
}

mod constant {
    pub mod lines;
    pub mod name;
}

mod utils {
    pub mod clear_screen;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut _songs: Vec<Daum> = vec![];

    loop {
        clear_screen();
        println!("{}", application_name());

        let stdin = io::stdin();

        if _songs.len() > 0 {
            println!("Songs Found in Total: {}", _songs.len());
        }

        let songs_clone = _songs.clone();

        lines(songs_clone);
        for i in 0.._songs.len() {
            println!("{} - {}", _songs[i].artist.name, _songs[i].title)
        }
        lines(_songs);

        print!("Search Song: ");
        io::stdout().flush().unwrap();

        let mut search = String::new();
        stdin.read_line(&mut search).expect("Counld't Read Input");

        _songs = fetch_songs(search.trim().to_string()).await?;
    }
}
