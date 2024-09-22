# kasane

kasane is a simple (and blazingly fast 🚀) library for the Tetter social media platform API.

It allows you to post tets at blazingly-fast(🚀) speeds, powered by Rust, Reqwest and Tokio.

> [!NOTE]
> Both Tetter and kasane.rs are still in development. Things may change as time goes on.


## Usage

1. Log in to your Tetter account on [https://tetter.vercel.app].
2. Find your token (required for authenticated requests).
    You can find this in 2 ways:
    - Find the Firebase token in IndexedDB
      1. Open DevTools, and enter the `Storage` tab
      2. Go to IndexedDB, and find `tetter.vercel.app > firebaseLocalStorageDb > firebase:authUser:XXXXXXXX > value > stsTokenManager > accessToken`
      3. Copy the value of `accessToken`
    - Get the token from headers
      1. Open DevTools, and enter the `Network` tab
      2. Do any action that requires authorization, like checking notifications
      3. Find the `Authorization` header, and copy the value of `Bearer XXXXXXXX`
3. Now use kasane to post your tets!

```rust
use kasane::Tetter;

#[tokio::main]
async fn main() {
    let client = kasane::Client::new(Some("your_token_here".to_string()));
    let tet = kasane::Tet::compose(&client, "Hello from kasane.rs").await.unwrap();
    println!("{:?}", tet);
}
```

See [examples/](examples/) for more examples.
