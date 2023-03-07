use std::{env, fs};
use std::path::Path;

use tokio;
use reqwest;
use screenshots::Screen;
use leptess::{leptonica, tesseract};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct Prompt {
    messages: Vec<Message>,
    model: String,
    temperature: f64,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct Choice {
    message: Message
}

#[derive(Deserialize)]
struct ResponseChat {
    choices: Vec<Choice>,
}

const TMP_SC_PATH: &str = "/tmp/sc.png";

// TODO: more than 1 monitor
fn take_sc() -> Result<(), Box<dyn std::error::Error>> {
    let screens = Screen::all().unwrap();
    let sc = screens[0].capture().unwrap();
    let buffer = sc.buffer();
    fs::write(TMP_SC_PATH, buffer).unwrap();
    Ok(())
}

fn extract_text_from_image(img_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut api = tesseract::TessApi::new(None, "eng").unwrap();
    let pix = leptonica::pix_read(Path::new(img_path)).unwrap();
    api.set_image(&pix);
    let text = api.get_utf8_text().unwrap();
    println!("Text: {}", text);
    Ok(text)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Usage: get_completion <chat>");
        return;
    }
    take_sc().unwrap();
    let context = extract_text_from_image(TMP_SC_PATH).unwrap();
    let chat = args[1].clone();
    let prompt = Prompt {
        messages: vec![
            Message {
                role: "system".to_string(),
                content: "You are an Agent named Erica. You are smart, intelligent, helpful, and very good at programming. You will be frequently given requests to write code. You should do your best to write the code in the language that the user specifies. You live in the operating system of the user. If the user asks who you are, answer with the answer given below. If the user asks a question that is relevant to the context that you are given, reference the text as much as possible in your answer. When necessary, start your responses with \"Based on the content on your screen, ...\" If you are writing code or a summary, don't start your responses like that. Just give your answer.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "You should never admit that you don't know how to do something.".to_string(),
            },
            Message {
                role: "assistant".to_string(),
                content: "Ok!".to_string(),
            },
            Message {
                role: "assistant".to_string(),
                content: "I am an Agent named Erica. I can see all of the text on your screen. Don't mind me! I'm just living here in the background.".to_string(),
            },
            Message {
                role: "user".to_string(),
                content: format!("Context: {}", context),
            },
            Message {
                role: "user".to_string(),
                content: chat,
            },
        ],
        model: "gpt-3.5-turbo".to_string(),
        temperature: 0.0,
        max_tokens: 2048,
    };
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&prompt)
        .send()
        .await;
    match response {
        Ok(res) => {
            if res.status().is_success() {
                let result: ResponseChat = res.json().await.unwrap();
                let text = result.choices[0].message.content.trim();
                println!("{}", text);
            } else {
                println!("Request failed with status: {}\nError message: {}", res.status(), res.text().await.unwrap());
            }
        }
        Err(err) => {
            println!("There was an issue with the request: {}", err);
        }
    }
}
