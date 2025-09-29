use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;
use crate::defaults::default_optional_string_usd;
use super::SeatBid;

#[cfg(feature="coercion")]
use crate::json_coercion::{AsString, AsI64};
#[cfg(feature="coercion")]
use serde_with::serde_as;

#[cfg(feature="utoipa")]
use utoipa::ToSchema;

/// Object: BidResponse
/// This object is the top-level bid response object (i.e., the unnamed outer JSON
/// object). The `id` attribute reflects the bid request ID for logging purposes.
/// Similarly, `bidid` is an optional response tracking ID for bidders. If specified,
/// it can be included in the subsequent win notice call if the bidder wins. At least
/// one `seatbid` object is required, which contains at least one bid for an impression.
/// Other attributes are optional.
///
/// To express a "no-bid", the options are to return an empty response with HTTP 204.
/// Alternately if the bidder wishes to convey to the exchange a reason for not bidding,
/// just a `BidResponse` object is returned with a reason code in the `nbr` attribute.
#[cfg_attr(feature="coercion", cfg_eval::cfg_eval, serde_as)]
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature="utoipa", derive(ToSchema))]
#[cfg_attr(feature="utoipa", schema(
    description="OpenRTB Bid Response - Response containing bids for the requested impressions",
    example=json!({
        "id": "test-request-123",
        "seatbid": [{
            "bid": [{
                "id": "bid-123",
                "impid": "imp-1",
                "price": 0.75,
                "adm": "<div style='width:320px;height:50px;background:#4CAF50;color:white;display:flex;align-items:center;justify-content:center;'>Sample Ad</div>",
                "crid": "creative-001"
            }],
            "seat": "angostura"
        }],
        "cur": "USD"
    })
))]
pub struct BidResponse {
    /// ID of the bid request to which this is a response.
    #[cfg_attr(feature="coercion", serde_as(as="AsString"))]
    pub id: String,
    /// Array of seatbid objects; 1+ required if a bid is to be made.
    pub seatbid: Option<Vec<SeatBid>>,
    /// Bidder generated response ID to assist with logging/tracking.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub bidid: Option<String>,
    /// Bid currency using ISO-4217 alpha codes.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    #[serde(default="default_optional_string_usd")]
    pub cur: Option<String>,
    /// Optional feature to allow a bidder to set data in the exchange's
    /// cookie. The string must be in base85 cookie safe characters and be
    /// in any format. Proper JSON encoding must be used to include
    /// "escaped" quotation marks.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsString>"))]
    pub customdata: Option<String>,
    /// Reason for not bidding. Refer to List: No-Bid Reason Codes
    /// in OpenRTB 3.0.
    #[cfg_attr(feature="coercion", serde_as(as="Option<AsI64>"))]
    pub nbr: Option<i64>,
    /// Placeholder for bidder-specific extensions to OpenRTB.
    pub ext: Option<Value>,
}
