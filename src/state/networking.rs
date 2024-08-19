use crate::state::brush::Dot;
use crate::BRUSH;
use reqwest::blocking::{Client, Response};

pub async fn get(lines: &mut Vec<Dot>) -> Vec<Dot> {
    unsafe {
        let url = format!(
            "http://{}/{}/{}",
            BRUSH.ip.clone(),
            BRUSH.room.to_string(),
            BRUSH.apikey
        );

        match reqwest::blocking::get(&url) {
            Ok(resp) => match resp.text() {
                Ok(text) => match serde_json::from_str::<Vec<Dot>>(&text) {
                    Ok(vec) => {
                        println!("RETRIEVED");
                        lines.extend(vec);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse response: {:?}, error: {:?}", text, e);
                    }
                },
                Err(e) => eprintln!("Failed to get response text: {:?}", e),
            },
            Err(err) => eprintln!("Request failed: {:?}", err),
        }

        lines.clone()
    }
}

pub async fn put(cache: &mut Vec<Dot>, ct: &mut i32) {
    unsafe {
        let client = Client::new();
        let url = format!(
            "http://{}/{}/{}",
            BRUSH.ip.clone(),
            BRUSH.room.to_string(),
            BRUSH.apikey
        );

        if let Err(e) = client.post(&url).json(cache).send() {
            eprintln!("Failed to send PUT request: {:?}", e);
        }

        *ct = 0;
        cache.clear();
    }
}

pub fn delete() {
    unsafe {
        let client = Client::new();
        let url = format!(
            "http://{}/delete/{}/{}",
            BRUSH.ip.clone(),
            BRUSH.room.to_string(),
            BRUSH.apikey
        );

        match client.get(&url).send() {
            Ok(resp) => println!("Delete request response: {:?}", resp),
            Err(e) => eprintln!("Failed to send DELETE request: {:?}", e),
        }
    }
}
