# IAB Rust

This library provides strongly-typed structures for OpenRTB, AdCOM, and other specifications published by the Interactive Advertising Bureau (IAB) for the Rust programming language.

The primary goal is to define these types in an idiomatic Rust way, adhering strictly to the official specifications. This allows for easier integration and validation of OpenRTB-based protocols within Rust applications.

## Complete

- [OpenRTB 2.6](https://iabtechlab.com/standards/openrtb/)

## Roadmap

The following specifications are planned for future implementation:

- [AdCOM 1.0](https://iabtechlab.com/standards/openmedia/)
- [OpenRTB Dynamic Native Ads 1.2](https://iabtechlab.com/standards/openrtb-native/)

## Creating a Bid Request

Below is an example of creating a bid request:

```rust
use iab::openrtb2::*;
use serde_json;

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

let output = serde_json::to_string_pretty(&request).expect("Failed to serialize BidRequest");
```

## Parsing a Bid Request

Below is an example of parsing a bid request:

```rust
use iab::openrtb2::*;
use serde_json::{Result};

let data = r#"
    {
      "id": "80ce30c53c16e6ede735f123ef6e32361bfc7b22",
      "at": 1,
      "cur": ["USD"],
      "imp": [
        { "id": "1", "bidfloor": 0.03, "banner": {"h": 250, "w": 300, "pos": 0} }
      ],
      "site": {
        "id": "102855",
        "cat": ["IAB3-1"],
        "domain": "www.foobar.com",
        "page": "http://www.foobar.com/1234.html",
        "publisher": {
          "id": "8953",
          "name": "foobar.com",
          "cat": ["IAB3-1"],
          "domain": "foobar.com"
        }
      },
      "user": {"id": "55816b39711f9b5acf3b90e313ed29e51665623f"}
    }"#;

// Parse the Bid Request JSON
let br: BidRequest = serde_json::from_str(data).unwrap();

// Do things just like with any other Rust data structure
println!(
    "Bid Request {}, with impression {} on {:?}",
    br.id,
    br.imp[0].id,
    br.site.unwrap().domain
);
```

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
