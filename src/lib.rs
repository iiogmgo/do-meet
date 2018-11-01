extern crate slack;
extern crate rand;

use rand::Rng;
use std::error::Error;
use slack::{Event, RtmClient, Message, User};
use slack::api::MessageStandard;

struct DoMeetHandler;

pub struct Config {
    pub token: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let token = args[1].clone();

        Ok(Config { token })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut handler = DoMeetHandler;
    let r = RtmClient::login_and_run(&config.token, &mut handler);
    match r {
        Ok(_) => { Ok(()) }
        Err(err) => panic!("Error: {}", err)
    }
}

fn get_bot_id(cli: &RtmClient) -> String {
    let cli_slf = match cli.start_response().slf.clone() {
        Some(c) => c,
        _ => panic!("Error")
    };

    match cli_slf.id {
        Some(c) => c,
        _ => panic!("Error")
    }
}

fn lunch() -> String {
    let vs = vec!["김치찌개", "설렁탕", "중식", "닭갈비", "오버런치나해"];
    let pick = rand::thread_rng().choose(&vs);

    match pick {
        Some(c) => format!("오늘은 `{}` 어때?", c).to_string(),
        _ => panic!("Error")
    }
}

fn msg_to_event(msg: &String) -> String {
    if msg.contains("lunch") || msg.contains("점심") {
        lunch()
    } else {
        String::from("미안, 아직 점심추천밖에 못해!")
    }
}

#[allow(unused_variables)]
impl slack::EventHandler for DoMeetHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);

        match event {
            Event::Message(message) => match *message {
                Message::Standard(MessageStandard { ref text, ref channel, .. }) => {
                    let bot_id = get_bot_id(cli);
                    let origin_text = text.as_ref().unwrap();
                    let origin_msg = str::replace(origin_text, &format!("<@{}> ", bot_id).to_string(), "");
                    let msg = msg_to_event(&origin_msg);

                    let _ = cli.sender().send_message(
                        channel.as_ref().unwrap(),
                        &msg,
                    );
                }
                _ => panic!("Message decoded into incorrect variant."),
            }
            _ => println!("other")
        }
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("on_close");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        let general_channel_id = cli.start_response()
            .channels
            .as_ref()
            .and_then(|channels| {
                channels
                    .iter()
                    .find(|chan| match chan.name {
                        None => false,
                        Some(ref name) => name == "general",
                    })
            })
            .and_then(|chan| chan.id.as_ref())
            .expect("general channel not found");
    }
}
