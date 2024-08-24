use crate::state::brush::Dot;
use crate::BRUSH;
use quad_net::http_request::{self, Request, RequestBuilder};
use quad_net::quad_socket::client;
use serde_json::to_string;

//TODO: Write out support for quad net, remove reqwest
//
async fn request(request: &mut Request) -> Option<String> {
    loop {
        if let Some(result) = request.try_recv() {
            return match result {
                Ok(data) => Some(data),
                Err(error) => {
                    eprintln!("Error reading inputdata: {}", error);
                    None
                }
            };
        }
    }
}



pub async fn get(lines: &mut Vec<Dot>) -> Vec<Dot> {
    unsafe {
        let url = format!(
            "http://{}/{}/{}",
            BRUSH.ip.clone(),
            BRUSH.room,
            BRUSH.apikey
        );

        let mut req = RequestBuilder::new(url.as_str()).send();
        match request(&mut req).await {
            Some(data) => match serde_json::from_str::<Vec<Dot>>(&data) {
                Ok(vec) => {
                    println!("RETRIEVED");
                    lines.extend(vec);
                }
                Err(e) => {
                    eprintln!("Failed to parse response: {:?}, error: {:?}", data, e);
                }
            },
            None => eprintln!("NONE"),
        }

        // match quad_net::http_request::Method::Get:(&url) {
        //     Ok(resp) => match resp.text() {
        //         Ok(text) => match serde_json::from_str::<Vec<Dot>>(&text) {
        //             Ok(vec) => {
        //                 println!("RETRIEVED");
        //                 lines.extend(vec);
        //             }
        //             Err(e) => {
        //                 eprintln!("Failed to parse response: {:?}, error: {:?}", text, e);
        //             }
        //         },
        //         Err(e) => eprintln!("Failed to get response text: {:?}", e),
        //     },
        //     Err(err) => eprintln!("Request failed: {:?}", err),
        // }

        lines.clone()
    }
}

pub async fn put(cache: &mut Vec<Dot>, ct: &mut i32) {
    unsafe {
        let url = format!(
            "http://{}/{}/{}",
            BRUSH.ip.clone(),
            BRUSH.room,
            BRUSH.apikey
        );
        let str = to_string(cache).unwrap();

        let _ = RequestBuilder::new(url.as_str()).body(&str).send();

        *ct = 0;
        cache.clear();
    }
}

pub fn delete() {
    // unsafe {
    //     let client = Client::new();
    //     let url = format!(
    //         "http://{}/delete/{}/{}",
    //         BRUSH.ip.clone(),
    //         BRUSH.room,
    //         BRUSH.apikey
    //     );

    //     match client.get(&url).send() {
    //         Ok(resp) => println!("Delete request response: {:?}", resp),
    //         Err(e) => eprintln!("Failed to send DELETE request: {:?}", e),
    //     }
    // }
}

//REFACTOR AND REMOVE REQWEST
