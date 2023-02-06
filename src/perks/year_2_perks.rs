use std::collections::HashMap;

use crate::d2_enums::{StatHashes, WeaponType};

use super::{
    clamp,
    lib::{
        CalculationInput, DamageModifierResponse, ExplosivePercentResponse, ExtraDamageResponse,
        FiringModifierResponse, HandlingModifierResponse, MagazineModifierResponse,
        RangeModifierResponse, RefundResponse, ReloadModifierResponse, ReloadOverrideResponse,
    },
};

pub(super) fn sbr_air_assault(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let ae_per_stack = if _is_enhanced { 35 } else { 20 };
    let ae = ae_per_stack * _value as i32;
    stats.insert(StatHashes::AIRBORNE.to_u32(), ae);
    stats
}

pub(super) fn fmr_archers_tempo(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> FiringModifierResponse {
    FiringModifierResponse {
        burst_delay_scale: 0.75,
        burst_delay_add: 0.0,
        inner_burst_scale: 1.0,
        burst_size_add: 0.0,
    }
}

pub(super) fn dmr_explosive_head(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    if _pvp {
        DamageModifierResponse::new()
    } else {
        DamageModifierResponse {
            impact_dmg_scale: 1.0,
            explosive_dmg_scale: 1.3,
            crit_scale: 1.0,
        }
    }
}

pub(super) fn epr_explosive_head(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ExplosivePercentResponse {
    ExplosivePercentResponse {
        percent: 0.5,
        delyed: if _pvp { 0.0 } else { 0.2 },
        retain_base_total: true,
    }
}

pub(super) fn rsmr_feeding_frenzy(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let val = clamp(_value, 0, 5);
    let duration = 3.5;
    let mut reload_mult = 1.0;
    let mut reload = 0;
    if val == 1 {
        reload = 10;
        reload_mult = 1.0;
    } else if val == 2 {
        reload = 45;
        reload_mult = 0.9;
    } else if val == 3 {
        reload = 55;
        reload_mult = 0.88;
    } else if val == 4 {
        reload = 70;
        reload_mult = 0.85;
    } else if val == 5 {
        reload = 100;
        reload_mult = 0.8;
    };
    if _input.time_total > duration {
        reload = 0;
        reload_mult = 1.0;
    };
    ReloadModifierResponse {
        reload_stat_add: reload,
        reload_time_scale: reload_mult,
    }
}

pub(super) fn sbr_feeding_frenzy(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> HashMap<u32, i32> {
    let mut stats = HashMap::new();
    let val = clamp(_value, 0, 5);
    let duration = 3.5;
    let mut reload = 0;
    if val == 1 {
        reload = 10;
    } else if val == 2 {
        reload = 45;
    } else if val == 3 {
        reload = 55;
    } else if val == 4 {
        reload = 70;
    } else if val == 5 {
        reload = 100;
    };
    if _input.time_total > duration {
        reload = 0;
    };
    stats.insert(StatHashes::RELOAD.to_u32(), reload);
    stats
}

pub(super) fn dmr_firing_line(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut crit_mult = 1.0;
    if _value > 0 {
        crit_mult = 1.2;
    }
    DamageModifierResponse {
        crit_scale: crit_mult,
        explosive_dmg_scale: 1.0,
        impact_dmg_scale: 1.0,
    }
}

pub(super) fn rr_fourth_times(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> RefundResponse {
    RefundResponse {
        crit: true,
        requirement: 4,
        refund_mag: 2,
        refund_reserves: 0,
    }
}

pub(super) fn dmr_killing_tally(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let val = clamp(_value, 0, 3);
    let mut damage_mult = 0.1 * val as f64;
    if _pvp {
        damage_mult *= 0.5;
    };
    if _input.num_reloads > 0.0 {
        damage_mult = 0.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + damage_mult,
        explosive_dmg_scale: 1.0 + damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn mmr_overflow(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> MagazineModifierResponse {
    let mut mag_scale = if _value > 0 { 2.0 } else { 1.0 };
    if _input.total_shots_fired == 0.0 {
        mag_scale = 1.0;
    };
    MagazineModifierResponse {
        magazine_stat_add: 0,
        magazine_scale: mag_scale,
        magazine_add: 0.0,
    }
}

pub(super) fn rsmr_rapid_hit(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadModifierResponse {
    let values = vec![
        (0, 1.0),
        (5, 0.99),
        (30, 0.97),
        (35, 0.96),
        (45, 0.94),
        (60, 0.93),
    ];
    let entry_to_get = clamp(_value + _input.shots_fired_this_mag as u32, 0, 5);
    ReloadModifierResponse {
        reload_stat_add: values[entry_to_get as usize].0,
        reload_time_scale: values[entry_to_get as usize].1,
    }
}

pub(super) fn dmr_resevoir_burst(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = 1.0;
    if _input.curr_mag >= _input.base_mag {
        damage_mult = 1.25;
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_surrounded(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = 1.0;
    if _value > 0 {
        damage_mult = if *_input.weapon_type == WeaponType::SWORD {
            1.35
        } else {
            1.4
        };
        if _is_enhanced {
            damage_mult *= 1.05;
        };
    };
    DamageModifierResponse {
        impact_dmg_scale: damage_mult,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn ror_demolitionist(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> ReloadOverrideResponse {
    //todo implement system for cooldown
    let grenade_throw_time = 0.8;
    if _value == 1 {
        return ReloadOverrideResponse {
            valid: true,
            reload_time: _input.handling_data.ready_time + grenade_throw_time,
            ammo_to_reload: _input.base_mag as i32,
            priority: 0,
            count_as_reload: false,
            uses_ammo: true,
        };
    }
    ReloadOverrideResponse::invalid()
}

pub(super) fn dmr_full_court(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let mut damage_mult = 1.0;
    if _value > 0 {
        damage_mult = 1.25;
    };
    DamageModifierResponse {
        impact_dmg_scale: 1.0,
        explosive_dmg_scale: damage_mult,
        crit_scale: 1.0,
    }
}

pub(super) fn dmr_swash_buckler(
    _input: &CalculationInput,
    _value: u32,
    _is_enhanced: bool,
    _pvp: bool,
    _cached_data: &mut HashMap<String, f64>,
) -> DamageModifierResponse {
    let val = clamp(_value, 0, 5);
    let duration = if _is_enhanced { 6.0 } else { 4.5 };
    let mut dmg_boost = 0.067 * val as f64;
    if _input.time_total > duration {
        dmg_boost = 0.0;
    };
    DamageModifierResponse {
        impact_dmg_scale: 1.0 + dmg_boost,
        explosive_dmg_scale: 1.0 + dmg_boost,
        crit_scale: 1.0,
    }
}
