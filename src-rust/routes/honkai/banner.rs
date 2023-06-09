use crate::handler::{error::WorkerError, FromAxumResponse};
use axum::Json;
use response_derive::JsonResponse;
use serde::Serialize;
use vercel_runtime::{Body, Response, StatusCode};

use super::probability_rate::BannerType;

#[derive(Debug, Serialize, JsonResponse, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub banner_name: String,
    pub banner: f64,
    pub guaranteed: f64,
    pub guaranteed_pity: Option<i32>,
    pub const_prefix: String,
    pub min_const: i32,
    pub max_const: i32,
    pub max_pity: i32,
    pub banner_type: BannerType,
}
impl Banner {
    pub fn char_ssr() -> Self {
        Self {
            banner_name: "5* Banner character".into(),
            banner: 0.5,
            guaranteed: 1.0,
            guaranteed_pity: None,
            min_const: -1,
            max_const: 6,
            max_pity: 90,
            const_prefix: BannerType::Ssr.const_prefix(),
            banner_type: BannerType::Ssr,
        }
    }
    pub fn char_sr() -> Self {
        Self {
            banner_name: "Specific 4* banner character".into(),
            banner: 0.5,
            guaranteed: 0.333333333,
            guaranteed_pity: None,
            min_const: -1,
            max_const: 6,
            max_pity: 10,
            const_prefix: BannerType::Sr.const_prefix(),
            banner_type: BannerType::Sr,
        }
    }
    pub fn basic_weapon() -> Self {
        Self {
            banner_name: "5* Light Cone".into(),
            banner: 0.75,
            guaranteed: 1.0,
            guaranteed_pity: None,
            min_const: -1,
            max_const: 5,
            max_pity: 80,
            const_prefix: BannerType::Lc.const_prefix(),
            banner_type: BannerType::Lc,
        }
    }

    pub fn dev_weapon() -> Self {
        Self {
            banner_name: "Specific 5* banner weapon".into(),
            banner: 0.75,
            guaranteed: 0.5,
            guaranteed_pity: Some(3),
            min_const: 0,
            max_const: 5,
            max_pity: 80,
            const_prefix: BannerType::Lc.const_prefix(),
            banner_type: BannerType::Lc,
        }
    }

    pub fn to_internal(&self, pity_rate_fn: Box<dyn Fn(i32) -> f64>) -> BannerIternal {
        BannerIternal {
            banner_name: self.banner_name.to_owned(),
            banner: self.banner,
            guaranteed: self.guaranteed,
            guaranteed_pity: self.guaranteed_pity,
            min_const: self.min_const,
            max_const: self.max_const,
            max_pity: self.max_pity,
            rate: pity_rate_fn,
        }
    }
}

/// struct that is used in the backend
pub struct BannerIternal {
    pub banner_name: String,
    /// rate of the featured ssr (0.5 for character, 0.75 for LC)
    pub banner: f64,
    pub guaranteed: f64,
    /// not yet implemented, genshin epitomized path ???
    pub guaranteed_pity: Option<i32>,
    pub min_const: i32,
    pub max_const: i32,
    /// pity count (90 for char, 80 lc)
    pub max_pity: i32,
    // constFormat: string
    // constName: string
    pub rate: Box<dyn Fn(i32) -> f64>, // (pity: number) => number
}

#[derive(Serialize, JsonResponse, Clone, Debug)]
pub struct BannerList {
    pub banners: Vec<Banner>,
}
pub async fn gacha_banner_list() -> Result<Json<BannerList>, WorkerError> {
    let banner_list = BannerList {
        banners: vec![
            Banner::char_ssr(),
            Banner::char_sr(),
            Banner::basic_weapon(),
            // dev_weapon uses unreleased pity systems
        ],
    };
    Ok(Json(banner_list))
}
