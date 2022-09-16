use std::{path::Path, fs::File, io::Write};
use serde::{Serialize, Deserialize};
use diqwest::WithDigestAuth;

#[derive(Serialize, Deserialize, Debug)]
struct Speakers {
    speaker: Vec<Speaker>,
}

impl Speakers {
    fn new() -> Self {
        Self {
            speaker: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Speaker {
    ip: String,
    name: String,
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {

    //Check if settings.json file exists in local folder
    if !Path::new("settings.json").exists() {
        println!("settings.json file not found. Creating new file.");
        //prompt for settings
        match prompt_for_speaker_info().await {
            Ok(_) => println!("settings.json file created."),
            Err(e) => println!("Error getting speaker info: {}", e),
        }
    }

    let file = File::open("settings.json").unwrap();

    let speakers: Speakers = serde_json::from_reader(file).unwrap();

    loop {
        let mut speaker = String::new();
        println!("Select speaker:");
        for (i, s) in speakers.speaker.iter().enumerate() {
            println!("{}: {}", i, s.name);
        }

        std::io::stdin().read_line(&mut speaker).unwrap();
        let speaker = speaker.trim().parse::<usize>().unwrap();

        let selected_speaker = &speakers.speaker[speaker];

        println!("Enter File Name: ");
        let mut file_name = String::new();
        std::io::stdin().read_line(&mut file_name).unwrap();
        let file_name = file_name.trim();

        //ensure volume is between 0 and 100, but I think it can be set higher (maybe 200?)
        let mut volume = 0;
        while volume < 1 || volume > 100 {
            println!("Enter Volume (1-100): ");
            let mut volume_str = String::new();
            std::io::stdin().read_line(&mut volume_str).unwrap();
            volume = volume_str.trim().parse().unwrap();
        }

        let url = format!("http://{}/axis-cgi/playclip.cgi?location={}&repeat=0&volume={}&audiooutput=1", selected_speaker.ip, file_name, volume);

        let client = reqwest::Client::new();
        let res = client.get(&url).send_with_digest_auth(&selected_speaker.username, &selected_speaker.password).await.unwrap();
        println!("Response: {} | {}", res.status(), res.text().await.unwrap());
    }

}

async fn prompt_for_speaker_info() -> Result<(), Box<dyn std::error::Error>> {

    let mut file = File::create("settings.json").unwrap();
    let mut speakers = Speakers::new();

    let mut add_more = true;

    while add_more == true {
    
        println!("Enter Speaker IP: ");
        let mut speaker_ip = String::new();
        std::io::stdin().read_line(&mut speaker_ip).unwrap();
        let speaker_ip = speaker_ip.trim();

        println!("Enter Speaker Name: ");
        let mut speaker_name = String::new();
        std::io::stdin().read_line(&mut speaker_name).unwrap();
        let speaker_name = speaker_name.trim();

        println!("Enter Username: ");
        let mut username = String::new();
        std::io::stdin().read_line(&mut username).unwrap();
        let username = username.trim();

        println!("Enter Password: ");
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).unwrap();
        let password = password.trim();

        //test API nefore adding
        let url = format!("http://{}/axis-cgi/playclip.cgi?location=dsfsdf34&repeat=0&volume=1&audiooutput=1", speaker_ip);
        let client = reqwest::Client::new();
        let res = client.get(&url).send_with_digest_auth(&username, &password).await.unwrap();

        if res.status() == 200 {
            let speaker = Speaker {
                ip: speaker_ip.to_string(),
                name: speaker_name.to_string(),
                username: username.to_string(),
                password: password.to_string(),
            };
            speakers.speaker.push(speaker);
        }
        else {
            println!("Failed to connect to speaker. Please try again.");
            continue;
        }

        println!("Add another speaker? (y/n)");
        let mut add_more_str = String::new();
        std::io::stdin().read_line(&mut add_more_str).unwrap();
        let add_more_str = add_more_str.trim();

        //add more?
        if add_more_str == "y" {
            add_more = true;
        }
        else {
            add_more = false;
        }

        continue;
    }

    let json = serde_json::to_string_pretty(&speakers).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    

    Ok(())

}