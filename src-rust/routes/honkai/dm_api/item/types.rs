use crate::{
    handler::error::WorkerError,
    routes::honkai::{
        dm_api::{hash::TextHash, types::TextMap},
        traits::DbData,
    },
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, EnumString, EnumIter, Display)]
pub enum ItemType {
    Usable,
    Mission,
    Display,
    Virtual,
    Material,
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, EnumString, EnumIter, Display)]
pub enum ItemSubType {
    Book,
    Virtual,
    Gift,
    ChatBubble,
    Food,
    PhoneTheme,
    GameplayCounter,
    RelicRarityShowOnly,
    ForceOpitonalGift,
    Material,
    MuseumExhibit,
    RelicSetShowOnly,
    MuseumStuff,
    Formula,
    Mission,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, EnumString, EnumIter, Display)]
pub enum ItemRarity {
    VeryRare,
    SuperRare,
    Rare,
    NotNormal,
    Normal,
}

#[derive(Serialize, Deserialize)]
pub struct UpstreamItem {
    #[serde(alias = "ID")]
    id: u32,
    #[serde(alias = "ItemMainType")]
    item_main_type: ItemType,
    #[serde(alias = "ItemSubType")]
    item_sub_type: ItemSubType,
    #[serde(alias = "InventoryDisplayTag")]
    inventory_display_tag: u32,
    #[serde(alias = "Rarity")]
    rarity: ItemRarity,
    #[serde(alias = "PurposeType")]
    purpose_type: Option<u32>,
    #[serde(alias = "ItemName")]
    item_name: TextHash,
    #[serde(alias = "ItemDesc")]
    item_desc: TextHash,
    #[serde(alias = "ItemBGDesc")]
    item_bgdesc: TextHash,
    #[serde(alias = "ItemIconPath")]
    item_icon_path: String,
    #[serde(alias = "ItemFigureIconPath")]
    item_figure_icon_path: String,
    #[serde(alias = "ItemCurrencyIconPath")]
    item_currency_icon_path: String,
    #[serde(alias = "ItemAvatarIconPath")]
    item_avatar_icon_path: String,
    #[serde(alias = "PileLimit")]
    pile_limit: u32,
    // unknown
    #[serde(alias = "CustomDataList")]
    custom_data_list: Vec<u32>,
    // unknown
    #[serde(alias = "ReturnItemIDList")]
    return_item_idlist: Vec<u32>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub item_main_type: ItemType,
    pub item_sub_type: ItemSubType,
    pub inventory_display_tag: u32,
    pub rarity: ItemRarity,
    pub purpose_type: Option<u32>,
    pub item_name: String,
    pub item_desc: String,
    pub item_bgdesc: String,
    #[serde(skip)]
    item_icon_path: String,
    #[serde(skip)]
    item_figure_icon_path: String,
    #[serde(skip)]
    item_currency_icon_path: String,
    #[serde(skip)]
    item_avatar_icon_path: String,
    pub pile_limit: u32,
    // unknown
    pub custom_data_list: Vec<u32>,
    // unknown
    #[serde(skip)]
    pub return_item_idlist: Vec<u32>,
}

#[async_trait]
impl DbData for Item {
    type TUpstream = BTreeMap<u32, UpstreamItem>;
    type TLocal = BTreeMap<u32, Item>;

    fn path_data() -> &'static str {
        "ExcelOutput/ItemConfig.json"
    }

    async fn upstream_convert(
        item_db: BTreeMap<u32, UpstreamItem>,
    ) -> Result<BTreeMap<u32, Item>, WorkerError> {
        let text_map: HashMap<String, String> = TextMap::read().await?;
        let transformed = item_db
            .into_iter()
            .map(|(key, value)| {
                let converted_value = value.raw_into(&text_map);
                (key, converted_value)
            })
            .collect();
        Ok(transformed)
    }
}

impl UpstreamItem {
    fn raw_into(&self, text_map: &HashMap<String, String>) -> Item {
        Item {
            id: self.id,
            item_main_type: self.item_main_type,
            item_sub_type: self.item_sub_type,
            inventory_display_tag: self.inventory_display_tag,
            rarity: self.rarity,
            purpose_type: self.purpose_type,
            item_name: self
                .item_name
                .read_from_textmap(text_map)
                .unwrap_or_default(),
            item_desc: self
                .item_desc
                .read_from_textmap(text_map)
                .unwrap_or_default(),
            item_bgdesc: self
                .item_bgdesc
                .read_from_textmap(text_map)
                .unwrap_or_default(),
            item_icon_path: self.item_icon_path.clone(),
            item_figure_icon_path: self.item_figure_icon_path.clone(),
            item_currency_icon_path: self.item_currency_icon_path.clone(),
            item_avatar_icon_path: self.item_avatar_icon_path.clone(),
            pile_limit: self.pile_limit,
            custom_data_list: self.custom_data_list.clone(),
            return_item_idlist: self.return_item_idlist.clone(),
        }
    }
}
