use iab::openrtb2::*;
use std::fs;
use std::path::Path;

fn load_and_parse_response(filename: &str) -> BidResponse {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/response")
        .join(filename);
    println!("Testing response file: {}", path.display());

    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read file {filename}: {e}"));

    let result: Result<OpenRtb, _> = serde_json::from_str(&content);

    let deserialized_response =
        result.unwrap_or_else(|e| panic!("Failed to deserialize {filename}: {e:?}"));

    match deserialized_response {
        OpenRtb::BidResponse(resp) => {
            println!("OK: {filename}");
            resp
        }
        _ => panic!("Deserialized object for {filename} is not a BidResponse"),
    }
}

#[test]
fn test_ad_served_on_win_notice_response() {
    let resp = load_and_parse_response("ad-served-on-win-notice.json");

    assert_eq!(resp.id, "1234567890");
    assert_eq!(resp.bidid.as_deref(), Some("abc1123"));
    assert_eq!(resp.cur, Some("USD".to_string()));
    assert!(resp.seatbid.is_some());
    let seatbid_vec = resp.seatbid.as_ref().unwrap();
    assert_eq!(seatbid_vec.len(), 1);
    let seatbid = &seatbid_vec[0];
    assert_eq!(seatbid.seat.as_deref(), Some("512"));
    assert_eq!(seatbid.bid.len(), 1);
    let bid = &seatbid.bid[0];
    assert_eq!(bid.id, "1");
    assert_eq!(bid.impid, "102");
    assert_eq!(bid.price, 9.43);
    assert_eq!(
        bid.nurl.as_deref(),
        Some("http://adserver.com/winnotice?impid=102")
    );
    assert_eq!(
        bid.iurl.as_deref(),
        Some("http://adserver.com/pathtosampleimage")
    );
    assert_eq!(bid.adomain, Some(vec!["advertiserdomain.com".to_string()]));
    assert_eq!(bid.cid.as_deref(), Some("campaign111"));
    assert_eq!(bid.crid.as_deref(), Some("creative112"));
    assert_eq!(bid.attr, Some(vec![1, 2, 3, 4, 5, 6, 7, 12]));
}

#[test]
fn test_direct_deal_ad_served_on_win_notice_response() {
    let resp = load_and_parse_response("direct-deal-ad-served-on-win-notice.json");

    assert_eq!(resp.id, "1234567890");
    assert_eq!(resp.bidid.as_deref(), Some("abc1123"));
    assert_eq!(resp.cur, Some("USD".to_string()));
    assert!(resp.seatbid.is_some());
    let seatbid_vec = resp.seatbid.as_ref().unwrap();
    assert_eq!(seatbid_vec.len(), 1);
    let seatbid = &seatbid_vec[0];
    assert_eq!(seatbid.seat.as_deref(), Some("512"));
    assert_eq!(seatbid.bid.len(), 1);
    let bid = &seatbid.bid[0];
    assert_eq!(bid.id, "1");
    assert_eq!(bid.impid, "102");
    assert_eq!(bid.price, 5.00);
    assert_eq!(bid.dealid.as_deref(), Some("ABC-1234-6789"));
    assert_eq!(
        bid.nurl.as_deref(),
        Some("http://adserver.com/winnotice?impid=102")
    );
    assert_eq!(bid.adomain, Some(vec!["advertiserdomain.com".to_string()]));
    assert_eq!(
        bid.iurl.as_deref(),
        Some("http://adserver.com/pathtosampleimage")
    );
    assert_eq!(bid.cid.as_deref(), Some("campaign111"));
    assert_eq!(bid.crid.as_deref(), Some("creative112"));
    assert_eq!(bid.adid.as_deref(), Some("314"));
    assert_eq!(bid.attr, Some(vec![1, 2, 3, 4]));
}

#[test]
fn test_multiple_imp_response() {
    let resp = load_and_parse_response("multiple-imp.json");

    assert_eq!(resp.id, "some-request-id");
    assert!(resp.seatbid.is_some());
    let seatbid_vec = resp.seatbid.as_ref().unwrap();
    assert_eq!(seatbid_vec.len(), 1);
    let seatbid = &seatbid_vec[0];
    assert_eq!(seatbid.seat.as_deref(), Some("appnexus"));
    assert_eq!(seatbid.bid.len(), 3);

    // Bid 1
    let bid1 = &seatbid.bid[0];
    assert_eq!(bid1.id, "apn-bid-1");
    assert_eq!(bid1.impid, "imp-id-1");
    assert_eq!(bid1.price, 0.1);
    assert_eq!(bid1.w, Some(3000));
    assert_eq!(bid1.h, Some(6000));
    assert_eq!(bid1.crid.as_deref(), Some("creative-1"));
    assert!(bid1.ext.is_some());

    // Bid 2
    let bid2 = &seatbid.bid[1];
    assert_eq!(bid2.id, "apn-bid-2");
    assert_eq!(bid2.impid, "imp-id-2");
    assert_eq!(bid2.price, 0.2);
    assert_eq!(bid2.w, Some(200));
    assert_eq!(bid2.h, Some(400));
    assert_eq!(bid2.crid.as_deref(), Some("creative-2"));
    assert!(bid2.ext.is_some());

    // Bid 3
    let bid3 = &seatbid.bid[2];
    assert_eq!(bid3.id, "apn-bid-3");
    assert_eq!(bid3.impid, "imp-id-3");
    assert_eq!(bid3.price, 0.3);
    assert_eq!(bid3.w, Some(300));
    assert_eq!(bid3.h, Some(600));
    assert_eq!(bid3.crid.as_deref(), Some("creative-3"));
    assert!(bid3.ext.is_some());
}

#[test]
fn test_native_markup_returned_inline_response() {
    let resp = load_and_parse_response("native-markup-returned-inline.json");

    assert_eq!(resp.id, "123");
    assert!(resp.seatbid.is_some());
    let seatbid_vec = resp.seatbid.as_ref().unwrap();
    assert_eq!(seatbid_vec.len(), 1);
    let seatbid = &seatbid_vec[0];
    assert!(seatbid.seat.is_none());
    assert_eq!(seatbid.bid.len(), 1);
    let bid = &seatbid.bid[0];
    assert_eq!(bid.id, "12345");
    assert_eq!(bid.impid, "2");
    assert_eq!(bid.price, 3.00);
    assert_eq!(bid.nurl.as_deref(), Some("http://example.com/winnoticeurl"));
    assert!(bid.adm.is_some());

    // Should be a native JSON object
    assert!(bid.adm.as_ref().unwrap().starts_with("{\"native\":{"));
}

#[test]
fn test_vast_xml_document_returned_inline_response() {
    let resp = load_and_parse_response("vast-xml-document-returned-inline.json");

    assert_eq!(resp.id, "123");
    assert!(resp.seatbid.is_some());
    let seatbid_vec = resp.seatbid.as_ref().unwrap();
    assert_eq!(seatbid_vec.len(), 1);
    let seatbid = &seatbid_vec[0];
    assert!(seatbid.seat.is_none());
    assert_eq!(seatbid.bid.len(), 1);
    let bid = &seatbid.bid[0];
    assert_eq!(bid.id, "12345");
    assert_eq!(bid.impid, "2");
    assert_eq!(bid.price, 3.00);
    assert_eq!(bid.nurl.as_deref(), Some("http://example.com/winnoticeurl"));
    assert!(bid.adm.is_some());

    // Should be a VAST XML
    assert!(bid.adm.as_ref().unwrap().starts_with("<?xml version="));
}
