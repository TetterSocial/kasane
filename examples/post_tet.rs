
#[tokio::main]
pub async fn main() {
    
    let auth_key = dotenvy::var("AUTH").ok();
    
    println!("{:?}", auth_key);
    
    let client = kasane::Client::new(auth_key);
    
    
    let tet = kasane::Tet::compose(&client, "Hello from kasane.rs").await.unwrap();
    
    tet.rate(&client, kasane::Rating::Yeah).await.unwrap();
    
    
    println!("{:#?}", tet);
    
    // reply to the tet
    
    let reply = tet.reply(&client, "Hello again from kasane.rs, this is a test reply").await.unwrap();
    
    reply.rate(&client, kasane::Rating::Tomato).await.unwrap();
    
    println!("{:#?}", reply);
    
    
    // Get current tetifications
    let notifs = kasane::Tetification::get(&client).await.unwrap();
    
    println!("{:#?}", notifs);
}

