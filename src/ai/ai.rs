use serde::{Serialize, Deserialize};
use serenity::futures::future::UnwrapOrElse;

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
    // early_stopping: bool,
    seed: i32,
    add_bos_token: bool,
}

pub async fn message_responder() {

    let request1 =  Request {
        prompt: "this is a string".to_string(),
        max_new_tokens: 200,
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
        seed: -1,
        add_bos_token: true
    };
    let request_body = serde_json::to_string(&request1).unwrap();
    dbg!("dings: {}", &request_body);
    request(request_body).await.unwrap();
}

async fn request(body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = "http://127.0.0.1:5000/api/v1/generate"; // url for the text generation model

    let response = client.post(url).body(body).send().await?;
    dbg!(&response);
    let text = response.text().await?;
    dbg!(&text);
    Ok(text)
}