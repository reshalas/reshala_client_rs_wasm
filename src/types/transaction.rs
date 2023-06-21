use super::*;
use crate::DOMEN;
use chrono::{NaiveDate, NaiveTime};

impl Transaction {
    pub fn get_uuid(&self) -> String {
        self.id.clone()
    }

    pub async fn get_sender_slot(&self) -> Slot {
        let client = reqwest::Client::new();
        let request = client
            .get(DOMEN.to_string() + "/slots/{}" + self.get_uuid().as_str())
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        responce.json().await.unwrap()
    }

    pub async fn get_sender(&self) -> User {
        self.get_sender_slot().await.get_owner().await
    }

    pub async fn get_recipient(&self) -> User {
        let client = reqwest::Client::new();
        let request = client
            .get(DOMEN.to_string() + "/users/" + self.get_uuid().as_str())
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        responce.json().await.unwrap()
    }

    pub fn sender_slot(&self) -> String {
        self.recipient.clone()
    }

    pub fn recipient_uuid(&self) -> String {
        self.recipient.clone()
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

    pub async fn finish(&mut self) {
        let client = reqwest::Client::new();
        let request = client
            .post(DOMEN.to_string() + "/transactions/" + self.get_uuid().as_str()+"/finish")
            .build()
            .unwrap();
        client.execute(request).await.unwrap();
    }
}
