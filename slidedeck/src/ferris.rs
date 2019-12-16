//! Makes an animated [ferris](https://www.rustacean.net/) slide deck using
//! [this animation](https://jsfiddle.net/Diggsey/3pdgh52r/embedded/result/) by
//! [Diggsey](https://github.com/Diggsey).

use super::common::make_image;
use lc3_isa::util::MemoryDump;

use std::concat;

/// Produces the image for the program + the data for this slide deck.
pub fn slide_deck() -> MemoryDump {
    make_image((WIDTH, HEIGHT), &SLIDES, false)
}

/// Height (in characters) of this slide deck.
pub const HEIGHT: usize = 11;

/// Width (in characters) of this slide deck.
pub const WIDTH: usize = 31;

/// Borrowed from [rustacean.net](rustacean.net); more specifically from [this
/// lovely `JSFiddle`](https://jsfiddle.net/Diggsey/3pdgh52r/embedded/result/)
/// by [Diggsey](https://github.com/Diggsey).
///
/// Non-ASCII characters were replaced (just `¬` -> `~`).
pub static SLIDES: [& str; 68] = [
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   ~   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#" '-,   -  _'\                  "#,
r#"  | '----'                     "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    .~'^'^-, (/                "#,
r#"\) /  o O  |'                  "#,
r#" '-,   -  _'\                  "#,
r#"  | '----'                     "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    .~'^'^-, (/                "#,
r#"\) /  o O  |'                  "#,
r#" '-,   -  _'\                  "#,
r#"  | '----'                     "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#" '-,   -  _'\                  "#,
r#"  | '----'                     "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"     _~^~^~_                   "#,
r#" \) /  o o  \ (/               "#,
r#"   '_   ~   _'                 "#,
r#"   / '-----' \                 "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"      _~^~^~_                  "#,
r#"  \) /  o o  \ (/              "#,
r#"    '_   ~   _'                "#,
r#"    | '-----' |                "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"       _~^~^~_                 "#,
r#"   \) /  o o  \ (/             "#,
r#"     '_   ~   _'               "#,
r#"     \ '-----' /               "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"        _~^~^~_                "#,
r#"    \) /  o o  \ (/            "#,
r#"      '_   ~   _'              "#,
r#"      | '-----' |              "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"         _~^~^~_               "#,
r#"     \) /  o o  \ (/           "#,
r#"       '_   ~   _'             "#,
r#"       / '-----' \             "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"          _~^~^~_              "#,
r#"      \) /  o o  \ (/          "#,
r#"        '_   ~   _'            "#,
r#"        | '-----' |            "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"           _~^~^~_             "#,
r#"       \) /  o o  \ (/         "#,
r#"         '_   ~   _'           "#,
r#"         \ '-----' /           "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"            _~^~^~_            "#,
r#"        \) /  o o  \ (/        "#,
r#"          '_   ~   _'          "#,
r#"          | '-----' |          "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"             _~^~^~_           "#,
r#"         \) /  o o  \ (/       "#,
r#"           '_   ~   _'         "#,
r#"           / '-----' \         "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"              _~^~^~_          "#,
r#"          \) /  o o  \ (/      "#,
r#"            '_   ~   _'        "#,
r#"            | '-----' |        "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"               _~^~^~_         "#,
r#"           \) /  o o  \ (/     "#,
r#"             '_   ~   _'       "#,
r#"             \ '-----' /       "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                _~^~^~_        "#,
r#"            \) /  o o  \ (/    "#,
r#"              '_   ~   _'      "#,
r#"              | '-----' |      "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                 _~^~^~_       "#,
r#"             \) /  o o  \ (/   "#,
r#"               '_   ~   _'     "#,
r#"               / '-----' \     "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                  _~^~^~_      "#,
r#"              \) /  o o  \ (/  "#,
r#"                '_   ~   _'    "#,
r#"                | '-----' |    "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   ~   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \/ /  o o  \ \/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \/ /  o o  \ \/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \/ /  o o  \ \/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   u   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   ~   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 '_   ~   _'   "#,
r#"                 \ '-----' /   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 /'_  -   ,-'  "#,
r#"                    '----' |   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"               \) ,-^'^'~.     "#,
r#"                 '|  O o  \ (/ "#,
r#"                 /'_  -   ,-'  "#,
r#"                    '----' |   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"               \) ,-^'^'~.     "#,
r#"                 '|  O o  \ (/ "#,
r#"                 /'_  -   ,-'  "#,
r#"                    '----' |   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                   _~^~^~_     "#,
r#"               \) /  o o  \ (/ "#,
r#"                 /'_  -   ,-'  "#,
r#"                    '----' |   "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                  _~^~^~_      "#,
r#"              \) /  o o  \ (/  "#,
r#"                '_   ~   _'    "#,
r#"                / '-----' \    "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                 _~^~^~_       "#,
r#"             \) /  o o  \ (/   "#,
r#"               '_   ~   _'     "#,
r#"               | '-----' |     "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"                _~^~^~_        "#,
r#"            \) /  o o  \ (/    "#,
r#"              '_   ~   _'      "#,
r#"              \ '-----' /      "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"               _~^~^~_         "#,
r#"           \) /  o o  \ (/     "#,
r#"             '_   ~   _'       "#,
r#"             | '-----' |       "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"              _~^~^~_          "#,
r#"          \) /  o o  \ (/      "#,
r#"            '_   ~   _'        "#,
r#"            / '-----' \        "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"             _~^~^~_           "#,
r#"         \) /  o o  \ (/       "#,
r#"           '_   ~   _'         "#,
r#"           | '-----' |         "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"            _~^~^~_            "#,
r#"        \) /  o o  \ (/        "#,
r#"          '_   ~   _'          "#,
r#"          \ '-----' /          "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"           _~^~^~_             "#,
r#"       \) /  o o  \ (/         "#,
r#"         '_   ~   _'           "#,
r#"         | '-----' |           "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"          _~^~^~_              "#,
r#"      \) /  o o  \ (/          "#,
r#"        '_   ~   _'            "#,
r#"        / '-----' \            "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"         _~^~^~_               "#,
r#"     \) /  o o  \ (/           "#,
r#"       '_   ~   _'             "#,
r#"       | '-----' |             "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"        _~^~^~_                "#,
r#"    \) /  o o  \ (/            "#,
r#"      '_   ~   _'              "#,
r#"      \ '-----' /              "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"       _~^~^~_                 "#,
r#"   \) /  o o  \ (/             "#,
r#"     '_   ~   _'               "#,
r#"     | '-----' |               "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"      _~^~^~_                  "#,
r#"  \) /  o o  \ (/              "#,
r#"    '_   ~   _'                "#,
r#"    / '-----' \                "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"     _~^~^~_                   "#,
r#" \) /  o o  \ (/               "#,
r#"   '_   ~   _'                 "#,
r#"   | '-----' |                 "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   ~   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\/ /  o o  \ \/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\/ /  o o  \ \/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\/ /  o o  \ \/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   u   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
concat!(
r#"                               "#,
r#"                               "#,
r#"                               "#,
r#"    _~^~^~_                    "#,
r#"\) /  o o  \ (/                "#,
r#"  '_   ~   _'                  "#,
r#"  \ '-----' /                  "#,
r#"                               "#,
r#"                               "#,
r#"                               "#,
),
];
