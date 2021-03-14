use log::{trace};
use anyhow::{Result};
use crawler::NotifyData;
use crate::settings::Site;

pub struct HotlineParser {}

impl HotlineParser {
    pub fn parse_json(&self, response: String, site: &Site) -> Result<Option<NotifyData>> {
        trace!("Parsing json");
        let json: RootObject = serde_json::from_str(response.as_str())?;
        let max_price = site.max_allowed_price_uah.unwrap();
        trace!("Json has {:?} items", json.prices.len());

        for price in json.prices {

            // stock type id 1 is present
            if price.price_uah_real_raw <= max_price && price.stock_type_id == "1" {
                return Ok(
                    Some(
                        NotifyData {
                            message:
                            format!("{:?}. {:?} has a price of {:?} and stock type: {:?}",
                                    site.preflight_request.as_ref().unwrap(),
                                    price.firm_title,
                                    price.price_uah_real_raw,
                                    price.stock_type),
                            site_name: site.name.to_string(),
                        }));
            }
        }

        Ok(None)
    }
}

// models

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootObject {
    pub prices: Vec<Price>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    #[serde(rename = "is_shop_new")]
    pub is_shop_new: String,
    #[serde(rename = "checkout_only")]
    pub checkout_only: String,
    #[serde(rename = "new_shop_from_date")]
    pub new_shop_from_date: ::serde_json::Value,
    pub condition: String,
    #[serde(rename = "condition_title")]
    pub condition_title: String,
    pub rank: Option<String>,
    pub dt: String,
    pub id: String,
    pub popularity: String,
    #[serde(rename = "history_id")]
    pub history_id: String,
    #[serde(rename = "product_condition")]
    pub product_condition: String,
    #[serde(rename = "firm_city_id")]
    pub firm_city_id: String,
    #[serde(rename = "firm_city_genitive")]
    pub firm_city_genitive: String,
    #[serde(rename = "firm_region_id")]
    pub firm_region_id: String,
    pub currency: String,
    #[serde(rename = "official_vendor_status")]
    pub official_vendor_status: String,
    #[serde(rename = "official_icon_show")]
    pub official_icon_show: Option<String>,
    #[serde(rename = "official_vendor_status_hint")]
    pub official_vendor_status_hint: Option<String>,
    #[serde(rename = "official_vendor_status_link")]
    pub official_vendor_status_link: Option<String>,
    #[serde(rename = "merchant_title")]
    pub merchant_title: String,
    pub special: i64,
    #[serde(rename = "is_official")]
    pub is_official: bool,
    pub bolder: i64,
    #[serde(rename = "go_link")]
    pub go_link: String,
    pub url: String,
    #[serde(rename = "favorite_shop")]
    pub favorite_shop: String,
    #[serde(rename = "firm_logo")]
    pub firm_logo: String,
    pub slogan: String,
    pub date: String,
    pub title: String,
    #[serde(rename = "in_checkout")]
    pub in_checkout: Option<String>,
    #[serde(rename = "price_uah")]
    pub price_uah: String,
    #[serde(rename = "price_usd")]
    pub price_usd: String,
    #[serde(rename = "price_uah_real")]
    pub price_uah_real: String,
    #[serde(rename = "price_uah_real_raw")]
    pub price_uah_real_raw: f64,
    #[serde(rename = "yp_id")]
    pub yp_id: String,
    #[serde(rename = "firm_id")]
    pub firm_id: String,
    #[serde(rename = "firm_title")]
    pub firm_title: String,
    #[serde(rename = "firm_phones")]
    pub firm_phones: Vec<String>,
    #[serde(rename = "complaint_firm_title")]
    pub complaint_firm_title: String,
    #[serde(rename = "complaint_title")]
    pub complaint_title: String,
    #[serde(rename = "firm_webshop")]
    pub firm_webshop: String,
    #[serde(rename = "firm_website")]
    pub firm_website: String,
    #[serde(rename = "delivery_settings")]
    pub delivery_settings: ::serde_json::Value,
    #[serde(rename = "firm_shop_points")]
    pub firm_shop_points: ::serde_json::Value,
    #[serde(rename = "order_days")]
    pub order_days: String,
    #[serde(rename = "stock_type")]
    pub stock_type: String,
    #[serde(rename = "stock_type_id")]
    pub stock_type_id: String,
    #[serde(rename = "guarantee_format")]
    pub guarantee_format: String,
    #[serde(rename = "has_sales")]
    pub has_sales: bool,
    #[serde(rename = "has_free_delivery")]
    pub has_free_delivery: bool,
    #[serde(rename = "raw_offer_title")]
    pub raw_offer_title: String,
    pub title_short: String,
    #[serde(rename = "price_line_type")]
    pub price_line_type: String,
    #[serde(rename = "price_decimals")]
    pub price_decimals: String,
    #[serde(rename = "dollar_price_disabled")]
    pub dollar_price_disabled: bool,
    #[serde(rename = "price_usd_real")]
    pub price_usd_real: String,
    #[serde(rename = "is_auction")]
    pub is_auction: bool,
    pub cardid: String,
    pub sale: ::serde_json::Value,
    #[serde(rename = "price_old_decimals")]
    pub price_old_decimals: ::serde_json::Value,
    #[serde(rename = "price_old")]
    pub price_old: ::serde_json::Value,
    #[serde(rename = "hide_guarantee")]
    pub hide_guarantee: bool,
}