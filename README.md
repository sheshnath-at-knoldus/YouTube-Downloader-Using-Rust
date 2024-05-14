# YouTube Video Downloader

This project is a simple YouTube video downloader written in Rust. It uses the `rusty_ytdl` crate to search for and download YouTube videos.

## Features

- Download videos in the highest quality.
- Simple command-line interface for user input.

## Prerequisites

Before running this project, ensure you have the following installed:

- Rust (https://www.rust-lang.org/tools/install)

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/rusty_ytdl.git
   cd rusty_ytdl

2. Install the required Rust crates by adding the following to your Cargo.toml:
    ```sh
    [dependencies]
    rusty_ytdl = "0.1"
    tokio = { version = "1", features = ["full"] }

3.Update your dependencies:

    cargo update


## Usage

1.Run the application:

    cargo run 

2. Enter the YouTube link when prompted.

    The video will be downloaded to the        current directory with the name test.mp4.


# Code Overview

The main components of the code are:

## Getting User Input

    ```rust
    fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }
    input.trim().to_string()
    }


## Video Downloader

This Rust project provides functionalities to download videos from YouTube.

## Video Options

    fn get_video_with_options() -> VideoOptions {
        VideoOptions {
            quality: VideoQuality::Highest,
            filter: VideoSearchOptions::VideoAudio,
            ..Default::default()
        }
    }



## Download Video Function

    
    async fn download_video(url: &String) -> bool {
        let path = std::path::Path::new("test.mp4");
        let youtube = YouTube::new().expect("failed to get youtube Instance");
        let video = Video::new_with_options(url, get_video_with_options()).expect("");
        video.download(path).await.is_ok()
    }



 ## Main Function


    #[tokio::main]
    async fn main() {
        let input = Rc::new(get_user_input("Enter your YouTube link:"));
        if download_video(&input).await {
            println!("Video downloaded");
        }
    }

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any changes or improvements.

## Acknowledgements

This project uses the `rusty_ytdl` crate for interacting with YouTube.


