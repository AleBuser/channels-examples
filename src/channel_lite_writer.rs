#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use iota_lib_rs::prelude::iota_client;
use iota_streams::{
    app_channels::api::tangle::{ Address,Author}
};
use iota_streams::app::transport::Transport;
use iota_streams::app::transport::tangle::client::SendTrytesOptions;
use iota_streams::protobuf3::types::Trytes;
use iota_streams::core::tbits::Tbits;
use std::string::ToString;
use std::str::FromStr;
use failure::Fallible;


pub struct Channel{
    author: Author,
    client: iota_client::Client<'static>,
    send_options: SendTrytesOptions,
    channel_address:String,
}

impl Channel {

    pub fn new(seed: &str, node_ulr: &'static str) -> Channel{

        let author = Author::new(seed, 3, true);

        let channel_addresss = author.channel_address().to_string();
        let mut send_opt = SendTrytesOptions::default();

        send_opt.min_weight_magnitude = 9;
        send_opt.local_pow = false;

        Self {
            author:author,
            client: iota_client::Client::new(node_ulr),
            send_options: send_opt,
            channel_address:channel_addresss,
        }
    }


    pub fn open(&mut self)-> Result<String, &str>{
       
        let announcement = self.author.announce().unwrap();
        self.client.send_message_with_options(&announcement,self.send_options ).unwrap();
        println!("Announced a new channel");
        Ok(announcement.link.msgid.to_string())
    }

    pub fn write(&mut self,announce_message_identifier: &str, public_payload: &str, private_payload: &str)-> Result<String, &str>{

        let public_payload = Trytes(Tbits::from_str(&public_payload).unwrap());
        let private_payload = Trytes(Tbits::from_str(&private_payload).unwrap());

        let announcement_link = Address::from_str(&self.channel_address, &announce_message_identifier).unwrap();

        let message = self.author.sign_packet(&announcement_link, &public_payload, &private_payload).unwrap();

        // Convert the message to a bundle and send it to a node
        self.client.send_message_with_options(&message,self.send_options).unwrap();
        println!("Sent signed message");
        Ok(message.link.msgid.to_string())

    }

    pub fn remove_subscriber() -> Fallible<()>{
        //TODO
        Ok(())
    }

    pub fn get_channel_address(&mut self) -> String{
        self.channel_address.clone()
    }
      
}