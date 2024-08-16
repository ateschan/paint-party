use crate::state::brush::Dot;
use crate::BRUSH;
//Query timer (Push and pull)
pub async fn get(lines: &mut Vec<Dot>) -> Vec<Dot> {
    unsafe {
        let resp = match reqwest::blocking::get(
            "http://127.0.0.1:8000/".to_owned() + &BRUSH.room.to_string(),
        ) {
            Ok(resp) => resp.text().unwrap(),
            Err(err) => "Error: {}".to_owned() + &err.to_string(),
        };

        match serde_json::from_str::<Vec<Dot>>(&resp) {
            Ok(vec) => {
                println!("RETREIVED");
                for item in vec {
                    lines.push(item)
                }
            }
            Err(e) => {
                println!("{:?}{:?}", resp, e);
                return lines.clone();
            }
        }
        lines.clone()
    }
}

pub async fn put(cache: &mut Vec<Dot>, ct: &mut i32) {
    unsafe {
        let client = reqwest::blocking::Client::new();
        let _ = client
            .post("http://127.0.0.1:8000/".to_owned() + &BRUSH.room.to_string())
            .json(cache)
            .send();
    }
    *ct = 0;
    *cache = Vec::new();
}

pub fn delete() {
    unsafe {
        let client = reqwest::blocking::Client::new();
        let resp = client
            .get("http://127.0.0.1:8000/delete/".to_owned() + &BRUSH.room.to_string())
            .send();
        println!("{:?}", resp);
    }
}
