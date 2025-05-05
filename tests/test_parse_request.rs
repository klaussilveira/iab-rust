use iab::openrtb2::*;
use std::fs;
use std::path::Path;

fn load_and_parse_request(filename: &str) -> BidRequest {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/request")
        .join(filename);
    println!("Testing request file: {}", path.display());

    let content = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read file {filename}: {e}"));

    let result: Result<OpenRtb, _> = serde_json::from_str(&content);

    let deserialized_request =
        result.unwrap_or_else(|e| panic!("Failed to deserialize {filename}: {e:?}"));

    match deserialized_request {
        OpenRtb::BidRequest(req) => {
            println!("OK: {filename}");
            req
        }
        _ => panic!("Deserialized object for {filename} is not a BidRequest"),
    }
}

#[test]
fn test_simple_banner_request() {
    let req = load_and_parse_request("simple-banner.json");

    assert_eq!(req.id, "80ce30c53c16e6ede735f123ef6e32361bfc7b22");
    assert_eq!(req.at, Some(1));
    assert_eq!(req.cur, Some(vec!["USD".to_string()]));
    assert_eq!(req.imp.len(), 1);
    let imp = &req.imp[0];
    assert_eq!(imp.id, "1");
    assert_eq!(imp.bidfloor, Some(0.03));
    assert!(imp.banner.is_some());
    let banner = imp.banner.as_ref().unwrap();
    assert_eq!(banner.w, Some(300));
    assert_eq!(banner.h, Some(250));
    assert!(req.site.is_some());
    let site = req.site.as_ref().unwrap();
    assert_eq!(site.id.as_deref(), Some("102855"));
    assert_eq!(site.domain.as_deref(), Some("www.foobar.com"));
}

#[test]
fn test_expandable_creative_request() {
    let req = load_and_parse_request("expandable-creative.json");

    assert_eq!(req.id, "123456789316e6ede735f123ef6e32361bfc7b22");
    assert_eq!(req.at, Some(2));
    assert_eq!(req.cur, Some(vec!["USD".to_string()]));
    assert_eq!(req.imp.len(), 1);
    let imp = &req.imp[0];
    assert_eq!(imp.id, "1");
    assert_eq!(imp.bidfloor, Some(0.03));
    assert_eq!(
        imp.iframebuster,
        Some(vec!["vendor1.com".to_string(), "vendor2.com".to_string()])
    );
    assert!(imp.banner.is_some());
    let banner = imp.banner.as_ref().unwrap();
    assert_eq!(banner.w, Some(300));
    assert_eq!(banner.h, Some(250));
    assert_eq!(banner.battr, Some(vec![13]));
    assert_eq!(banner.expdir, Some(vec![2, 4]));
    assert!(req.site.is_some());
    let site = req.site.as_ref().unwrap();
    assert_eq!(site.id.as_deref(), Some("102855"));
    assert_eq!(site.domain.as_deref(), Some("www.foobar.com"));
    assert!(req.user.is_some());
    let user = req.user.as_ref().unwrap();
    assert_eq!(
        user.id.as_deref(),
        Some("55816b39711f9b5acf3b90e313ed29e51665623f")
    );
    assert_eq!(
        user.buyeruid.as_deref(),
        Some("545678765467876567898765678987654")
    );
}

