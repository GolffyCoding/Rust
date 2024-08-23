use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use hex;
use rand::Rng;
use sha2::{Digest, Sha256};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use chrono::Local;
extern crate lazy_static;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::error::Error;



lazy_static! {
    static ref GLOBAL_FunctionName: Mutex<String> = Mutex::new("".to_string());
}



static _URL: &str = "https://script.google.com/macros/s/AKfycbxsIV5y6IaQAwJ4MnBfD4GiIyNZtQYXV-sem7pB1gFT-yl4vFF5BBXqApFfMSh6VMuL1w/exec";
static URL_TRANSACTION:&str ="https://script.google.com/macros/s/AKfycbz9HMVfWR2b1wo5VH6jkTfgQt1_cCjObAVmL0HMUd-WkiBNcFqqVj-avzTnLpUdUsQ/exec";


#[derive(Debug, Clone)]
pub struct Wallet {
    pub address: String,
    pub balance: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSignature {
    sender: String,
    signature: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeDetails {
    address: String,
    type_: String,
    balance_change_amount: f64,
    currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BalanceChange {
    changes: Vec<ChangeDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionData {
    type_of_func: String,
    digest: String,
    age: String,
    sender: String,
    recipients: String,
    amount_of_transactions: f64,
    user_signature: UserSignature,
    balance_change: BalanceChange,
   
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct TransactionBlock {
    inputs: Vec<String>,
}




impl TransactionBlock { 

    pub fn empty() -> Self {
        TransactionBlock {
            inputs: Vec::new(),
        }
    }

    // Method to initialize TransactionBlock with a Vec<String>
    pub fn new(inputs: Vec<String>) -> Self {
        TransactionBlock {
            inputs,
        }
    }
    
    pub async fn get_coin_metadata(&self, coin_symbol: &str) -> Option<Coin> {
        let client = Client::new();
        let url = format!("{}?action=getCoin&symbol={}",_URL, coin_symbol);

        // Await the Future and handle the result
        let response = client.get(&url).send().await.ok()?;
        let response_text = response.text().await.ok()?;
        let coin: Coin = serde_json::from_str(&response_text).ok()?;
        Some(coin)
    }

    pub async fn get_total_supply(&self, coin_symbol: &str) -> Option<f64> {
        let client = Client::new();
        let url = format!("{}?action=getCoin&symbol={}",_URL, coin_symbol);

        // Await the Future and handle the result
        let response = client.get(&url).send().await.ok()?;
        let response_text = response.text().await.ok()?;
        let coin: Coin = serde_json::from_str(&response_text).ok()?;
        Some(coin);
        self.get_coin_metadata(coin_symbol).await.map(|coin| coin.total_supply)
    }

    pub async fn get_coin_address(&self, coin_symbol: &str) -> Option<String> {
        let client = Client::new();
        let url = format!("{}?action=getCoin&symbol={}",_URL, coin_symbol);

        // Await the Future and handle the result
        let response = client.get(&url).send().await.ok()?;
        let response_text = response.text().await.ok()?;
        let coin: Coin = serde_json::from_str(&response_text).ok()?;
        Some(coin);
        self.get_coin_metadata(coin_symbol).await.map(|coin| coin.address)
    }

    pub async fn get_coin_packageid(&self, coin_symbol: &str) -> Option<String> {
        let client = Client::new();
        let url = format!("{}?action=getCoin&symbol={}",_URL, coin_symbol);

        // Await the Future and handle the result
        let response = client.get(&url).send().await.ok()?;
        let response_text = response.text().await.ok()?;
        let coin: Coin = serde_json::from_str(&response_text).ok()?;
        Some(coin);
        self.get_coin_metadata(coin_symbol).await.map(|coin| coin.package_id)
    }

    pub async fn get_coin_creator(&self, coin_symbol: &str) -> Option<String> {
        let client = Client::new();
        let url = format!("{}?action=getCoin&symbol={}",_URL, coin_symbol);

        // Await the Future and handle the result
        let response = client.get(&url).send().await.ok()?;
        let response_text = response.text().await.ok()?;
        let coin: Coin = serde_json::from_str(&response_text).ok()?;
        Some(coin);
        self.get_coin_metadata(coin_symbol).await.map(|coin| coin.creator)
    }
    
    pub async fn get_coin_description(&self, coin_symbol: &str) -> Option<String> {
        let client = Client::new();
        let url = format!("{}?action=getCoin&symbol={}",_URL, coin_symbol);

        // Await the Future and handle the result
        let response = client.get(&url).send().await.ok()?;
        let response_text = response.text().await.ok()?;
        let coin: Coin = serde_json::from_str(&response_text).ok()?;
        Some(coin);
        self.get_coin_metadata(coin_symbol).await.map(|coin| coin.description)
    }

    pub async fn get_coin_objectid(&self, coin_symbol: &str) -> Option<String> {
        let client = Client::new();
        let url = format!("{}?action=getCoin&symbol={}",_URL, coin_symbol);

        // Await the Future and handle the result
        let response = client.get(&url).send().await.ok()?;
        let response_text = response.text().await.ok()?;
        let coin: Coin = serde_json::from_str(&response_text).ok()?;
        Some(coin);
        self.get_coin_metadata(coin_symbol).await.map(|coin| coin.object_id)
    }

    pub async fn get_coin_image(&self, coin_symbol: &str) -> Option<String> {
        let client = Client::new();
        let url = format!("{}?action=getCoin&symbol={}",_URL, coin_symbol);

        // Await the Future and handle the result
        let response = client.get(&url).send().await.ok()?;
        let response_text = response.text().await.ok()?;
        let coin: Coin = serde_json::from_str(&response_text).ok()?;
        Some(coin);
        self.get_coin_metadata(coin_symbol).await.map(|coin| coin.image)
    }

    pub async fn get_allcoins(&self) -> Option<String> {
        let client = reqwest::Client::new();
        let url = format!("{}?action=getAllCoins", _URL);
    
        let response = client.get(&url).send().await;
        let response_text = match response {
            Ok(resp) => {
                println!("Response Status: {}", resp.status());
                resp.text().await.ok()?
            },
            Err(e) => {
                println!("Request Error: {}", e);
                return None;
            }
        };

        Some(response_text)
    }

    pub async fn get_allcoins_data(&self) -> Option<String> {
        let client = Client::new();
        let url = format!("{}?action=getAllCoinData",_URL);

        let response = client.get(&url).send().await;
        let response_text = match response {
            Ok(resp) => {
                println!("Response Status: {}", resp.status());
                resp.text().await.ok()?
            },
            Err(e) => {
                println!("Request Error: {}", e);
                return None;
            }
        }; 
        Some(response_text)
    }

    pub async fn create_transaction(&self,
        sender: &str,
        recipients: &str,
        amount_of_transactions: f64,
        currency: Option<&str>) -> Result<(), reqwest::Error> {
        {
            let mut global_var = GLOBAL_FunctionName.lock().unwrap();
            *global_var = "create_transaction".to_string();
        }
         
        let url = format!("{}", URL_TRANSACTION);
        let client = Client::new();
        let now = Local::now();
        let _formatted_time = now.format("%-d/%-m/%Y, %-H:%M:%S").to_string();
        let currency = currency.unwrap_or("USD");
        
    let transaction = TransactionData {
        type_of_func: GLOBAL_FunctionName.lock().unwrap().to_string(),
        digest: generate_random_string(64).to_string(),
        age: _formatted_time.to_string(),
        sender: sender.to_string(),
        recipients: recipients.to_string(),
        amount_of_transactions: amount_of_transactions,
        user_signature: UserSignature {
            sender: sender.to_string(),
            signature: format!("{}::{}",generate_package_id(128),generate_random_string(128)),
        },
        balance_change: BalanceChange {
            changes: vec![
                ChangeDetails {
                    address: sender.to_string(),
                    type_: "Account".to_string(),
                    balance_change_amount: -amount_of_transactions,
                    currency: currency.to_string(),
                },
                ChangeDetails {
                    address: recipients.to_string(),
                    type_: "Account".to_string(),
                    balance_change_amount: amount_of_transactions,
                    currency: currency.to_string(),
                },
            ],
        },
        
            
        
    };
    println!("{:?}",&transaction);

    client.post(&url)
            .json(&transaction)
            .send()
            .await?
            .error_for_status()?;
    Ok(())
    }

    pub async fn  get_lasttransaction_data() -> Result<Vec<TransactionData>, Box<dyn Error>>{
        let url = URL_TRANSACTION; // Replace with your actual URL
        let client = Client::new();
    
        let response = client.get(url).send().await?.json::<Vec<TransactionData>>().await?;
         
        Ok(response)
       
    }

    pub async fn get_transaction_by_digest(digest: &str) -> Result<Option<TransactionData>, Box<dyn Error>> {
        let transactions = TransactionBlock::get_lasttransaction_data().await?;
        
        let transaction = transactions.iter().find(|&tx| tx.digest == digest).cloned();
    
        Ok(transaction)
    }

    

   


}


#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDataOptions {
    pub include_metadata: bool,
    pub include_history: bool,
    pub additional_fields: Option<Vec<String>>, // Adjust type based on your needs
}


#[derive(Default)]
pub struct TransactionBlockBuilder {
    coins: Option<Vec<Coin>>,
    wallets: Option<Vec<Wallet>>,
}

impl TransactionBlockBuilder {
    pub fn new() -> Self {
        TransactionBlockBuilder::default()
    }

    pub fn with_coins(mut self, coins: Vec<Coin>) -> Self {
        self.coins = Some(coins);
        self
    }

    pub fn with_wallets(mut self, wallets: Vec<Wallet>) -> Self {
        self.wallets = Some(wallets);
        self
    }

    
}


// Define the Coin struct with the Serialize trait
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coin {
    name: String,
    symbol: String,
    description: String,
    decimals: u8,
    total_supply: f64,
    fixed_supply: bool,
    address: String,
    package_id: String,
    creator: String,
    timestamp: String,
    object_id: String,
    image: String,
}

// Helper function to generate random strings (example implementation)
pub fn generate_random_string(len: usize) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let mut rng = rand::thread_rng();
    (0..len).map(|_| chars[rng.gen_range(0..chars.len())]).collect()
}

// Helper function to generate package IDs (example implementation)
pub fn generate_package_id(len: usize) -> String {
    generate_random_string(len)
}

pub async fn create_coin(
    name: &str,
    symbol: &str,
    description: &str,
    decimals: Option<u8>,
    total_supply: f64,
    fixed_supply: bool,
    creator: &str,
    image: &str,
) -> Result<String, reqwest::Error>{
    let decimals = decimals.unwrap_or(9);
    let object_id = generate_random_string(64);
    let package_id = generate_package_id(64);

    let mut hasher = Sha256::new();
    hasher.update(name);
    hasher.update(symbol);
    hasher.update(description);
    hasher.update(&object_id);
    hasher.update(&package_id);
    let hash = hasher.finalize();
    let address = format!("0x0{}::{}", hex::encode(&hash), name);
    //let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let now = Local::now();
    // Format the timestamp
    let _formatted_time = now.format("%-d/%-m/%Y, %-H:%M:%S").to_string();

    let coin = Coin {
        name: name.to_string(),
        symbol: symbol.to_string(),
        description: description.to_string(),
        image: image.to_string(),
        decimals,
        total_supply,
        fixed_supply,
        address,
        package_id,
        creator: creator.to_string(),
        timestamp:_formatted_time,
        object_id,
    };

    {
        let mut global_var = GLOBAL_FunctionName.lock().unwrap();
        *global_var = "create_coin".to_string();
    }
    println!("Function Name : {}", GLOBAL_FunctionName.lock().unwrap());

    let client = Client::new();
    let res = client.post(_URL)
        .json(&coin) // Serialize the coin to JSON
        .send()
        .await?;

        // Convert the response to a string
    let response_text = res.text().await?;
    if response_text!="Duplicate data found, coin not created" {
        let coin_data = TransactionBlock::empty();

        match coin_data.get_coin_metadata("C1").await{
        Some(coin) => println!("Got coin metadata: {:?}", coin),
        None => println!("Failed to get coin metadata"),
        
        }

    }else{
        println!("Duplicate data found, coin not created");
    }
    
    Ok(response_text)
    


    
}




pub struct NFT {
    pub name: String,
    pub description: String,
    pub image: String,
    pub total_supply: f64,
}

impl NFT {
    pub fn new(
        name: &str,
        description: &str,
        image_base64: &str,
        total_supply: f64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let decoded_image = BASE64_STANDARD.decode(image_base64)?;
        let image_size = decoded_image.len();

        if image_size > 1_048_576 {
            return Err("Image size exceeds 1MB.".into());
        }

        if image_size != 250 * 250 * 4 {
            return Err("Image dimensions are not 250x250.".into());
        }

        Ok(NFT {
            name: name.to_string(),
            description: description.to_string(),
            image: image_base64.to_string(),
            total_supply,
        })
    }
}


pub struct Domain {
    pub domain_name: String,
    pub wallet_address: String,
}

impl Domain {
    pub fn create_domain(domain_name: &str, wallet_address: &str) -> String {

        {
            let mut global_var = GLOBAL_FunctionName.lock().unwrap();
            *global_var = "create_domain".to_string();
        }
        println!("Function Name : {}", GLOBAL_FunctionName.lock().unwrap());

        // Format the domain_name with '@' prefix
        let formatted_domain_name = format!("@{}", domain_name);

        // Example validation (you can add more validations as needed)
        if wallet_address.is_empty() {
            //return Err("Wallet address cannot be empty".into());
        }

        // Create and return an instance of Domain

        let result = format!(
            "Domain Name : {}\nWallet Address : {}",
            formatted_domain_name, wallet_address
        );

        result
    }
}




pub fn create_wallet_address() -> String {
   
    {
        let mut global_var = GLOBAL_FunctionName.lock().unwrap();
        *global_var = "create_wallet_address".to_string();
    }
    println!("Function Name : {}", GLOBAL_FunctionName.lock().unwrap());
    
    let mut rng = rand::thread_rng();
    let random_bytes: [u8; 32] = rng.gen();

    let mut hasher = Sha256::new();
    hasher.update(&random_bytes);

    let result = hasher.finalize();

    let hex_string = hex::encode(result);

    // Join "0x1" with the hex string
    let joined_string = format!("0x1{}", hex_string);

    joined_string
}

/// Options for retrieving transaction block responses
#[derive(Debug, Clone)]
pub struct TransactionBlockResponseOptions {
    pub include_details: bool, // Whether to include detailed transaction information
    pub include_events: bool,  // Whether to include events related to the transaction
    pub include_status: bool,  // Whether to include the status of the transaction
}

impl TransactionBlockResponseOptions {
    pub fn new(include_details: bool, include_events: bool, include_status: bool) -> Self {
        TransactionBlockResponseOptions {
            include_details,
            include_events,
            include_status,
        }
    }
}