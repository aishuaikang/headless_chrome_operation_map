use serde::Deserialize;

const REGIONS_JSON: &str = include_str!("../../data/regions.json");

#[derive(Debug, Deserialize)]
pub struct Region {
    pub id: String,
    pub name: String,
    pub fullname: String,
    pub location: Location,
    pub cidx: Option<Vec<i32>>,
}
#[derive(Debug, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

impl Region {
    pub fn get_regions() -> anyhow::Result<Vec<Region>> {
        Ok(serde_json::from_str(REGIONS_JSON)?)
    }

    pub fn get_map_region_by_region_full_name(region_full_name: &str) -> Option<Region> {
        let regions: Vec<Region> = Region::get_regions().ok()?;

        regions
            .into_iter()
            .find(|region| region_full_name.split("#").any(|i| region.fullname == i))
    }
}
