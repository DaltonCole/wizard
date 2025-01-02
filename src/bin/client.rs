use wizard::client::random_client::RandomClient;

fn main() {
    let mut client = RandomClient::new("0.0.0.0", "7878");

    client.client();
}
