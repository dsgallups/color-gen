use fnv::FnvHashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Tailwind(FnvHashMap<String, TailwindColor>);

#[derive(Deserialize)]
pub struct TailwindColor {
    #[serde(rename = "DEFAULT")]
    default: String,
    #[serde(rename = "100")]
    l100: String,
    #[serde(rename = "200")]
    l200: String,
    #[serde(rename = "300")]
    l300: String,
    #[serde(rename = "400")]
    l400: String,
    #[serde(rename = "500")]
    l500: String,
    #[serde(rename = "600")]
    l600: String,
    #[serde(rename = "700")]
    l700: String,
    #[serde(rename = "800")]
    l800: String,
    #[serde(rename = "900")]
    l900: String,
}
