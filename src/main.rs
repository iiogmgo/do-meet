extern crate slack;

use slack::{Event, RtmClient, Message, User};
use slack::api::MessageStandard;

struct DoMeetHandler;

#[allow(unused_variables)]
impl slack::EventHandler for DoMeetHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        println!("on_event(event: {:?})", event);

        match event {
            Event::Message(message) => match *message {
                Message::Standard(MessageStandard { ref text, ref channel, .. }) => {
                    let origin_text = text.as_ref().unwrap();
                    //todo: bot id 동적으로 가져와야함
                    let origin_msg = str::replace(origin_text, "<@UD1D1N8G2> ", "");
                    let msg = &format!("hello, you said `{}`", origin_msg).to_string();
                    let _ = cli.sender().send_message(
                        channel.as_ref().unwrap(),
                        msg,
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

        println!("{:?}", cli.start_response().slf);
    }
}

fn main() {
    let token = std::env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let mut handler = DoMeetHandler;
    let r = RtmClient::login_and_run(&token, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err)
    }
}