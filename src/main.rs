use std::io::Read;

use rouille::Response;

use flate2::read::GzDecoder;

fn main() {
    rouille::start_server("0.0.0.0:9000", move |request| {
        println!("Request received. {} {}", request.method(), request.url());

        // Meilisearch will **always** sends you the content gzipped.
        assert_eq!(request.header("Content-Encoding"), Some("gzip"));

        let body = request.data().unwrap();

        // We'll unzip the content manually, most production webserver will do this for you.
        let mut gz = GzDecoder::new(body);
        let mut s = String::new();
        gz.read_to_string(&mut s).unwrap();

        println!("Received {s}");

        // Just to 100% validate the fact that the content is json lines we're going to parse every lines as json.
        let deserialized_payload = s
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect::<Vec<serde_json::Value>>();

        dbg!(deserialized_payload);

        // We don't return anything to Meilisearch
        Response::empty_204()
    });
}
