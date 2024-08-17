use crate::state::brush::Dot;
use crate::BRUSH;
//Query timer (Push and pull)
pub async fn get(lines: &mut Vec<Dot>) -> Vec<Dot> {
    unsafe {
        let resp = match reqwest::blocking::get(
            format!("http://{}/{}/{}",  BRUSH.ip.clone(), &BRUSH.room.to_string(),BRUSH.apikey)) {
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
            .post(format!("http://{}/{}/{}",  BRUSH.ip.clone(), &BRUSH.room.to_string(),BRUSH.apikey))
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
            .get(format!("http://{}/delete/{}/{}",  BRUSH.ip.clone(), &BRUSH.room.to_string(),BRUSH.apikey))
            .send();
        println!("{:?}", resp);
    }
}
