use reqwest::Client;

pub struct NetworkClient {
    client: Client,
}

impl NetworkClient {
    pub fn new() -> Self {
        let client = Client::new();
        NetworkClient { client }
    }

    pub async fn send_data(&self, data: &str) {
        let _ = self.client.post("http://example.com/api")
            .body(data.to_string())
            .send()
            .await;
    }
}