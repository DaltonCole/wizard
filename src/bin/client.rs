use wizard::client::client::Client;
use wizard::client::random_client::RandomClient;

fn main() {
    let mut client = RandomClient::new();

    if let Err(e) = client.client("0.0.0.0", "7878") {
        eprintln!("Error occurred: {e}");
    }
}
