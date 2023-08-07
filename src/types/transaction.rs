use super::*;
use crate::DOMEN;
use chrono::{NaiveDate, NaiveTime};

use reqwest_wasm::blocking::Client;

impl Transaction {
    pub fn get_uuid(&self) -> String {
        self.id.clone()
    }

    pub fn sender_slot(&self) -> Slot {
        let client = Client::new();
        let request = client
            .get(DOMEN.to_string() + "/slots/{}" + self.get_uuid().as_str())
            .build()
            .unwrap();
        let responce = client.execute(request).unwrap();
        responce.json().unwrap()
    }

    pub fn sender(&self) -> User {
        self.sender_slot().owner()
    }

    pub fn recipient(&self) -> User {
        let client = Client::new();
        let request = client
            .get(DOMEN.to_string() + "/users/" + self.get_uuid().as_str())
            .build()
            .unwrap();
        let responce = client.execute(request).unwrap();
        responce.json().unwrap()
    }

    pub fn recipient_username(&self) -> String {
        self.recipient_username.clone()
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn passed(&self) -> bool {
        self.passed
    }

    pub fn time_of_rigestration(&self) -> NaiveTime {
        self.time_of_rigestration
    }

    pub fn date_of_rigestration(&self) -> NaiveDate {
        self.date_of_rigestration
    }

    pub fn time_of_ending(&self) -> Option<NaiveTime> {
        self.time_of_ending
    }

    pub fn date_of_ending(&self) -> Option<NaiveDate> {
        self.date_of_ending
    }

    pub fn finish(&mut self) {
        let client = reqwest::Client::new();
        let request = client
            .post(DOMEN.to_string() + "/transactions/" + self.get_uuid().as_str()+"/finish")
            .build()
            .unwrap();
        client.execute(request).unwrap();
    }
}
