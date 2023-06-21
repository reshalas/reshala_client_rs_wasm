use super::{LocationData, SlotsComponent, Subjects, Task, TelegramData, Transaction, User};
use crate::{
    dto::{EmailDTO, PhoneDTO, SlotDTO, TaskDTO, UserDTO},
    DOMEN,
};
use reqwest::StatusCode;

impl User {
    pub async fn register(user: UserDTO) -> Result<User, String> {
        let client = reqwest::Client::new();
        let request = client
            .post(DOMEN.to_string() + "/users/register")
            .json(&user)
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        if responce.status() == StatusCode::OK {
            return Ok(
                serde_json::from_str::<User>(responce.text().await.unwrap().as_str()).unwrap(),
            );
        }
        Err(responce.text().await.unwrap())
    }

    async fn refresh(&mut self) {
        let new_data = User::get_by_telegram_id(self.telegram_id()).await.unwrap();
        *self = new_data;
    }

    pub async fn get_by_telegram_id(id: u64) -> Option<User> {
        let client = reqwest::Client::new();
        let request = client
            .get(DOMEN.to_string() + "/users/from_telegramm/" + id.to_string().as_str())
            .build()
            .unwrap();
        let responce = client.execute(request).await.unwrap();
        if responce.status() == StatusCode::OK {
            return Some(responce.json().await.unwrap());
        }
        None
    }

    pub async fn add_slot(&mut self, dto: SlotDTO) {
        let url = format!(
            "{}/users/{}/slots/activate",
            DOMEN.to_string(),
            self.get_uuid()
        );
        let client = reqwest::Client::new();
        let request = client.post(url).json(&dto).build().unwrap();
        client.execute(request).await.unwrap();
        self.refresh().await;
    }

    pub async fn remove_slot(&mut self, subject: Subjects) -> Result<(), ()> {
        let url = format!(
            "{}/users/{}/slots/deactivate/{:?}",
            DOMEN.to_string(),
            self.get_uuid(),
            subject
        );
        let client = reqwest::Client::new();
        let request = client.delete(url).build().unwrap();
        let result = client.execute(request).await.unwrap();
        self.refresh().await;
        match result.status() {
            StatusCode::OK => Ok(()),
            _ => Err(()),
        }
    }

    pub fn location_info(&self) -> LocationData {
        self.location_data.clone()
    }

    pub fn telegram_info(&self) -> TelegramData {
        self.telegram_data.clone()
    }

    pub fn get_uuid(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn class(&self) -> i16 {
        self.class
    }

    pub fn telegram_id(&self) -> u64 {
        self.telegram_data.telegram_id
    }

    pub fn telegram_chat_id(&self) -> i64 {
        self.telegram_data.chat_id
    }

    pub fn school(&self) -> i32 {
        self.school
    }

    pub fn email(&self) -> Option<String> {
        self.email.clone()
    }

    pub fn phone_number(&self) -> Option<String> {
        self.phone_number.clone()
    }

    pub fn slots_component(&mut self) -> &mut SlotsComponent {
        &mut self.slots_component
    }

    //Сеттеры
    pub async fn change_email(&mut self, email: Option<String>) {
        let url = format!(
            "{}/users/{}/change/email",
            DOMEN.to_string(),
            self.get_uuid()
        );
        let client = reqwest::Client::new();
        let request = client.post(url).json(&EmailDTO { email }).build().unwrap();
        client.execute(request).await.unwrap();
        self.refresh().await;
    }

    pub async fn change_phone_number(&mut self, phone: Option<String>) {
        let url = format!(
            "{}/users/{}/change/phone",
            DOMEN.to_string(),
            self.get_uuid()
        );
        let client = reqwest::Client::new();
        let request = client.post(url).json(&PhoneDTO { phone }).build().unwrap();
        client.execute(request).await.unwrap();
        self.refresh().await;
    }

    //Работа с тасками
    pub async fn publish_task(&self, dto: TaskDTO) -> Task {
        let url = format!(
            "{}/users/{}/publish_task",
            DOMEN.to_string(),
            self.get_uuid()
        );
        let client = reqwest::Client::new();
        let request = client.post(url).json(&dto).build().unwrap();
        let responce = client.execute(request).await.unwrap();
        responce.json().await.unwrap()
    }

    pub async fn accept_task(&mut self, task: Task) -> Result<Transaction, String> {
        let url = format!(
            "{}/users/{}/accept_task/{}",
            DOMEN.to_string(),
            self.get_uuid(),
            task.get_uuid()
        );
        let client = reqwest::Client::new();
        let request = client.post(url).build().unwrap();
        let responce = client.execute(request).await.unwrap();
        let result = responce.json().await.unwrap();
        self.refresh().await;
        result
    }

    pub async fn get_my_tasks(&self) -> Vec<Task> {
        let url = format!("{}/users/{}/tasks", DOMEN.to_string(), self.get_uuid());
        let client = reqwest::Client::new();
        let request = client.get(url).build().unwrap();
        let responce = client.execute(request).await.unwrap();
        responce.json().await.unwrap()
    }

    //рейтинг
    pub async fn rate(&mut self, mark: u8) {
        if mark > 5 {
            panic!("Wrong mark")
        }
        let url = format!(
            "{}/users/{}/rate/{}",
            DOMEN.to_string(),
            self.get_uuid(),
            mark
        );
        let client = reqwest::Client::new();
        let request = client.post(url).build().unwrap();
        client.execute(request).await.unwrap();
        self.refresh().await;
    }

    pub fn raiting(&self) -> f64 {
        if self.raiting.len() == 0 {
            return 0.0;
        }
        let mut sum: f64 = 0.0;
        for mark in self.raiting.clone() {
            sum += mark as f64;
        }
        sum / self.raiting.len() as f64
    }
}
