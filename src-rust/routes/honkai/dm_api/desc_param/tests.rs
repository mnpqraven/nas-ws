
use crate::routes::honkai::dm_api::types::Param;

use super::get_sorted_params;

#[test]
fn as_func() {
    let desc = "Increases the wearer's Effect Hit Rate by <color=#f29e38ff><unbreak>#1[i]%</unbreak></color>. When the wearer deals DMG to an enemy that currently has <unbreak>#4[i]</unbreak> or more debuffs, increases the wearer's CRIT Rate by <color=#f29e38ff><unbreak>#5[i]%</unbreak></color>. After the wearer uses their Basic ATK, Skill, or Ultimate, there is a <unbreak>#2[i]%</unbreak> base chance to implant Aether Code on a random hit target that does not yet have it. Targets with Aether Code receive <color=#f29e38ff><unbreak>#3[i]%</unbreak></color> increased DMG for 1 turn.";
    // 1 4 5 2 3
    let param_list: Vec<Param> = vec![
        Param {
            value: 0.24000000022351742,
        },
        Param { value: 1.0 },
        Param {
            value: 0.12000000011175871,
        },
        Param { value: 3.0 },
        Param {
            value: 0.12000000011175871,
        },
    ];

    let t = get_sorted_params(param_list.iter().map(|param| param.value).collect(), desc);
    let right = vec![
        (0.24000000022351742, true),
        (3.0, false),
        (0.12000000011175871, true),
        (1.0, true),
        (0.12000000011175871, true),
    ];
    let left: Vec<(f64, bool)> = t.iter().map(|para| (para.0.0, para.0.1)).collect();
    assert_eq!(left, right);
}

