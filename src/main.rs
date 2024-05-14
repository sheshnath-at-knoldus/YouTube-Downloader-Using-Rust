use rusty_ytdl::search::YouTube;
use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use std::io;
use std::rc::Rc;

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input) => {}
        Err(_no_update_is_fine) => {}
    }
    input.trim().to_string()
}

fn get_video_with_options() -> VideoOptions {
    let options = VideoOptions {
        quality: VideoQuality::Highest,
        filter: VideoSearchOptions::VideoAudio,
        ..Default::default()
    };
    options
}
async fn download_video(url: &String) -> bool {
    let path = std::path::Path::new("test.mp4");
    let youtube = YouTube::new().expect("failed to get youtube Instance");
    let video = Video::new_with_options(url,get_video_with_options()).expect("");
    video.download(path).await.is_ok()
}

#[tokio::main]
async fn main() {
    let input = Rc::new(get_user_input("Enter your youtube link"));
    if download_video(&input).await {
        println!("video downloaded");
    }
}
