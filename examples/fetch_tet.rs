
const POST_ID: &str = "6f67def6-a63c-45e9-81f9-a49726a1f87d";

#[tokio::main]
pub async fn main() {
    
    let client = kasane::Client::new(None);
    
    let tet = kasane::Tet::get(&client, POST_ID).await.unwrap();
    
    println!("{:#?}", tet);
    
    let author = tet.get_author(&client).await.unwrap();
    
    println!("{:#?}", author);
    
    let thread = tet.get_thread(&client).await.unwrap();
    
    println!("{:#?}", thread);
    

    
}

