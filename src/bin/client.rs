use wizard::client::random_client::RandomClient;

fn main() {
    let mut client = RandomClient::new("0.0.0.0", "7878");

    if let Err(e) = client.client() {
        eprintln!("Error occurred: {e}");
    }
}
