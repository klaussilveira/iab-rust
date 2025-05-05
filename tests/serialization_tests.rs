use iab::openrtb2::*;
use serde_json;
use std::fs;

#[test]
fn test_serializ() {
    let request = BidRequest {
        id: "80ce30c53c16e6ede735f123ef6e32361bfc7b22".to_string(),
        at: Some(1),
        cur: Some(vec!["USD".to_string()]),
        imp: vec![Imp {
            id: "1".to_string(),
            bidfloor: Some(0.03),
            banner: Some(Banner {
                h: Some(250),
                w: Some(300),
                pos: Some(0),
                ..Default::default()
            }),
            ..Default::default()
        }],
        site: Some(Site {
            id: Some("102855".to_string()),
            cat: Some(vec!["IAB3-1".to_string()]),
            domain: Some("www.foobar.com".to_string()),
            page: Some("http://www.foobar.com/1234.html".to_string()),
            publisher: Some(Publisher {
                id: Some("8953".to_string()),
                name: Some("foobar.com".to_string()),
                cat: Some(vec!["IAB3-1".to_string()]),
                domain: Some("foobar.com".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        user: Some(User {
            id: Some("55816b39711f9b5acf3b90e313ed29e51665623f".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Serialize the struct
    let generated_json_str =
        serde_json::to_string_pretty(&request).expect("Failed to serialize BidRequest");
    println!("Serialized JSON:\n{}\n", generated_json_str);

    // Read the test JSON
    let expected_json_str = fs::read_to_string("tests/request/simple-banner.json")
        .expect("Failed to read simple-banner.json");
    println!("Test JSON:\n{}\n", expected_json_str);

    // Parse both JSON files and compare them
    let generated_value: serde_json::Value =
        serde_json::from_str(&generated_json_str).expect("Failed to parse generated JSON");
    let expected_value: serde_json::Value =
        serde_json::from_str(&expected_json_str).expect("Failed to parse expected JSON test");
    assert_eq!(
        generated_value, expected_value,
        "Generated JSON does not match expected test JSON"
    );
}
