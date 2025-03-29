use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct CosmeticItem {
    name: String,
    item_type: String,
}

#[derive(Serialize, Deserialize)]
struct UnlockRequest {
    items: Vec<CosmeticItem>,
}

#[derive(Serialize, Deserialize)]
struct UnlockResponse {
    success: bool,
    message: String,
}

struct FortniteHybrid {
    client: Client,
}

impl FortniteHybrid {
    fn new() -> Self {
        let client = Client::new();
        FortniteHybrid { client }
    }

    async fn unlock_items(&self, items: Vec<CosmeticItem>) -> Result<UnlockResponse, Box<dyn Error>> {
        let request = UnlockRequest { items };
        let response = self.client.post("https://api.fortnite.com/unlock")
            .json(&request)
            .send()
            .await?
            .json::<UnlockResponse>()
            .await?;
        Ok(response)
    }

    fn display_menu(&self) {
        println!("Welcome to Fortnite Hybrid");
        println!("Select items to unlock:");
        println!("1. Outfits");
        println!("2. Emotes");
        println!("3. Pickaxes");
        println!("4. Backblings");
        println!("5. Gliders");
        println!("6. Kicks");
        println!("7. Wraps");
        println!("8. Loadings");
        println!("9. Music");
        println!("10. Contrails");
        println!("11. Sprays");
        println!("12. Emojis");
        println!("13. Banners");
        println!("14. Bundles");
        println!("Enter your choices separated by commas:");
    }

    fn get_user_input(&self) -> Vec<CosmeticItem> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choices: Vec<&str> = input.trim().split(',').collect();
        let mut items = Vec::new();
        for choice in choices {
            let item = CosmeticItem {
                name: choice.trim().to_string(),
                item_type: "Cosmetic".to_string(),
            };
            items.push(item);
        }
        items
    }

    async fn run(&self) {
        self.display_menu();
        let items = self.get_user_input();
        match self.unlock_items(items).await {
            Ok(response) => {
                if response.success {
                    println!("Items unlocked successfully: {}", response.message);
                } else {
                    println!("Failed to unlock items: {}", response.message);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let fortnite_hybrid = FortniteHybrid::new();
    fortnite_hybrid.run().await;
}