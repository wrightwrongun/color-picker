#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(dead_code, unused)]


use std::io;
use std::{fs::File, io::BufReader};

use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize};

use slint::{ComponentHandle, SharedString};

slint::include_modules!();

mod slintutils;


#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
struct ColorCode {
    name: String,
    hex: String,
    #[serde(deserialize_with="parse_rgb")]
    rgb: [u8; 3],
    families: Vec<String>
}

fn parse_rgb<'de, D>(deserializer: D) -> Result<[u8;3], D::Error>
where
    D: Deserializer<'de>
{
    let raw: String = Deserialize::deserialize(deserializer)?;
    let values = &raw[4..(raw.len() -1)].split(',').collect::<Vec<&str>>();

    let mut color: [u8; 3] = [0, 0, 0];
    color[0] = values[0].parse::<u8>().unwrap_or_default();
    color[1] = values[1].trim().parse::<u8>().unwrap_or_default();
    color[2] = values[2].trim().parse::<u8>().unwrap_or_default();

    Ok(color)
}

fn read_json_colors(path: &str) -> io::Result<Vec<ColorCode>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader)?)
}

fn main() {
    let path = "./data/colors.json";

    if let Ok(mut colors) = read_json_colors(path) {
        colors.sort();

        let mut families: Vec<String> = colors.iter()
                                                .map(|color| color.families.clone())
                                                .inspect(|a| println!("{:?}", a))
                                                .flatten()
                                                .unique()
                                                .collect();

        families.sort();

        let ui = AppWindow::new().unwrap();

        ui.on_get_colors(move || {
            slintutils::map_to_array(&colors, |color| {
                SharedColor {
                    name: SharedString::from(&color.name),
                    rgb: slintutils::wrap_array(&[color.rgb[0] as i32, color.rgb[1] as i32, color.rgb[2] as i32])
                }
            })
        });

        ui.on_get_families(move || {
            println!("get families (count={})", families.len());
            slintutils::map_to_array(&families, |s| SharedString::from(s))
        });

        ui.on_send_colors(move |colors| {
            let x = slintutils::map_from_array(colors, |color| String::from(color));
            
            for q in x.unwrap() {
                println!("color={:?}", q);
            }
        });

        // Have to create the slint 'families' combo-box dynamically so that
        // it can call the 'get-families' callback...
        ui.set_show_families(true);

        ui.run();
    }
    else {
        eprintln!("cannot read file '{}' - exiting", path);
    }
}
