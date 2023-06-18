use bincode::serialize as to_bincode;
use serde::Serialize;
use serde_cbor::to_vec as to_cbor;
use serde_json::to_string as to_json;

#[derive(Serialize)]
struct City {
    name: String,
    population: usize,
    latitude: f64,
    longtitude: f64,
}
#[test]
fn listing_7_1() {
    let calabar = City {
        name: String::from("Calabar"),
        population: 470_000,
        latitude: 4.95,
        longtitude: 8.33,
    };

    let as_json = to_json(&calabar).unwrap();
    let as_cbor = to_cbor(&calabar).unwrap();
    let as_bincode = to_bincode(&calabar).unwrap();

    println!("json:\n{}\n", as_json);
    println!("cbor:\n{:?}\n", as_cbor);
    println!("bincode:\n{:?}\n", as_bincode);

    println!(
        "json (as UTF8):\n{}\n",
        String::from_utf8_lossy(as_json.as_bytes())
    );
    println!("cbor (as UTF8):\n{:?}\n", String::from_utf8_lossy(&as_cbor));
    println!(
        "bincode (as UTF8):\n{:?}\n",
        String::from_utf8_lossy(&as_bincode)
    );
}
