use std::collections::BTreeMap;

use serde::Serialize;

use crate::keys::{Key, Legend, Size, KEYS};

/// YUZU colour ids, named for how they render.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Colour {
    #[serde(rename = "PL001")]
    Lavender,
    #[serde(rename = "PL002")]
    Violet,
    #[serde(rename = "PL003")]
    PaleLavender,
    #[serde(rename = "PL004")]
    NearBlack,
    #[serde(rename = "OG006")]
    Peach,
    #[serde(rename = "BL008")]
    LightBlue,
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum Lock {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    x: f64,
    y: f64,
    id: &'static str,
    lock_x: Lock,
    color_id: Colour,
    text_size: f64,
    text_label: &'static str,
    text_font_id: u8,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    id: &'static str,
    vect_id: Option<u32>,
    color_id: Colour,
    vect_def: &'static str,
    vect_scale: f64,
    vect_viewbox: &'static str,
    vect_collection_id: Option<&'static str>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Layer {
    Text(Text),
    Icon(Icon),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyDesign {
    color_id: Colour,
    layers: Vec<Layer>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutKey {
    x: f64,
    y: f64,
    id: &'static str,
    v_row: u8,
    size_id: Size,
    label_id: &'static str,
    row_number: u8,
}

/// The import format of YUZU's designer, as its export writes it.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Design {
    keycap_profile_id: &'static str,
    print_type: &'static str,
    keyboard: &'static str,
    language: Option<&'static str>,
    name: &'static str,
    author: Option<&'static str>,
    mods: Vec<String>,
    extensions: Vec<String>,
    layers: Vec<String>,
    layout_keys: Vec<LayoutKey>,
    keys: BTreeMap<&'static str, KeyDesign>,
    key_templates: BTreeMap<&'static str, &'static str>,
    design_url: &'static str,
}

struct Ink {
    main: Colour,
    sub: Colour,
}

const ACCENT_IDS: [&str; 6] = ["7P28FW", "H2CWXF", "kOxNNJ", "fzsvxA", "dqz1KA", "R4QLAP"];

// layer 2 numpad
const NUMPAD: [(&str, &str); 10] = [
    ("EyXwyF", "7"),
    ("6Sh7Tl", "8"),
    ("Y1R9rb", "9"),
    ("DmcEQn", "4"),
    ("7tj5BS", "5"),
    ("cGx7Jw", "6"),
    ("Uzb6qW", "1"),
    ("q4tLCR", "2"),
    ("CRIsLR", "3"),
    ("R4QLAP", "0"),
];

const ICONS: [(&str, &str); 2] = [
    (
        "4GSW3A",
        "M100 150L500 500L100 850Z M600 150H720V850H600Z M800 150H920V850H800Z",
    ),
    (
        "ZK573M",
        "M80 150L450 500L80 850Z M450 150L820 500L450 850Z M840 150H940V850H840Z",
    ),
];

const WORD_X: f64 = 9.0;
const WORD_SIZE: f64 = 3.0;

#[must_use]
pub fn build_design() -> Design {
    Design {
        keycap_profile_id: "kam",
        print_type: "dyesub",
        keyboard: "ergodox",
        language: None,
        name: "ErgoDox UK purple",
        author: None,
        mods: Vec::new(),
        extensions: Vec::new(),
        layers: Vec::new(),
        layout_keys: KEYS.iter().map(build_layout_key).collect(),
        keys: KEYS
            .iter()
            .map(|key| (key.at.id, build_key_design(key)))
            .collect(),
        key_templates: KEYS
            .iter()
            .map(|key| (key.at.id, get_template(key.legend)))
            .collect(),
        design_url: "kam-default",
    }
}

fn build_layout_key(key: &Key) -> LayoutKey {
    LayoutKey {
        x: key.at.x,
        y: key.at.y,
        id: key.at.id,
        v_row: key.at.v_row,
        size_id: key.at.size,
        label_id: key.at.label_id,
        row_number: key.at.row_number,
    }
}

fn build_key_design(key: &Key) -> KeyDesign {
    let cap = get_cap_colour(key);

    KeyDesign {
        color_id: cap,
        layers: build_layers(key, &get_ink(cap)),
    }
}

fn get_template(legend: Legend) -> &'static str {
    match legend {
        Legend::Pair { .. } => "VLMW",
        Legend::Single(_) | Legend::Dead => "cPpy",
    }
}

/// Lavender, violet, then near-black going down the board; accents in peach.
#[must_use]
fn get_cap_colour(key: &Key) -> Colour {
    let is_accent = ACCENT_IDS.contains(&key.at.id);
    if is_accent {
        return Colour::Peach;
    }
    let is_thumb = key.at.y > 3.0;
    if is_thumb {
        return Colour::NearBlack;
    }

    match key.at.v_row {
        1 => Colour::Lavender,
        2 | 3 => Colour::Violet,
        _ => Colour::NearBlack,
    }
}

