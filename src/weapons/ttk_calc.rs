use std::collections::HashMap;

use serde::Serialize;

use crate::{perks::{lib::CalculationInput, get_dmg_modifier, get_firing_modifier}};

use super::{FiringConfig, Weapon};

//just to make code cleaner for now
fn ceil(x: f64) -> f64 {
    x.ceil()
}

const RESILIENCE_VALUES: [f64; 11] = [
    185.01, 186.01, 187.01, 188.01, 189.01, 190.01, 192.01, 194.01, 196.01, 198.01, 200.01,
];

fn average_range(_range_data: &Vec<(f64, f64)>, _wanted_percent: f64, _dmagae_floor: f64) -> f64 {
    let mut total_range = 0.0;
    let mut num_entries = 0.0;
    for range_pair in _range_data {
        total_range += (range_pair.1-range_pair.0)*((1.0-_dmagae_floor)*_wanted_percent) + range_pair.0;
        num_entries += 1.0;
    }
    total_range/num_entries
}

#[derive(Debug, Clone, Serialize)]
pub struct OptimalKillData{
    pub headshots: i32,
    pub bodyshots: i32,
    pub time_taken: f64,
    //defines how far away this ttk is achievalbe if all hits ar crits
    pub all_crit_range: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct BodyKillData{
    pub bodyshots: i32,
    pub time_taken: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResillienceSummary{
    pub value: i32,
    pub body_ttk: BodyKillData,
    pub optimal_ttk: OptimalKillData,
}

pub fn calc_ttk(_weapon: &Weapon, _overshield: f64, _explosive_percent: f64) -> Vec<ResillienceSummary> {
    let mut ttk_data: Vec<ResillienceSummary> = Vec::new();
    for i in 0..RESILIENCE_VALUES.len() {
        let health = RESILIENCE_VALUES[i] + _overshield;

        let mut opt_damage_dealt =  0.0_f64;
        let mut opt_time_taken =    0.0_f64;
        let mut opt_bullets_fired =   0.0_f64;
        let mut opt_bullets_hit =   0.0_f64;
        let mut opt_range_data_vec: Vec<(f64, f64)> = Vec::new();
        let mut opt_infnite_range = _explosive_percent >= 1.0;
        let mut opt_bodyshots = 0;
        let mut opt_headshots = 0;
        //Optimal ttk
        while opt_bullets_hit < 50.0 {
            //PERK CALCULATIONS////////////
            let calc_input = _weapon.pvp_calc_input(
                opt_bullets_fired,
                opt_bullets_hit,
                opt_time_taken,
                (_overshield-opt_damage_dealt)>0.0
            );
            let dmg_mods = get_dmg_modifier(_weapon.list_perks().clone(), &calc_input, true);
            let firing_mods = get_firing_modifier(_weapon.list_perks().clone(), &calc_input, true);
            let tmp_range_data = _weapon.calc_range_falloff(Some(calc_input));
            if tmp_range_data.ads_falloff_start > 998.0 {
                opt_infnite_range = true;
            } else {
                opt_range_data_vec.push((tmp_range_data.ads_falloff_start, tmp_range_data.ads_falloff_end));
            }
            ///////////////////////////////

            let body_damage = _weapon.firing_data.damage*dmg_mods.dmg_scale;
            let critical_multiplier = _weapon.firing_data.crit_mult*dmg_mods.crit_scale;
            let head_diff = (body_damage*critical_multiplier) - body_damage;

            let shot_burst_delay =
                    (_weapon.firing_data.burst_delay + firing_mods.burst_delay_add) * firing_mods.burst_delay_scale;
            let shot_burst_duration = _weapon.firing_data.burst_duration * firing_mods.burst_duration_scale;
            let shot_burst_size = _weapon.firing_data.burst_size as f64 + firing_mods.burst_size_add;
            let shot_inner_burst_delay = shot_burst_duration / (shot_burst_size - 1.0);

            let shot_delay = if opt_bullets_hit%shot_burst_size > 0.0 {
                shot_inner_burst_delay
            } else {
                shot_burst_delay
            };

            if opt_bullets_hit%shot_burst_size == 0.0 {
                opt_bullets_fired += 1.0;
                opt_bullets_hit += 1.0;
            } else {
                opt_bullets_hit += 1.0;
            };

            if (opt_damage_dealt + body_damage) > health {
                opt_bodyshots += 1;
                opt_damage_dealt += body_damage;
                break;
            } else if (opt_damage_dealt + body_damage + head_diff) > health {
                opt_headshots += 1;
                opt_damage_dealt += body_damage + head_diff;
                break;
            } else {
                opt_headshots += 1;
                opt_damage_dealt += body_damage + head_diff;
            }
            opt_time_taken += shot_delay;
        }
        let dropoff_wanted: f64 = ((opt_damage_dealt - health) / opt_damage_dealt) / (1.0 - _explosive_percent);
        let range_possible = if !opt_infnite_range {
                average_range(&opt_range_data_vec, dropoff_wanted, _weapon.range_formula.floor_percent)} 
            else {
                999.9
            };
        let optimal_ttk = OptimalKillData{
            headshots: opt_headshots,
            bodyshots: opt_bodyshots,
            time_taken: opt_time_taken,
            all_crit_range: range_possible,
        };

        let mut bdy_bullets_hit = 0.0;
        let mut bdy_bullets_fired = 0.0;
        let mut bdy_time_taken = 0.0;
        let mut bdy_damage_dealt = 0.0;
        while bdy_bullets_hit < 50.0 {
            //PERK CALCULATIONS////////////
            let calc_input = _weapon.pvp_calc_input(
                bdy_bullets_fired,
                bdy_bullets_hit,
                bdy_time_taken,
                (_overshield-bdy_damage_dealt)>0.0
            );
            let dmg_mods = get_dmg_modifier(_weapon.list_perks().clone(), &calc_input, true);
            let firing_mods = get_firing_modifier(_weapon.list_perks().clone(), &calc_input, true);
            ///////////////////////////////

            let body_damage = _weapon.firing_data.damage*dmg_mods.dmg_scale;

            let shot_burst_delay =
                    (_weapon.firing_data.burst_delay + firing_mods.burst_delay_add) * firing_mods.burst_delay_scale;
            let shot_burst_duration = _weapon.firing_data.burst_duration * firing_mods.burst_duration_scale;
            let shot_burst_size = _weapon.firing_data.burst_size as f64 + firing_mods.burst_size_add;
            let shot_inner_burst_delay = shot_burst_duration / (shot_burst_size - 1.0);

            let shot_delay = if bdy_bullets_hit%shot_burst_size > 0.0 {
                shot_inner_burst_delay
            } else {
                shot_burst_delay
            };

            if bdy_bullets_hit%shot_burst_size == 0.0 {
                bdy_bullets_fired += 1.0;
                bdy_bullets_hit += 1.0;
            } else {
                bdy_bullets_hit += 1.0;
            };

            if (bdy_damage_dealt + body_damage) > health {
                break;
            } else {
                bdy_damage_dealt += body_damage;
            }
            bdy_time_taken += shot_delay;
        }
        let body_ttk = BodyKillData{
            time_taken: bdy_time_taken,
            bodyshots: bdy_bullets_hit as i32,
        };
        ttk_data.push(ResillienceSummary {value: i as i32, body_ttk, optimal_ttk });
    }
    ttk_data
}

impl Weapon{
    // fn get_explosive_amount(&self) -> f64{
    //     let mut explosive_amount = 0.0;
    //     for perk in self.list_perks(){
    //         if perk.hash == 
    //     }
    //     explosive_amount
    // }
    pub fn calc_ttk(&self, _overshield: f64) -> Vec<ResillienceSummary>{
        calc_ttk(self, _overshield, 0.0)
    }
}




