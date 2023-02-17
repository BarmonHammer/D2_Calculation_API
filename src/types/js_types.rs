#![cfg(feature = "wasm")]

use std::collections::HashMap;

use crate::{
    activity::damage_calc::DifficultyOptions,
    enemies::EnemyType,
    perks::Perk,
    types::rs_types::StatQuadraticFormula,
    weapons::{ttk_calc::ResillienceSummary, FiringData, Stat},
};
use serde::{Deserialize, Serialize};
// use tsify::Tsify;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use super::rs_types::{
    AmmoFormula, AmmoResponse, DamageMods, DpsResponse, FiringResponse, HandlingFormula,
    HandlingResponse, RangeFormula, RangeResponse, ReloadFormula, ReloadResponse,
};

#[derive(Debug, Clone, Copy, Serialize)]
#[wasm_bindgen(js_name = "HandlingResponse", inspectable)]
pub struct JsHandlingResponse {
    #[wasm_bindgen(js_name = "readyTime", readonly)]
    pub ready_time: f64,
    #[wasm_bindgen(js_name = "stowTime", readonly)]
    pub stow_time: f64,
    #[wasm_bindgen(js_name = "adsTime", readonly)]
    pub ads_time: f64,
}
impl From<HandlingResponse> for JsHandlingResponse {
    fn from(handling: HandlingResponse) -> Self {
        JsHandlingResponse {
            ready_time: handling.ready_time,
            stow_time: handling.stow_time,
            ads_time: handling.ads_time,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[wasm_bindgen(js_name = "RangeResponse", inspectable)]
pub struct JsRangeResponse {
    #[wasm_bindgen(js_name = "hipFalloffStart", readonly)]
    pub hip_falloff_start: f64,
    #[wasm_bindgen(js_name = "hipFalloffEnd", readonly)]
    pub hip_falloff_end: f64,
    #[wasm_bindgen(js_name = "adsFalloffStart", readonly)]
    pub ads_falloff_start: f64,
    #[wasm_bindgen(js_name = "adsFalloffEnd", readonly)]
    pub ads_falloff_end: f64,
    #[wasm_bindgen(js_name = "floorPercent", readonly)]
    pub floor_percent: f64,
}
impl From<RangeResponse> for JsRangeResponse {
    fn from(range: RangeResponse) -> Self {
        JsRangeResponse {
            hip_falloff_start: range.hip_falloff_start,
            hip_falloff_end: range.hip_falloff_end,
            ads_falloff_start: range.ads_falloff_start,
            ads_falloff_end: range.ads_falloff_end,
            floor_percent: range.floor_percent,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[wasm_bindgen(js_name = "ReloadResponse", inspectable)]
pub struct JsReloadResponse {
    #[wasm_bindgen(js_name = "reloadTime", readonly)]
    pub reload_time: f64,
    #[wasm_bindgen(js_name = "ammoTime", readonly)]
    pub ammo_time: f64,
}
impl From<ReloadResponse> for JsReloadResponse {
    fn from(reload: ReloadResponse) -> Self {
        JsReloadResponse {
            reload_time: reload.reload_time,
            ammo_time: reload.ammo_time,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[wasm_bindgen(js_name = "AmmoResponse", inspectable)]
pub struct JsAmmoResponse {
    #[wasm_bindgen(js_name = "magSize" ,readonly)]
    pub mag_size: i32,
    #[wasm_bindgen(js_name = "reserveSize", readonly)]
    pub reserve_size: i32,
}
impl From<AmmoResponse> for JsAmmoResponse {
    fn from(ammo: AmmoResponse) -> Self {
        JsAmmoResponse {
            mag_size: ammo.mag_size,
            reserve_size: ammo.reserve_size,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[wasm_bindgen(js_name = "DpsResponse")]
pub struct JsDpsResponse {
    #[wasm_bindgen(skip)]
    pub dps_per_mag: Vec<f64>,
    #[wasm_bindgen(skip)]
    pub time_damage_data: Vec<(f64, f64)>,
    #[wasm_bindgen(js_name = "totalDamage", readonly)]
    pub total_damage: f64,
    #[wasm_bindgen(js_name = "totalTime", readonly)]
    pub total_time: f64,
    #[wasm_bindgen(js_name = "totalShots", readonly)]
    pub total_shots: i32,
}
#[wasm_bindgen(js_class = "DpsResponse")]
impl JsDpsResponse {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(self) -> String {
        serde_wasm_bindgen::to_value(&self)
            .unwrap()
            .as_string()
            .unwrap()
    }
    ///Returns a list of tuples of time and damage
    #[wasm_bindgen(getter, js_name = "timeDamageData")]
    pub fn time_damage_data(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.time_damage_data).unwrap()
    }
    ///Returns a list of dps values for each magazine
    #[wasm_bindgen(getter, js_name = "dpsPerMag")]
    pub fn dps_per_mag(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.dps_per_mag).unwrap()
    }
}
impl From<DpsResponse> for JsDpsResponse {
    fn from(dps: DpsResponse) -> Self {
        JsDpsResponse {
            dps_per_mag: dps.dps_per_mag,
            time_damage_data: dps.time_damage_data,
            total_damage: dps.total_damage,
            total_time: dps.total_time,
            total_shots: dps.total_shots,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[wasm_bindgen(js_name = "TtkResponse")]
pub struct JsTtkResponse {
    #[wasm_bindgen(skip)]
    pub data: Vec<ResillienceSummary>,
}
#[wasm_bindgen(js_class = "TtkResponse")]
impl JsTtkResponse {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(self) -> String {
        serde_wasm_bindgen::to_value(&self)
            .unwrap()
            .as_string()
            .unwrap()
    }
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.data).unwrap()
    }
}

#[derive(Debug, Clone, Default, Serialize)]
#[wasm_bindgen(js_name = "FiringResponse", inspectable)]
pub struct JsFiringResponse {
    #[wasm_bindgen(js_name = "pvpImpactDamage", readonly)]
    pub pvp_impact_damage: f64,
    #[wasm_bindgen(js_name = "pvpExplosionDamage", readonly)]
    pub pvp_explosion_damage: f64,
    #[wasm_bindgen(js_name = "pvpCritMult", readonly)]
    pub pvp_crit_mult: f64,

    #[wasm_bindgen(js_name = "pveImpactDamage", readonly)]
    pub pve_impact_damage: f64,
    #[wasm_bindgen(js_name = "pveExplosionDamage", readonly)]
    pub pve_explosion_damage: f64,
    #[wasm_bindgen(js_name = "pveCritMult", readonly)]
    pub pve_crit_mult: f64,

    #[wasm_bindgen(js_name = "burstDelay", readonly)]
    pub burst_delay: f64,
    #[wasm_bindgen(js_name = "innerBurstDelay", readonly)]
    pub inner_burst_delay: f64,
    #[wasm_bindgen(js_name = "burstSize", readonly)]
    pub burst_size: i32,

    pub rpm: f64,
}
impl From<FiringResponse> for JsFiringResponse {
    fn from(firing: FiringResponse) -> Self {
        JsFiringResponse {
            pvp_impact_damage: firing.pvp_impact_damage,
            pvp_explosion_damage: firing.pvp_explosion_damage,
            pvp_crit_mult: firing.pvp_crit_mult,
            pve_impact_damage: firing.pve_impact_damage,
            pve_explosion_damage: firing.pve_explosion_damage,
            pve_crit_mult: firing.pve_crit_mult,
            burst_delay: firing.burst_delay,
            inner_burst_delay: firing.inner_burst_delay,
            burst_size: firing.burst_size,
            rpm: firing.rpm,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
#[wasm_bindgen(js_name = "Stat")]
pub struct JsStat {
    #[wasm_bindgen(js_name = "baseValue")]
    pub base_value: i32,
    #[wasm_bindgen(js_name = "partValue")]
    pub part_value: i32,
    #[wasm_bindgen(js_name = "traitValue")]
    pub trait_value: i32,
}
#[wasm_bindgen(js_class = "Stat")]
impl JsStat {
    #[wasm_bindgen(js_name = "toString")]
    pub fn to_string(self) -> String {
        format!("{:?}", self)
    }
}
impl From<Stat> for JsStat {
    fn from(stat: Stat) -> Self {
        JsStat {
            base_value: stat.base_value,
            part_value: stat.part_value,
            trait_value: stat.perk_value,
        }
    }
}

#[derive(Debug, Clone)]
#[wasm_bindgen(js_name = "DifficultyOptions")]
pub enum JsDifficultyOptions {
    NORMAL = 1,
    RAID = 2,
    MASTER = 3,
}
impl Into<DifficultyOptions> for JsDifficultyOptions {
    fn into(self) -> DifficultyOptions {
        match self {
            JsDifficultyOptions::NORMAL => DifficultyOptions::NORMAL,
            JsDifficultyOptions::RAID => DifficultyOptions::RAID,
            JsDifficultyOptions::MASTER => DifficultyOptions::MASTER,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[wasm_bindgen(js_name = "EnemyType")]
pub enum JsEnemyType {
    MINOR,
    ELITE,
    MINIBOSS,
    BOSS,
    VEHICLE,
    ENCLAVE,
    PLAYER,
    CHAMPION,
}
impl Into<EnemyType> for JsEnemyType {
    fn into(self) -> EnemyType {
        match self {
            JsEnemyType::MINOR => EnemyType::MINOR,
            JsEnemyType::ELITE => EnemyType::ELITE,
            JsEnemyType::MINIBOSS => EnemyType::MINIBOSS,
            JsEnemyType::BOSS => EnemyType::BOSS,
            JsEnemyType::VEHICLE => EnemyType::VEHICLE,
            JsEnemyType::ENCLAVE => EnemyType::ENCLAVE,
            JsEnemyType::PLAYER => EnemyType::PLAYER,
            JsEnemyType::CHAMPION => EnemyType::CHAMPION,
        }
    }
}