#[test]
fn test_mobile_request() {
    let req = load_and_parse_request("mobile.json");

    assert_eq!(req.id, "IxexyLDIIk");
    assert_eq!(req.at, Some(2));
    assert_eq!(
        req.bcat,
        Some(vec![
            "IAB25".to_string(),
            "IAB7-39".to_string(),
            "IAB8-18".to_string(),
            "IAB8-5".to_string(),
            "IAB9-9".to_string()
        ])
    );
    assert_eq!(
        req.badv,
        Some(vec![
            "apple.com".to_string(),
            "go-text.me".to_string(),
            "heywire.com".to_string()
        ])
    );
    assert_eq!(req.imp.len(), 1);
    let imp = &req.imp[0];
    assert_eq!(imp.id, "1");
    assert_eq!(imp.bidfloor, Some(0.5));
    assert_eq!(imp.instl, Some(0));
    assert_eq!(
        imp.tagid.as_deref(),
        Some("agltb3B1Yi1pbmNyDQsSBFNpdGUY7fD0FAw")
    );
    assert!(imp.banner.is_some());
    let banner = imp.banner.as_ref().unwrap();
    assert_eq!(banner.w, Some(728));
    assert_eq!(banner.h, Some(90));
    assert_eq!(banner.pos, Some(1));
    assert_eq!(banner.btype, Some(vec![4]));
    assert_eq!(banner.battr, Some(vec![14]));
    assert_eq!(banner.api, Some(vec![3]));
    assert!(req.app.is_some());
    let app = req.app.as_ref().unwrap();
    assert_eq!(
        app.id.as_deref(),
        Some("agltb3B1Yi1pbmNyDAsSA0FwcBiJkfIUDA")
    );
    assert_eq!(app.name.as_deref(), Some("Yahoo Weather"));
    assert_eq!(app.bundle.as_deref(), Some("12345"));
    assert!(req.device.is_some());
    let device = req.device.as_ref().unwrap();
    assert_eq!(
        device.ua.as_deref(),
        Some(
            "Mozilla/5.0 (iPhone; CPU iPhone OS 6_1 like Mac OS X) AppleWebKit/534.46 (KHTML, like Gecko) Version/5.1 Mobile/9A334 Safari/7534.48.3"
        )
    );
    assert_eq!(
        device.ifa.as_deref(),
        Some("AA000DFE74168477C70D291f574D344790E0BB11")
    );
    assert_eq!(device.make.as_deref(), Some("Apple"));
    assert_eq!(device.model.as_deref(), Some("iPhone"));
    assert_eq!(device.osv.as_deref(), Some("6.1"));
    assert_eq!(device.devicetype, Some(1));
    assert!(req.user.is_some());
    let user = req.user.as_ref().unwrap();
    assert_eq!(
        user.id.as_deref(),
        Some("ffffffd5135596709273b3a1a07e466ea2bf4fff")
    );
}

#[test]
fn test_multiple_imp_request() {
    let req = load_and_parse_request("multiple-imp.json");

    assert_eq!(req.id, "some-request-id");
    assert!(req.site.is_some());
    let site = req.site.as_ref().unwrap();
    assert_eq!(site.page.as_deref(), Some("test.somepage.com"));
    assert_eq!(req.imp.len(), 3);

    // Impression 1 (Banner)
    let imp1 = &req.imp[0];
    assert_eq!(imp1.id, "imp-id-1");
    assert!(imp1.banner.is_some());
    assert!(imp1.video.is_none());
    assert!(imp1.audio.is_none());
    let banner1 = imp1.banner.as_ref().unwrap();
    assert_eq!(banner1.w, Some(100));
    assert!(banner1.format.is_some());
    let format1 = banner1.format.as_ref().unwrap();
    assert_eq!(format1.len(), 1);
    assert_eq!(format1[0].w, Some(300));
    assert_eq!(format1[0].h, Some(600));
    assert!(imp1.ext.is_some());

    // Impression 2 (Video)
    let imp2 = &req.imp[1];
    assert_eq!(imp2.id, "imp-id-2");
    assert!(imp2.banner.is_none());
    assert!(imp2.video.is_some());
    assert!(imp2.audio.is_none());
    let video2 = imp2.video.as_ref().unwrap();
    assert_eq!(video2.mimes, vec!["video/mp3".to_string()]);
    assert_eq!(video2.w, Some(200));
    assert!(imp2.ext.is_some());

    // Impression 3 (Audio/Native)
    let imp3 = &req.imp[2];
    assert_eq!(imp3.id, "imp-id-3");
    assert!(imp3.banner.is_none());
    assert!(imp3.video.is_none());
    assert!(imp3.audio.is_some());
    let audio3 = imp3.audio.as_ref().unwrap();
    assert_eq!(audio3.mimes, vec!["audio/mp4".to_string()]);
    assert_eq!(audio3.minduration, Some(1));
    assert_eq!(audio3.maxduration, Some(10));
    assert!(imp3.ext.is_some());
}

