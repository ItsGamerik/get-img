use std::net::TcpStream;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serenity::model::prelude::Message;

// struct for making api requests to https://github.com/oobabooga/text-generation-webui
// there seems to be no real documentation
// also: idea stolen from desinc: https://github.com/DeSinc/SallyBot
#[derive(Serialize, Deserialize, Debug)]
struct Request {
    prompt: String,
    max_new_tokens: i32,
    do_sample: bool,
    temperature: f32,
    top_p: f32,
    typical_p: i32,
    repetition_penalty: f32,
    encoder_repetition_penalty: i32,
    top_k: i32,
    num_beams: i32,
    penalty_alpha: i32,
    min_length: i32,
    length_penalty: i32,
    no_repeat_ngram_size: i32,
    seed: i32,
    add_bos_token: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct InnerObject {
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OuterObject {
    results: Vec<InnerObject>,
}
pub async fn message_responder(msg: &Message) -> String {
    if check_port_online().await == false {
        let error = "webserver port is not reachable!".to_string();
        println!("{}", error);
        return error;
        
    }
    // facebook_opt-1.3b it is XD
    let messge_content = msg.content.to_string();
    let re = Regex::new("<.*>").unwrap();
    let cleaned = re.replace_all(&messge_content, "").to_string();
    println!("new prompt detected: {}", &cleaned);
    let request1 = Request {
        prompt: cleaned,
        max_new_tokens: 100,
        do_sample: false,
        temperature: 0.99,
        top_p: 0.9,
        typical_p: 1,
        repetition_penalty: 1.1,
        encoder_repetition_penalty: 1,
        top_k: 40,
        num_beams: 1,
        penalty_alpha: 0,
        min_length: 0,
        length_penalty: 1,
        no_repeat_ngram_size: 1,
        seed: -1, // 1106436159
        add_bos_token: true,
    };
    let request_body = serde_json::to_string(&request1).unwrap();
    let response_data = request(request_body).await;
    let outer_object: OuterObject = match serde_json::from_str(&response_data) {
        Ok(text) => text,
        Err(e) => panic!("another error occured: {}", e),
    };
    println!("{:?}", outer_object);
    let text = outer_object.results[0].text.clone();
    text
}

async fn check_port_online() -> bool {
    match TcpStream::connect(("localhost", 5000)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

async fn request(body: String) -> String {
    let client = reqwest::Client::new();
    let url = "http://127.0.0.1:5000/api/v1/generate"; // url for the text generation model api
    let response = match client.post(url).body(body).send().await {
        Ok(response) => response,
        Err(e) => panic!("an error occured: {}", e),
    };
    // dbg!(&response);
    let text = response.text().await.unwrap();
    text
}
