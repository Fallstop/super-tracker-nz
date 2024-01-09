use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseRoot {
    pub products: ApiResponseItems,
    pub isSuccessful: bool,
    pub dasFacets: Vec<ApiResponseDasFacet>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseItems {
    pub items: Vec<ApiResponseItem>,
    pub totalItems: usize
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ApiResponseItem {
    Product(ApiProduct),
    PromoTile(ApiPromoTile),
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiProduct {
    pub name: String,
    pub barcode: String,
    pub variety: Option<String>,
    pub brand: String,
    pub slug: String,
    pub sku: Option<String>,
    pub unit: String,
    pub price: ApiResponsePrice,
    pub images: ApiResponseImages,
    pub quantity: ApiResponseQuantity,
    pub stockLevel: usize,
    pub eachUnitQuantity: Option<String>,
    pub averageWeightPerUnit: Option<f32>,
    pub size: ApiResponseSize,
    pub departments: Vec<ApiResponseDepartment>,
    pub subsAllowed: bool,
    pub supportsBothEachAndKgPricing: bool,
    pub availabilityStatus: String,
    pub adId: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiPromoTile {
    pub name: String,
    pub id: usize,
    pub link: Option<String>,
    pub content: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponsePrice {
    pub originalPrice: Option<f32>,
    pub salePrice: Option<f32>,
    pub savePrice: Option<f32>,
    pub savePercentage: Option<f32>,
    pub canShowSavings: bool,
    pub hasBonusPoints: bool,
    pub isClubPrice: bool,
    pub isSpecial: bool,
    pub isNew: bool,
    pub canShowOriginalPrice: bool,
    pub discount: Option<String>,
    pub total: Option<String>,
    pub isTargetedOffer: bool,
    pub averagePricePerSingleUnit: Option<f32>,
    pub isBoostOffer: bool,
    pub purchasingUnitPrice: Option<String>,
    pub orderedPrice: Option<String>,
    pub isUsingOrderedPrice: bool,
    pub currentPricingMatchesOrderedPricing: Option<String>,
    pub extendedListPrice: Option<String>,
    pub originalAveragePricePerSingleUnit: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseImages {
    pub small: String,
    pub big: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseQuantity {
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub increment: Option<f32>,
    pub value: Option<String>,
    pub quantityInOrder: Option<String>,
    pub purchasingQuantityString: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseSize {
    pub cupPrice: Option<f32>,
    pub cupMeasure: Option<String>,
    pub packageType: Option<String>,
    pub volumeSize: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseDepartment {
    id: usize,
    name: String,
}

// {
//     "key": "Department",
//     "value": "1",
//     "isBooleanValue": false,
//     "name": "Fruit & Veg",
//     "productCount": 762,
//     "shelfResponses": null,
//     "group": "Department"
//   }

#[derive(Deserialize, Debug, Clone)]
pub struct ApiResponseDasFacet {
    pub key: String,
    pub value: String,
    pub name: String,
    pub productCount: usize,
    pub group: String,
}