#[test]
fn test_pmp_with_direct_deal_request() {
    let req = load_and_parse_request("pmp-with-direct-deal.json");

    assert_eq!(req.id, "80ce30c53c16e6ede735f123ef6e32361bfc7b22");
    assert_eq!(req.at, Some(1));
    assert_eq!(req.imp.len(), 1);
    let imp = &req.imp[0];
    assert_eq!(imp.id, "1");
    assert!(imp.banner.is_some());
    let banner = imp.banner.as_ref().unwrap();
    assert_eq!(banner.w, Some(300));
    assert_eq!(banner.h, Some(250));
    assert!(imp.pmp.is_some());
    let pmp = imp.pmp.as_ref().unwrap();
    assert_eq!(pmp.private_auction, Some(1));
    assert!(pmp.deals.is_some());
    let deals = pmp.deals.as_ref().unwrap();
    assert_eq!(deals.len(), 2);
    // Deal 1
    assert_eq!(deals[0].id, "AB-Agency1-0001");
    assert_eq!(deals[0].at, Some(1));
    assert_eq!(deals[0].bidfloor, Some(2.5));
    assert_eq!(deals[0].wseat, Some(vec!["Agency1".to_string()]));
    // Deal 2
    assert_eq!(deals[1].id, "XY-Agency2-0001");
    assert_eq!(deals[1].at, Some(2));
    assert_eq!(deals[1].bidfloor, Some(2.0));
    assert_eq!(deals[1].wseat, Some(vec!["Agency2".to_string()]));
    assert!(req.site.is_some());
    let site = req.site.as_ref().unwrap();
    assert_eq!(site.id.as_deref(), Some("102855"));
    assert_eq!(site.domain.as_deref(), Some("www.foobar.com"));
    assert!(req.device.is_some());
    let device = req.device.as_ref().unwrap();
    assert!(device.ua.is_some());
    assert!(req.user.is_some());
    let user = req.user.as_ref().unwrap();
    assert_eq!(
        user.id.as_deref(),
        Some("55816b39711f9b5acf3b90e313ed29e51665623f")
    );
}

#[test]
fn test_video_request() {
    let req = load_and_parse_request("video.json");

    assert_eq!(req.id, "1234567893");
    assert_eq!(req.at, Some(2));
    assert_eq!(req.tmax, Some(120));
    assert_eq!(req.imp.len(), 1);
    let imp = &req.imp[0];
    assert_eq!(imp.id, "1");
    assert_eq!(imp.bidfloor, Some(0.03));
    assert!(imp.video.is_some());
    let video = imp.video.as_ref().unwrap();
    assert_eq!(video.w, Some(640));
    assert_eq!(video.h, Some(480));
    assert_eq!(video.pos, Some(1));
    assert_eq!(video.startdelay, Some(0));
    assert_eq!(video.minduration, Some(5));
    assert_eq!(video.maxduration, Some(30));
    assert_eq!(video.protocols, Some(vec![2, 3]));
    assert_eq!(
        video.mimes,
        vec![
            "video/x-flv".to_string(),
            "video/mp4".to_string(),
            "application/javascript".to_string()
        ]
    );
    assert_eq!(video.linearity, Some(1));
    assert_eq!(video.playbackmethod, Some(vec![1, 3]));
    assert_eq!(video.delivery, Some(vec![2]));
    assert!(video.companionad.is_some());
    let companion_ads = video.companionad.as_ref().unwrap();
    assert_eq!(companion_ads.len(), 2);
    assert_eq!(companion_ads[0].id.as_deref(), Some("1234567893-1"));
    assert_eq!(companion_ads[0].w, Some(300));
    assert_eq!(companion_ads[0].h, Some(250));
    assert_eq!(companion_ads[1].id.as_deref(), Some("1234567893-2"));
    assert_eq!(companion_ads[1].w, Some(728));
    assert_eq!(companion_ads[1].h, Some(90));
    assert_eq!(video.companiontype, Some(vec![1, 2]));
    assert!(req.site.is_some());
    let site = req.site.as_ref().unwrap();
    assert_eq!(site.id.as_deref(), Some("1345135123"));
    assert_eq!(site.domain.as_deref(), Some("siteabcd.com"));
    assert_eq!(site.privacypolicy, Some(1));
    assert!(site.publisher.is_some());
    assert!(site.content.is_some());
    let content = site.content.as_ref().unwrap();
    assert_eq!(content.id.as_deref(), Some("1234567"));
    assert_eq!(content.episode, Some(23));
    assert_eq!(content.title.as_deref(), Some("Car Show"));
    assert!(req.device.is_some());
    assert!(req.user.is_some());
    let user = req.user.as_ref().unwrap();
    assert_eq!(user.id.as_deref(), Some("456789876567897654678987656789"));
    assert_eq!(
        user.buyeruid.as_deref(),
        Some("545678765467876567898765678987654")
    );
}