/// Dark legends on the light caps, peach and lavender on the dark ones.
#[must_use]
fn get_ink(cap: Colour) -> Ink {
    match cap {
        Colour::Violet | Colour::NearBlack => Ink {
            main: Colour::Peach,
            sub: Colour::PaleLavender,
        },
        Colour::Lavender | Colour::Peach | Colour::PaleLavender | Colour::LightBlue => Ink {
            main: Colour::NearBlack,
            sub: Colour::NearBlack,
        },
    }
}

fn build_layers(key: &Key, ink: &Ink) -> Vec<Layer> {
    let layer1 = key.layer1.unwrap_or("");
    let numpad = find_value(&NUMPAD, key.at.id).unwrap_or("");

    match key.legend {
        Legend::Pair { base, shifted } => vec![
            text("lJ31", 6.7, 2.8, Lock::Left, ink.main, 4.6, base),
            text("UIl3", 6.7, -2.4, Lock::Left, ink.main, 4.6, shifted),
            text(
                "XRRT",
                -6.7,
                2.8,
                Lock::Right,
                ink.sub,
                get_sub_size(layer1),
                layer1,
            ),
            text(
                "Y2V4",
                -6.7,
                -2.4,
                Lock::Right,
                Colour::LightBlue,
                3.6,
                numpad,
            ),
        ],
        Legend::Single(label) => match find_value(&ICONS, key.at.id) {
            Some(path) => vec![build_icon(path, ink.main), build_sub(layer1, ink)],
            None => build_single(label, layer1, numpad, ink),
        },
        Legend::Dead => build_single("RIP", layer1, numpad, ink),
    }
}

fn build_single(
    label: &'static str,
    layer1: &'static str,
    numpad: &'static str,
    ink: &Ink,
) -> Vec<Layer> {
    let length = label.chars().count();
    let is_function_key = label.starts_with('F') && length > 1;
    let is_word = length > 1 && !is_function_key;
    let x = if is_word { WORD_X } else { 7.0 };
    let size = get_main_size(is_word, is_function_key);

    vec![
        text("uZbA", x, -2.1, Lock::Left, ink.main, size, label),
        text("CGCC", -7.0, -2.1, Lock::Right, ink.main, size, ""),
        build_sub(layer1, ink),
        text("XLTW", 7.0, 3.0, Lock::Left, Colour::LightBlue, 3.6, numpad),
    ]
}

fn build_sub(layer1: &'static str, ink: &Ink) -> Layer {
    text(
        "TW2F",
        -7.0,
        3.0,
        Lock::Right,
        ink.sub,
        get_sub_size(layer1),
        layer1,
    )
}

#[must_use]
fn get_main_size(is_word: bool, is_function_key: bool) -> f64 {
    if is_word {
        return WORD_SIZE;
    }
    if is_function_key {
        return 4.6;
    }

    5.8
}

/// Long layer legends get cropped at the full size.
#[must_use]
fn get_sub_size(layer1: &str) -> f64 {
    let is_long = layer1.chars().count() > 2;
    if is_long {
        return 2.8;
    }

    3.6
}

fn find_value(table: &[(&'static str, &'static str)], id: &str) -> Option<&'static str> {
    table
        .iter()
        .find(|(key_id, _)| *key_id == id)
        .map(|(_, value)| *value)
}

fn text(
    id: &'static str,
    x: f64,
    y: f64,
    lock_x: Lock,
    color_id: Colour,
    text_size: f64,
    text_label: &'static str,
) -> Layer {
    Layer::Text(Text {
        x,
        y,
        id,
        lock_x,
        color_id,
        text_size,
        text_label,
        text_font_id: 26,
    })
}

fn build_icon(path: &'static str, colour: Colour) -> Layer {
    Layer::Icon(Icon {
        id: "zaxy",
        vect_id: None,
        color_id: colour,
        vect_def: path,
        vect_scale: 20.0,
        vect_viewbox: "0 0 1000 1000",
        vect_collection_id: None,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
mod tests {
    use super::*;

    fn find_key(id: &str) -> &'static Key {
        KEYS.iter().find(|key| key.at.id == id).unwrap()
    }

    #[test]
    fn test_layer_keys_are_peach() {
        assert_eq!(get_cap_colour(find_key("dqz1KA")), Colour::Peach);
    }

    #[test]
    fn test_number_row_is_lavender() {
        assert_eq!(get_cap_colour(find_key("GZQFPA")), Colour::Lavender);
    }

    #[test]
    fn test_thumbs_are_near_black() {
        assert_eq!(get_cap_colour(find_key("54TSLP")), Colour::NearBlack);
    }

    #[test]
    fn test_light_caps_get_dark_legends() {
        assert_eq!(get_ink(Colour::Lavender).main, Colour::NearBlack);
    }

    #[test]
    fn test_numpad_seven_is_on_u() {
        assert_eq!(find_value(&NUMPAD, "EyXwyF"), Some("7"));
    }

    #[test]
    fn test_long_layer_legends_are_smaller() {
        assert_eq!(get_sub_size("Vol+"), 2.8);
    }
}
