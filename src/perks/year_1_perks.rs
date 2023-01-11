use std::collections::HashMap;

use crate::D2Enums::{AmmoType, StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExtraDamageResponse, FiringModifierResponse,
        HandlingModifierResponse, MagazineModifierResponse, RangeModifierResponse, RefundResponse,
        ReloadModifierResponse,
    },
};

pub(super) fn dmr_high_impact_reserves(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t
    }
    let mut out_dmg_scale = 1.0;
    let base_value = if _pvp { 0.03 } else { 0.121 };
    let max_value = if _pvp { 0.06 } else { 0.256 };
    let threshold_divisor = if _is_enhanced { 4.0 / 3.0 } else { 2.0 };
    if _input.curr_mag <= _input.curr_mag / threshold_divisor {
        let t = 1.0 - (_input.curr_mag - 1.0) / ((_input.base_mag / threshold_divisor) - 1.0);
        if t > 0.0 {
            out_dmg_scale = lerp(base_value, max_value, t);
        }
    };
    DamageModifierResponse {
        damage_scale: out_dmg_scale,
        crit_scale: 1.0,
    }
}

pub(super) fn hmr_threat_detector(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HandlingModifierResponse {
    let val = clamp(_value, 0, 2);
    let time_scale = 0.75_f64.powi(val);
    HandlingModifierResponse {
        handling_stat_add: 0,
        handling_swap_scale: time_scale,
        handling_ads_scale: 1.0,
    }
}

pub(super) fn rsmr_threat_detector(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> ReloadModifierResponse {
    let mut reload = 0;
    if _value == 1 {
        reload = 15;
    } else if _value == 2 {
        reload = 55;
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: 1.0,
    }
}

pub(super) fn sbr_threat_detector(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let mut stability = 0;
    let mut reload = 0;
    if _value == 1 {
        stability = 15;
        reload = 15;
    } else if _value == 2 {
        stability = 40;
        reload = 55;
    };
    let mut out = HashMap::new();
    out.insert(StatHashes::STABILITY.to_u32(), stability);
    out.insert(StatHashes::RELOAD.to_u32(), reload);
    out
}

pub(super) fn mmr_abitious_assasin(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> MagazineModifierResponse {
    let val = _value as f64;
    if _input.total_shots_hit == 0.0 {
        let mut mag_mult = 1.0;
        if _input.ammo_type == AmmoType::PRIMARY {
            mag_mult += 0.2 * val;
        } else {
            mag_mult += 0.1 * val;
        };
        return MagazineModifierResponse {
            magazine_stat_add: 0,
            magazine_scale: clamp(mag_mult, 1.0, 2.5),
            magazine_add: 0.0,
        };
    };
    MagazineModifierResponse {
        magazine_stat_add: 0,
        magazine_scale: 1.0,
        magazine_add: 0.0,
    }
}

pub(super) fn dmr_assasins_blade(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    let mut out_dmg_scale = 1.0;
    let duration = if _is_enhanced { 6.0 } else { 5.0 };
    if _input.time_total < duration {
        out_dmg_scale = 1.15;
    };
    DamageModifierResponse {
        damage_scale: out_dmg_scale,
        crit_scale: 1.0,
    }
}
pub(super) fn dmr_box_breathing(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    if _input.total_shots_hit == 0.0 {
        let crit_mult = (_input.base_crit_mult + 1.0) / _input.base_crit_mult;
        let dmg_mult = if _input.weapon_type == WeaponType::SCOUTRIFLE {
            0.95
        } else {
            1.0
        };
        return DamageModifierResponse {
            damage_scale: dmg_mult,
            crit_scale: crit_mult,
        };
    };
    DamageModifierResponse {
        damage_scale: 1.0,
        crit_scale: 1.0,
    }
}

pub(super) fn fmr_desperado(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> FiringModifierResponse {
    let mut delay_mult = 1.0;
    let duration = if _is_enhanced { 7.0 } else { 6.0 };
    if _input.time_total < duration {
        delay_mult = 0.7;
    };
    FiringModifierResponse {
        burst_delay_scale: delay_mult,
        burst_duration_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub(super) fn dmr_explosive_payload(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    if _pvp {
        DamageModifierResponse {
            damage_scale: 1.0,
            crit_scale: 1.0,
        }
    } else {
        let damage_mult = ((1.0 / _input.base_crit_mult) * 0.15) + 1.0;
        DamageModifierResponse {
            damage_scale: damage_mult,
            crit_scale: 1.0,
        }
    }
}

pub(super) fn dmr_timed_payload(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    if _pvp {
        DamageModifierResponse {
            damage_scale: 1.0,
            crit_scale: 1.0,
        }
    } else {
        let damage_mult = ((1.0 / _input.base_crit_mult) * 0.15) + 1.0;
        DamageModifierResponse {
            damage_scale: damage_mult,
            crit_scale: 1.0,
        }
    }
}

pub(super) fn sbr_field_prep(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let mut reload = 0;
    if _value == 1 {
        reload = if _is_enhanced { 55 } else { 50 };
    };
    let mut out = HashMap::new();
    out.insert(StatHashes::RELOAD.to_u32(), reload);
    out
}

pub(super) fn rsmr_field_prep(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> ReloadModifierResponse {
    let mut reload = 0;
    let mut reload_mult = 1.0;
    if _value == 1 {
        reload = if _is_enhanced { 55 } else { 50 };
        reload_mult = if _is_enhanced { 0.77 } else { 0.8 };
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: reload_mult,
    }
}

pub(super) fn sbr_firmly_planted(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let mut handling = if _is_enhanced { 35 } else { 30 };
    let mut stabiltiy = if _is_enhanced { 25 } else { 20 };
    if _input.weapon_type == WeaponType::FUSIONRIFLE {
        handling = handling / 2;
        stabiltiy = stabiltiy / 2;
    };
    let mut out = HashMap::new();
    out.insert(StatHashes::HANDLING.to_u32(), handling);
    out.insert(StatHashes::STABILITY.to_u32(), stabiltiy);
    out
}

pub(super) fn hmr_firmly_planted(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HandlingModifierResponse {
    let mut handling = if _is_enhanced { 35 } else { 30 };
    if _input.weapon_type == WeaponType::FUSIONRIFLE {
        handling = handling / 2;
    };
    HandlingModifierResponse {
        handling_stat_add: handling,
        handling_ads_scale: 1.0,
        handling_swap_scale: 1.0,
    }
}

pub(super) fn frm_full_auto_trigger(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> FiringModifierResponse {
    let mut delay_mult = 1.0;
    if _input.weapon_type == WeaponType::SHOTGUN {
        delay_mult = 0.91;
    };
    FiringModifierResponse {
        burst_delay_scale: delay_mult,
        burst_duration_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub(super) fn rr_triple_tap(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> RefundResponse {
    RefundResponse {
        crit: true,
        requirement: 3,
        refund: 1,
        generate_ammo: true,
    }
}

pub(super) fn sbr_hip_fire_grip(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let mut out = HashMap::new();
    out.insert(StatHashes::AIM_ASSIST.to_u32(), 15);
    out.insert(StatHashes::STABILITY.to_u32(), 25);
    out
}

pub(super) fn rmr_hip_fire_grip(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> RangeModifierResponse {
    let mut hf_range_scale = 1.2;
    if _input.weapon_type == WeaponType::FUSIONRIFLE || _input.weapon_type == WeaponType::SHOTGUN {
        hf_range_scale = 1.0;
    };
    RangeModifierResponse {
        range_stat_add: 0,
        range_all_scale: 1.0,
        range_hip_scale: hf_range_scale,
        range_zoom_scale: 1.0,
    }
}

pub(super) fn dmr_impact_casing(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> DamageModifierResponse {
    DamageModifierResponse {
        damage_scale: 0.025,
        crit_scale: 1.0,
    }
}

pub(super) fn sbr_moving_target(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let aim_assist = if _is_enhanced { 11 } else { 10 };
    let mut out = HashMap::new();
    out.insert(StatHashes::AIM_ASSIST.to_u32(), aim_assist);
    out
}

pub(super) fn sbr_opening_shot(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let aim_assist = if _is_enhanced { 25 } else { 20 };
    let range = if _is_enhanced { 30 } else { 25 };
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::AIM_ASSIST.to_u32(), aim_assist);
        out.insert(StatHashes::RANGE.to_u32(), range);
    }
    out
}

pub(super) fn rmr_opening_shot(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> RangeModifierResponse {
    let range = if _is_enhanced { 30 } else { 25 };
    RangeModifierResponse {
        range_stat_add: range,
        range_all_scale: 1.0,
        range_hip_scale: 1.0,
        range_zoom_scale: 1.0,
    }
}

pub(super) fn sbr_outlaw(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::RELOAD.to_u32(), 70);
    }
    out
}

pub(super) fn rmr_range_finder(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> RangeModifierResponse {
    RangeModifierResponse {
        range_stat_add: 0,
        range_all_scale: 1.0,
        range_hip_scale: 1.0,
        range_zoom_scale: 1.1,
    }
}

pub(super) fn sbr_slide_shot(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let stability = if _is_enhanced { 35 } else { 30 };
    let range = if _is_enhanced { 25 } else { 20 };
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::STABILITY.to_u32(), stability);
        out.insert(StatHashes::RANGE.to_u32(), range);
    }
    out
}

pub(super) fn rmr_slideshot(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> RangeModifierResponse {
    let mut range = if _is_enhanced { 25 } else { 20 };
    if _input.weapon_type == WeaponType::FUSIONRIFLE {
        range = 2; //only applies to first proj so like should do alot less
    }
    RangeModifierResponse {
        range_stat_add: range,
        range_all_scale: 1.0,
        range_hip_scale: 1.0,
        range_zoom_scale: 1.0,
    }
}

pub(super) fn sbr_slide_ways(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let stability = if _is_enhanced { 25 } else { 20 };
    let handling = if _is_enhanced { 25 } else { 20 };
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::STABILITY.to_u32(), stability);
        out.insert(StatHashes::HANDLING.to_u32(), handling);
    }
    out
}

pub(super) fn hmr_snapshot(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HandlingModifierResponse {
    let mut ads_mult = 0.5;
    if _input.ammo_type == AmmoType::SPECIAL {
        ads_mult = 0.8; //its 0.8 from my testing idk
    };
    HandlingModifierResponse {
        handling_stat_add: 0,
        handling_ads_scale: ads_mult,
        handling_swap_scale: 1.0,
    }
}

pub(super) fn sbr_tap_the_trigger(
    _input: &CalculationInput,
    _value: i32,
    _is_enhanced: bool,
    _pvp: bool,
) -> HashMap<u32, i32> {
    let mut stability = if _is_enhanced { 44 } else { 40 };
    if _input.weapon_type == WeaponType::FUSIONRIFLE {
        stability = stability / 4;
    }
    let mut out = HashMap::new();
    if _value > 0 {
        out.insert(StatHashes::RANGE.to_u32(), stability);
    }
    out
}
