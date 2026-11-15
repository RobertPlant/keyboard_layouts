use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Size {
    #[serde(rename = "1u")]
    One,
    #[serde(rename = "1uh")]
    Homing,
    #[serde(rename = "1.5u")]
    OneAndHalf,
    #[serde(rename = "1.5uv")]
    OneAndHalfVertical,
    #[serde(rename = "2uv")]
    TwoVertical,
}

/// What's printed for layer 0: one legend, a UK shift pair, or nothing on a dead switch.
#[derive(Clone, Copy, Debug)]
pub enum Legend {
    Single(&'static str),
    Pair {
        base: &'static str,
        shifted: &'static str,
    },
    Dead,
}

/// Where YUZU's ErgoDox layout puts a key, ids and labels as YUZU exports them.
pub struct Position {
    pub id: &'static str,
    pub x: f64,
    pub y: f64,
    pub v_row: u8,
    pub size: Size,
    pub label_id: &'static str,
    pub row_number: u8,
}

pub struct Key {
    pub at: Position,
    pub legend: Legend,
    pub layer1: Option<&'static str>,
}

const fn at(
    id: &'static str,
    x: f64,
    y: f64,
    v_row: u8,
    size: Size,
    label_id: &'static str,
    row_number: u8,
) -> Position {
    Position {
        id,
        x,
        y,
        v_row,
        size,
        label_id,
        row_number,
    }
}

const fn pair(base: &'static str, shifted: &'static str) -> Legend {
    Legend::Pair { base, shifted }
}

// layer 0 and 1 of ../ergodox/keymap.c, placed by position: YUZU's own labels don't match the keymap
pub const KEYS: [Key; 76] = [
    Key {
        at: at("7P28FW", -3.0, -1.25, 1, Size::OneAndHalf, "space", 1),
        legend: Legend::Single("Esc"),
        layer1: None,
    },
    Key {
        at: at("GZQFPA", -1.5, -1.25, 1, Size::One, "1", 1),
        legend: pair("1", "!"),
        layer1: Some("F1"),
    },
    Key {
        at: at("FzWhED", -0.5, -1.25, 1, Size::One, "2", 1),
        legend: pair("2", "\""),
        layer1: Some("F2"),
    },
    Key {
        at: at("0R3YCk", 0.5, -1.25, 1, Size::One, "3", 1),
        legend: pair("3", "£"),
        layer1: Some("F3"),
    },
    Key {
        at: at("rW9ob7", 1.5, -1.25, 1, Size::One, "4", 1),
        legend: pair("4", "$"),
        layer1: Some("F4"),
    },
    Key {
        at: at("zYPGbQ", 2.5, -1.25, 1, Size::One, "5", 1),
        legend: pair("5", "%"),
        layer1: Some("F5"),
    },
    Key {
        at: at("A59P47", 3.5, -1.25, 1, Size::One, "space", 1),
        legend: Legend::Single("Del"),
        layer1: None,
    },
    Key {
        at: at("P56DXT", 6.0, -1.25, 1, Size::One, "space", 1),
        legend: Legend::Single("F5"),
        layer1: None,
    },
    Key {
        at: at("TIDz6n", 7.0, -1.25, 1, Size::One, "6", 1),
        legend: pair("6", "^"),
        layer1: Some("F6"),
    },
    Key {
        at: at("0C6IJF", 8.0, -1.25, 1, Size::One, "7", 1),
        legend: pair("7", "&"),
        layer1: Some("F7"),
    },
    Key {
        at: at("vTVl8h", 9.0, -1.25, 1, Size::One, "8", 1),
        legend: pair("8", "*"),
        layer1: Some("F8"),
    },
    Key {
        at: at("cNAoas", 10.0, -1.25, 1, Size::One, "9", 1),
        legend: pair("9", "("),
        layer1: Some("F9"),
    },
    Key {
        at: at("UVi9dK", 11.0, -1.25, 1, Size::One, "0", 1),
        legend: pair("0", ")"),
        layer1: Some("F10"),
    },
    Key {
        at: at("MKMFS4", 12.0, -1.25, 1, Size::OneAndHalf, "space", 1),
        legend: Legend::Single("Bksp"),
        layer1: Some("Del"),
    },
    Key {
        at: at("CEBPLX", -3.0, -0.25, 2, Size::OneAndHalf, "space", 2),
        legend: Legend::Single("Tab"),
        layer1: None,
    },
    Key {
        at: at("l7DqSU", -1.5, -0.25, 2, Size::One, "Q", 2),
        legend: Legend::Single("Q"),
        layer1: Some("!"),
    },
    Key {
        at: at("dfYXnc", -0.5, -0.25, 2, Size::One, "W", 2),
        legend: Legend::Single("W"),
        layer1: Some("@"),
    },
    Key {
        at: at("2bSRwq", 0.5, -0.25, 2, Size::One, "E", 2),
        legend: Legend::Single("E"),
        layer1: Some("{"),
    },
    Key {
        at: at("JaOAd8", 1.5, -0.25, 2, Size::One, "R", 2),
        legend: Legend::Single("R"),
        layer1: Some("}"),
    },
    Key {
        at: at("J9JHWP", 2.5, -0.25, 2, Size::One, "T", 2),
        legend: Legend::Single("T"),
        layer1: Some("|"),
    },
    Key {
        at: at(
            "4JFX3U",
            3.5,
            -0.25,
            2,
            Size::OneAndHalfVertical,
            "space",
            2,
        ),
        legend: pair("-", "_"),
        layer1: None,
    },
    Key {
        at: at(
            "GYPDRJ",
            6.0,
            -0.25,
            2,
            Size::OneAndHalfVertical,
            "space",
            2,
        ),
        legend: pair("#", "~"),
        layer1: None,
    },
    Key {
        at: at("ZBNA2M", 7.0, -0.25, 2, Size::One, "Y", 2),
        legend: Legend::Single("Y"),
        layer1: None,
    },
    Key {
        at: at("EyXwyF", 8.0, -0.25, 2, Size::One, "U", 2),
        legend: Legend::Single("U"),
        layer1: None,
    },
    Key {
        at: at("6Sh7Tl", 9.0, -0.25, 2, Size::One, "I", 2),
        legend: Legend::Single("I"),
        layer1: None,
    },
    Key {
        at: at("Y1R9rb", 10.0, -0.25, 2, Size::One, "O", 2),
        legend: Legend::Single("O"),
        layer1: None,
    },
    Key {
        at: at("1wB5ta", 11.0, -0.25, 2, Size::One, "P", 2),
        legend: Legend::Single("P"),
        layer1: Some("*"),
    },
    Key {
        at: at("T24GU9", 12.0, -0.25, 2, Size::OneAndHalf, "space", 2),
        legend: Legend::Dead,
        layer1: None,
    },
    Key {
        at: at("3R8UDS", -3.0, 0.75, 3, Size::OneAndHalf, "space", 3),
        legend: Legend::Single("Super"),
        layer1: None,
    },
    Key {
        at: at("I9bsjW", -1.5, 0.75, 3, Size::One, "A", 3),
        legend: Legend::Single("A"),
        layer1: Some("#"),
    },
    Key {
        at: at("dXk9mu", -0.5, 0.75, 3, Size::One, "S", 3),
        legend: Legend::Single("S"),
        layer1: Some("$"),
    },
    Key {
        at: at("0XL1hR", 0.5, 0.75, 3, Size::One, "D", 3),
        legend: Legend::Single("D"),
        layer1: Some("("),
    },
    Key {
        at: at("Cg7Wc0", 1.5, 0.75, 3, Size::Homing, "F", 3),
        legend: Legend::Single("F"),
        layer1: Some(")"),
    },
    Key {
        at: at("SGQPN7", 2.5, 0.75, 3, Size::One, "G", 3),
        legend: Legend::Single("G"),
        layer1: Some("`"),
    },
    Key {
        at: at("8EYLJ4", 7.0, 0.75, 3, Size::One, "H", 3),
        legend: Legend::Single("H"),
        layer1: Some("←"),
    },
    Key {
        at: at("DmcEQn", 8.0, 0.75, 3, Size::Homing, "J", 3),
        legend: Legend::Single("J"),
        layer1: Some("↓"),
    },
    Key {
        at: at("7tj5BS", 9.0, 0.75, 3, Size::One, "K", 3),
        legend: Legend::Single("K"),
        layer1: Some("↑"),
    },
    Key {
        at: at("cGx7Jw", 10.0, 0.75, 3, Size::One, "L", 3),
        legend: Legend::Single("L"),
        layer1: Some("→"),
    },
    Key {
        at: at("emeZD1", 11.0, 0.75, 3, Size::One, "semicolon", 3),
        legend: pair(";", ":"),
        layer1: Some("+"),
    },
    Key {
        at: at("QEKHQW", 12.0, 0.75, 3, Size::OneAndHalf, "space", 3),
        legend: pair("'", "@"),
        layer1: None,
    },
    Key {
        at: at("AMR4XD", 3.5, 1.25, 3, Size::OneAndHalfVertical, "space", 3),
        legend: pair("=", "+"),
        layer1: None,
    },
    Key {
        at: at("ZRHML7", 6.0, 1.25, 3, Size::OneAndHalfVertical, "space", 3),
        legend: pair(";", ":"),
        layer1: None,
    },
    Key {
        at: at("3UK6NS", -3.0, 1.75, 4, Size::OneAndHalf, "space", 4),
        legend: Legend::Single("Shift"),
        layer1: None,
    },
    Key {
        at: at("h9SSVX", -1.5, 1.75, 4, Size::One, "Z", 4),
        legend: pair("\\", "|"),
        layer1: Some("%"),
    },
    Key {
        at: at("s6w7jw", -0.5, 1.75, 4, Size::One, "X", 4),
        legend: Legend::Single("Z"),
        layer1: Some("^"),
    },
    Key {
        at: at("UAeurI", 0.5, 1.75, 4, Size::One, "C", 4),
        legend: Legend::Single("X"),
        layer1: Some("["),
    },
    Key {
        at: at("NGDt4W", 1.5, 1.75, 4, Size::One, "V", 4),
        legend: Legend::Single("C"),
        layer1: Some("]"),
    },
    Key {
        at: at("M8QS7R", 2.5, 1.75, 4, Size::One, "B", 4),
        legend: Legend::Single("V"),
        layer1: Some("~"),
    },
    Key {
        at: at("BFSHE3", 7.0, 1.75, 4, Size::One, "N", 4),
        legend: Legend::Single("B"),
        layer1: Some("&"),
    },
    Key {
        at: at("Uzb6qW", 8.0, 1.75, 4, Size::One, "M", 4),
        legend: Legend::Single("N"),
        layer1: None,
    },
    Key {
        at: at("q4tLCR", 9.0, 1.75, 4, Size::One, "comma", 4),
        legend: Legend::Single("M"),
        layer1: None,
    },
    Key {
        at: at("CRIsLR", 10.0, 1.75, 4, Size::One, "dot", 4),
        legend: pair(",", "<"),
        layer1: None,
    },
    Key {
        at: at("KbSM4v", 11.0, 1.75, 4, Size::One, "slash", 4),
        legend: pair(".", ">"),
        layer1: Some("\\"),
    },
    Key {
        at: at("EGDHBG", 12.0, 1.75, 4, Size::OneAndHalf, "space", 4),
        legend: pair("/", "?"),
        layer1: None,
    },
    Key {
        at: at("f19Yvj", -2.5, 2.75, 5, Size::One, "tilde", 4),
        legend: Legend::Single("Ctrl"),
        layer1: None,
    },
    Key {
        at: at("kOxNNJ", -1.5, 2.75, 5, Size::One, "apostrophe", 4),
        legend: Legend::Single("L3"),
        layer1: Some("="),
    },
    Key {
        at: at("fzsvxA", -0.5, 2.75, 5, Size::One, "space", 4),
        legend: Legend::Single("L2"),
        layer1: None,
    },
    Key {
        at: at("dqz1KA", 0.5, 2.75, 5, Size::One, "arrow-left", 4),
        legend: Legend::Single("L1"),
        layer1: None,
    },
    Key {
        at: at("K84UUS", 1.5, 2.75, 4, Size::One, "arrow-right", 4),
        legend: Legend::Single("Alt"),
        layer1: None,
    },
    Key {
        at: at("R4QLAP", 8.0, 2.75, 4, Size::One, "arrow-up", 4),
        legend: Legend::Single("L2"),
        layer1: None,
    },
    Key {
        at: at("SkCBkD", 9.0, 2.75, 5, Size::One, "arrow-down", 4),
        legend: Legend::Single("_"),
        layer1: None,
    },
    Key {
        at: at("kCDQMH", 10.0, 2.75, 5, Size::One, "bracket-right", 4),
        legend: Legend::Single("-"),
        layer1: None,
    },
    Key {
        at: at("Y2XSTU", 11.0, 2.75, 5, Size::One, "bracket-left", 4),
        legend: pair("=", "+"),
        layer1: Some("F11"),
    },
    Key {
        at: at("RTHPEX", 12.0, 2.75, 4, Size::One, "space", 4),
        legend: Legend::Single("Ctrl"),
        layer1: Some("F12"),
    },
    Key {
        at: at("GGCFEK", 3.0, 3.25, 4, Size::One, "space", 4),
        legend: Legend::Dead,
        layer1: None,
    },
    Key {
        at: at("LFQR52", 4.0, 3.25, 4, Size::One, "space", 4),
        legend: Legend::Single("^A"),
        layer1: None,
    },
    Key {
        at: at("XACG6H", 4.0, 4.25, 4, Size::One, "space", 4),
        legend: Legend::Single("Home"),
        layer1: None,
    },
    Key {
        at: at("TLB9ES", 4.0, 5.25, 4, Size::One, "space", 4),
        legend: Legend::Single("End"),
        layer1: None,
    },
    Key {
        at: at("54TSLP", 2.0, 4.25, 4, Size::TwoVertical, "space", 4),
        legend: Legend::Single("Space"),
        layer1: None,
    },
    Key {
        at: at("CD9XSL", 3.0, 4.25, 4, Size::TwoVertical, "space", 4),
        legend: Legend::Single("Bksp"),
        layer1: Some("Del"),
    },
    Key {
        at: at("4GSW3A", 5.5, 3.25, 4, Size::One, "space", 4),
        legend: Legend::Single("Play"),
        layer1: Some("Vol-"),
    },
    Key {
        at: at("ZK573M", 6.5, 3.25, 4, Size::One, "space", 4),
        legend: Legend::Single("Next"),
        layer1: Some("Vol+"),
    },
    Key {
        at: at("LLAJET", 5.5, 4.25, 4, Size::One, "space", 4),
        legend: Legend::Single("PgUp"),
        layer1: Some("Mute"),
    },
    Key {
        at: at("SQPXD7", 5.5, 5.25, 4, Size::One, "space", 4),
        legend: Legend::Single("PgDn"),
        layer1: None,
    },
    Key {
        at: at("Y53RLB", 6.5, 4.25, 4, Size::TwoVertical, "space", 4),
        legend: Legend::Single("Tab"),
        layer1: Some("Ins"),
    },
    Key {
        at: at("H2CWXF", 7.5, 4.25, 4, Size::TwoVertical, "space", 4),
        legend: Legend::Single("Enter"),
        layer1: None,
    },
];

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
mod tests {
    use super::*;

    fn count_size(size: Size) -> usize {
        KEYS.iter().filter(|key| key.at.size == size).count()
    }

    #[test]
    fn test_has_twelve_one_and_half_keys() {
        assert_eq!(
            count_size(Size::OneAndHalf) + count_size(Size::OneAndHalfVertical),
            12
        );
    }

    #[test]
    fn test_has_four_two_unit_thumb_keys() {
        assert_eq!(count_size(Size::TwoVertical), 4);
    }

    #[test]
    fn test_ids_are_unique() {
        let ids: std::collections::BTreeSet<&str> = KEYS.iter().map(|key| key.at.id).collect();
        assert_eq!(ids.len(), KEYS.len());
    }
}
