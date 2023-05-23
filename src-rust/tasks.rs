use nas_ws::routes::honkai::types::SurveyRate;
use std::{error::Error, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    // copy assets file to tmp
    Command::new("cp")
        .args(vec!["-r", "assets", "/tmp"])
        .output()?;

    // NOTE: formula
    // chartRate = percentRemaining * actualRate
    // -> actualRate = chartRate / percentRemaining
    let rates = SurveyRate::default();
    let mut pity_percent_remaining: f32 = 0.0;
    println!("calculating actual rates");
    for pull in rates.0.iter() {
        let percent_remaining: f32 = 0.994_f32.powi((pull.draw_number - 1).try_into()?);
        let actual_rate: f32 = pull.rate / percent_remaining;
        if pull.draw_number <= 75 {
            println!("pull {}: {} %", &pull.draw_number, &actual_rate);
        }

        // soft pity
        if pull.draw_number >= 76 {
            let last = (1.0_f32 - pity_percent_remaining).powi((pull.draw_number - 76).try_into()?);
            let percent_remaining: f32 = 0.994_f32.powi(75) * last;
            // only mutates once

            let actual_rate: f32 = pull.rate / percent_remaining;
            println!("pull {}: {} %", &pull.draw_number, &actual_rate);

            // mutates last
            pity_percent_remaining = actual_rate / 100.0;
        }
    }

    Ok(())
}
