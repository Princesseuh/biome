//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::all)]
#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum CssSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file.May have trivia attached"]
    EOF,
    SEMICOLON,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    TILDE,
    HASH,
    AMP,
    PIPE,
    PLUS,
    STAR,
    SLASH,
    CARET,
    PERCENT,
    DOT,
    COLON,
    COLON2,
    EQ,
    BANG,
    NEQ,
    MINUS,
    LTEQ,
    GTEQ,
    PLUSEQ,
    PIPEEQ,
    AMPEQ,
    CARETEQ,
    SLASHEQ,
    STAREQ,
    PERCENTEQ,
    AT,
    DOLLAR_EQ,
    TILDE_EQ,
    CDC,
    CDO,
    ALICEBLUE_KW,
    ANTIQUEWHITE_KW,
    AQUA_KW,
    AQUAMARINE_KW,
    AZURE_KW,
    BEIGE_KW,
    BISQUE_KW,
    BLACK_KW,
    BLANCHEDALMOND_KW,
    BLUE_KW,
    BLUEVIOLET_KW,
    BROWN_KW,
    BURLYWOOD_KW,
    CADETBLUE_KW,
    CHARTREUSE_KW,
    CHOCOLATE_KW,
    CORAL_KW,
    CORNFLOWERBLUE_KW,
    CORNSILK_KW,
    CRIMSON_KW,
    CYAN_KW,
    DARKBLUE_KW,
    DARKCYAN_KW,
    DARKGOLDENROD_KW,
    DARKGRAY_KW,
    DARKGREEN_KW,
    DARKKHAKI_KW,
    DARKMAGENTA_KW,
    DARKOLIVEGREEN_KW,
    DARKORANGE_KW,
    DARKORCHID_KW,
    DARKRED_KW,
    DARKSALMON_KW,
    DARKSEAGREEN_KW,
    DARKSLATEBLUE_KW,
    DARKSLATEGRAY_KW,
    DARKTURQUOISE_KW,
    DARKVIOLET_KW,
    DEEPPINK_KW,
    DEEPSKYBLUE_KW,
    DIMGRAY_KW,
    DODGERBLUE_KW,
    FIREBRICK_KW,
    FLORALWHITE_KW,
    FORESTGREEN_KW,
    FUCHSIA_KW,
    GAINSBORO_KW,
    GHOSTWHITE_KW,
    GOLD_KW,
    GOLDENROD_KW,
    GRAY_KW,
    GREEN_KW,
    GREENYELLOW_KW,
    HONEYDEW_KW,
    HOTPINK_KW,
    INDIANRED_KW,
    INDIGO_KW,
    IVORY_KW,
    KHAKI_KW,
    LAVENDER_KW,
    LAVENDERBLUSH_KW,
    LAWNGREEN_KW,
    LEMONCHIFFON_KW,
    LIGHTBLUE_KW,
    LIGHTCORAL_KW,
    LIGHTCYAN_KW,
    LIGHTGOLDENRODYELLOW_KW,
    LIGHTGREEN_KW,
    LIGHTGREY_KW,
    LIGHTPINK_KW,
    LIGHTSALMON_KW,
    LIGHTSEAGREEN_KW,
    LIGHTSKYBLUE_KW,
    LIGHTSLATEGRAY_KW,
    LIGHTSTEELBLUE_KW,
    LIGHTYELLOW_KW,
    LIME_KW,
    LIMEGREEN_KW,
    LINEN_KW,
    MAGENTA_KW,
    MAROON_KW,
    MEDIUMAQUAMARINE_KW,
    MEDIUMBLUE_KW,
    MEDIUMORCHID_KW,
    MEDIUMPURPLE_KW,
    MEDIUMSEAGREEN_KW,
    MEDIUMSLATEBLUE_KW,
    MEDIUMSPRINGGREEN_KW,
    MEDIUMTURQUOISE_KW,
    MEDIUMVIOLETRED_KW,
    MIDNIGHTBLUE_KW,
    MINTCREAM_KW,
    MISTYROSE_KW,
    MOCCASIN_KW,
    NAVAJOWHITE_KW,
    NAVY_KW,
    NAVYBLUE_KW,
    OLDLACE_KW,
    OLIVE_KW,
    OLIVEDRAB_KW,
    ORANGE_KW,
    ORANGERED_KW,
    ORCHID_KW,
    PALEGOLDENROD_KW,
    PALEGREEN_KW,
    PALETURQUOISE_KW,
    PALEVIOLETRED_KW,
    PAPAYAWHIP_KW,
    PEACHPUFF_KW,
    PERU_KW,
    PINK_KW,
    PLUM_KW,
    POWDERBLUE_KW,
    PURPLE_KW,
    RED_KW,
    ROSYBROWN_KW,
    ROYALBLUE_KW,
    SADDLEBROWN_KW,
    SALMON_KW,
    SANDYBROWN_KW,
    SEAGREEN_KW,
    SEASHELL_KW,
    SIENNA_KW,
    SILVER_KW,
    SKYBLUE_KW,
    SLATEBLUE_KW,
    SLATEGRAY_KW,
    SNOW_KW,
    SPRINGGREEN_KW,
    STEELBLUE_KW,
    TAN_KW,
    TEAL_KW,
    THISTLE_KW,
    TOMATO_KW,
    TURQUOISE_KW,
    VIOLET_KW,
    WHEAT_KW,
    WHITE_KW,
    WHITESMOKE_KW,
    YELLOW_KW,
    YELLOWGREEN_KW,
    MEDIA_KW,
    KEYFRAMES_KW,
    NOT_KW,
    AND_KW,
    ONLY_KW,
    OR_KW,
    I_KW,
    IMPORTANT_KW,
    FROM_KW,
    TO_KW,
    VAR_KW,
    CSS_STRING_LITERAL,
    CSS_NUMBER_LITERAL,
    CSS_CUSTOM_PROPERTY,
    CSS_SPACE_LITERAL,
    ERROR_TOKEN,
    IDENT,
    NEWLINE,
    WHITESPACE,
    COMMENT,
    MULTILINE_COMMENT,
    CSS_ROOT,
    CSS_RULE_LIST,
    CSS_RULE,
    CSS_SELECTOR_LIST,
    CSS_ANY_FUNCTION,
    CSS_AT_KEYFRAMES,
    CSS_AT_KEYFRAMES_BODY,
    CSS_AT_MEDIA,
    CSS_AT_MEDIA_QUERY,
    CSS_AT_MEDIA_QUERY_CONSEQUENT,
    CSS_AT_MEDIA_QUERY_FEATURE,
    CSS_AT_MEDIA_QUERY_FEATURE_BOOLEAN,
    CSS_AT_MEDIA_QUERY_FEATURE_COMPARE,
    CSS_AT_MEDIA_QUERY_FEATURE_PLAIN,
    CSS_AT_MEDIA_QUERY_FEATURE_RANGE,
    CSS_AT_MEDIA_QUERY_RANGE,
    CSS_BLOCK,
    CSS_DECLARATION,
    CSS_DIMENSION,
    CSS_IDENTIFIER,
    CSS_KEYFRAMES_BLOCK,
    CSS_KEYFRAMES_SELECTOR,
    CSS_NUMBER,
    CSS_PARAMETER,
    CSS_PERCENTAGE,
    CSS_RATIO,
    CSS_SIMPLE_FUNCTION,
    CSS_STRING,
    CSS_VAR_FUNCTION,
    CSS_VAR_FUNCTION_VALUE,
    CSS_AT_KEYFRAMES_ITEM_LIST,
    CSS_AT_MEDIA_QUERY_LIST,
    CSS_ATTRIBUTE_LIST,
    CSS_DECLARATION_LIST,
    CSS_KEYFRAMES_SELECTOR_LIST,
    CSS_PARAMETER_LIST,
    CSS_DECLARATION_IMPORTANT,
    CSS_ANY_SELECTOR_LIST,
    CSS_COMPLEX_SELECTOR,
    CSS_COMPLEX_SELECTOR_COMBINATOR,
    CSS_COMPOUND_SELECTOR,
    CSS_SUB_SELECTOR_LIST,
    CSS_ID_SELECTOR,
    CSS_CLASS_SELECTOR,
    CSS_TYPE_SELECTOR,
    CSS_UNIVERSAL_SELECTOR,
    CSS_PSEUDO_CLASS_SELECTOR,
    CSS_PSEUDO_ELEMENT_SELECTOR,
    CSS_PSEUDO_CLASS_SELECTOR_PARAMETERS,
    CSS_ATTRIBUTE_SELECTOR,
    CSS_ATTRIBUTE,
    CSS_ATTRIBUTE_MATCHER,
    CSS_ATTRIBUTE_META,
    CSS_ATTRIBUTE_MODIFIER,
    CSS_ATTRIBUTE_NAME,
    CSS_BOGUS,
    CSS_BOGUS_BODY,
    CSS_BOGUS_RULE,
    CSS_BOGUS_SELECTOR,
    CSS_BOGUS_SUB_SELECTOR,
    #[doc(hidden)]
    __LAST,
}
use self::CssSyntaxKind::*;
impl CssSyntaxKind {
    pub const fn is_punct(self) -> bool {
        match self {
            SEMICOLON | COMMA | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK
            | L_ANGLE | R_ANGLE | TILDE | HASH | AMP | PIPE | PLUS | STAR | SLASH | CARET
            | PERCENT | DOT | COLON | COLON2 | EQ | BANG | NEQ | MINUS | LTEQ | GTEQ | PLUSEQ
            | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ | PERCENTEQ | AT | DOLLAR_EQ
            | TILDE_EQ | CDC | CDO => true,
            _ => false,
        }
    }
    pub const fn is_literal(self) -> bool {
        match self {
            CSS_STRING_LITERAL | CSS_NUMBER_LITERAL | CSS_CUSTOM_PROPERTY | CSS_SPACE_LITERAL => {
                true
            }
            _ => false,
        }
    }
    pub const fn is_list(self) -> bool {
        match self {
            CSS_RULE_LIST
            | CSS_SELECTOR_LIST
            | CSS_AT_KEYFRAMES_ITEM_LIST
            | CSS_AT_MEDIA_QUERY_LIST
            | CSS_ATTRIBUTE_LIST
            | CSS_DECLARATION_LIST
            | CSS_KEYFRAMES_SELECTOR_LIST
            | CSS_PARAMETER_LIST
            | CSS_ANY_SELECTOR_LIST
            | CSS_SUB_SELECTOR_LIST => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<CssSyntaxKind> {
        let kw = match ident {
            "aliceblue" => ALICEBLUE_KW,
            "antiquewhite" => ANTIQUEWHITE_KW,
            "aqua" => AQUA_KW,
            "aquamarine" => AQUAMARINE_KW,
            "azure" => AZURE_KW,
            "beige" => BEIGE_KW,
            "bisque" => BISQUE_KW,
            "black" => BLACK_KW,
            "blanchedalmond" => BLANCHEDALMOND_KW,
            "blue" => BLUE_KW,
            "blueviolet" => BLUEVIOLET_KW,
            "brown" => BROWN_KW,
            "burlywood" => BURLYWOOD_KW,
            "cadetblue" => CADETBLUE_KW,
            "chartreuse" => CHARTREUSE_KW,
            "chocolate" => CHOCOLATE_KW,
            "coral" => CORAL_KW,
            "cornflowerblue" => CORNFLOWERBLUE_KW,
            "cornsilk" => CORNSILK_KW,
            "crimson" => CRIMSON_KW,
            "cyan" => CYAN_KW,
            "darkblue" => DARKBLUE_KW,
            "darkcyan" => DARKCYAN_KW,
            "darkgoldenrod" => DARKGOLDENROD_KW,
            "darkgray" => DARKGRAY_KW,
            "darkgreen" => DARKGREEN_KW,
            "darkkhaki" => DARKKHAKI_KW,
            "darkmagenta" => DARKMAGENTA_KW,
            "darkolivegreen" => DARKOLIVEGREEN_KW,
            "darkorange" => DARKORANGE_KW,
            "darkorchid" => DARKORCHID_KW,
            "darkred" => DARKRED_KW,
            "darksalmon" => DARKSALMON_KW,
            "darkseagreen" => DARKSEAGREEN_KW,
            "darkslateblue" => DARKSLATEBLUE_KW,
            "darkslategray" => DARKSLATEGRAY_KW,
            "darkturquoise" => DARKTURQUOISE_KW,
            "darkviolet" => DARKVIOLET_KW,
            "deeppink" => DEEPPINK_KW,
            "deepskyblue" => DEEPSKYBLUE_KW,
            "dimgray" => DIMGRAY_KW,
            "dodgerblue" => DODGERBLUE_KW,
            "firebrick" => FIREBRICK_KW,
            "floralwhite" => FLORALWHITE_KW,
            "forestgreen" => FORESTGREEN_KW,
            "fuchsia" => FUCHSIA_KW,
            "gainsboro" => GAINSBORO_KW,
            "ghostwhite" => GHOSTWHITE_KW,
            "gold" => GOLD_KW,
            "goldenrod" => GOLDENROD_KW,
            "gray" => GRAY_KW,
            "green" => GREEN_KW,
            "greenyellow" => GREENYELLOW_KW,
            "honeydew" => HONEYDEW_KW,
            "hotpink" => HOTPINK_KW,
            "indianred" => INDIANRED_KW,
            "indigo" => INDIGO_KW,
            "ivory" => IVORY_KW,
            "khaki" => KHAKI_KW,
            "lavender" => LAVENDER_KW,
            "lavenderblush" => LAVENDERBLUSH_KW,
            "lawngreen" => LAWNGREEN_KW,
            "lemonchiffon" => LEMONCHIFFON_KW,
            "lightblue" => LIGHTBLUE_KW,
            "lightcoral" => LIGHTCORAL_KW,
            "lightcyan" => LIGHTCYAN_KW,
            "lightgoldenrodyellow" => LIGHTGOLDENRODYELLOW_KW,
            "lightgreen" => LIGHTGREEN_KW,
            "lightgrey" => LIGHTGREY_KW,
            "lightpink" => LIGHTPINK_KW,
            "lightsalmon" => LIGHTSALMON_KW,
            "lightseagreen" => LIGHTSEAGREEN_KW,
            "lightskyblue" => LIGHTSKYBLUE_KW,
            "lightslategray" => LIGHTSLATEGRAY_KW,
            "lightsteelblue" => LIGHTSTEELBLUE_KW,
            "lightyellow" => LIGHTYELLOW_KW,
            "lime" => LIME_KW,
            "limegreen" => LIMEGREEN_KW,
            "linen" => LINEN_KW,
            "magenta" => MAGENTA_KW,
            "maroon" => MAROON_KW,
            "mediumaquamarine" => MEDIUMAQUAMARINE_KW,
            "mediumblue" => MEDIUMBLUE_KW,
            "mediumorchid" => MEDIUMORCHID_KW,
            "mediumpurple" => MEDIUMPURPLE_KW,
            "mediumseagreen" => MEDIUMSEAGREEN_KW,
            "mediumslateblue" => MEDIUMSLATEBLUE_KW,
            "mediumspringgreen" => MEDIUMSPRINGGREEN_KW,
            "mediumturquoise" => MEDIUMTURQUOISE_KW,
            "mediumvioletred" => MEDIUMVIOLETRED_KW,
            "midnightblue" => MIDNIGHTBLUE_KW,
            "mintcream" => MINTCREAM_KW,
            "mistyrose" => MISTYROSE_KW,
            "moccasin" => MOCCASIN_KW,
            "navajowhite" => NAVAJOWHITE_KW,
            "navy" => NAVY_KW,
            "navyblue" => NAVYBLUE_KW,
            "oldlace" => OLDLACE_KW,
            "olive" => OLIVE_KW,
            "olivedrab" => OLIVEDRAB_KW,
            "orange" => ORANGE_KW,
            "orangered" => ORANGERED_KW,
            "orchid" => ORCHID_KW,
            "palegoldenrod" => PALEGOLDENROD_KW,
            "palegreen" => PALEGREEN_KW,
            "paleturquoise" => PALETURQUOISE_KW,
            "palevioletred" => PALEVIOLETRED_KW,
            "papayawhip" => PAPAYAWHIP_KW,
            "peachpuff" => PEACHPUFF_KW,
            "peru" => PERU_KW,
            "pink" => PINK_KW,
            "plum" => PLUM_KW,
            "powderblue" => POWDERBLUE_KW,
            "purple" => PURPLE_KW,
            "red" => RED_KW,
            "rosybrown" => ROSYBROWN_KW,
            "royalblue" => ROYALBLUE_KW,
            "saddlebrown" => SADDLEBROWN_KW,
            "salmon" => SALMON_KW,
            "sandybrown" => SANDYBROWN_KW,
            "seagreen" => SEAGREEN_KW,
            "seashell" => SEASHELL_KW,
            "sienna" => SIENNA_KW,
            "silver" => SILVER_KW,
            "skyblue" => SKYBLUE_KW,
            "slateblue" => SLATEBLUE_KW,
            "slategray" => SLATEGRAY_KW,
            "snow" => SNOW_KW,
            "springgreen" => SPRINGGREEN_KW,
            "steelblue" => STEELBLUE_KW,
            "tan" => TAN_KW,
            "teal" => TEAL_KW,
            "thistle" => THISTLE_KW,
            "tomato" => TOMATO_KW,
            "turquoise" => TURQUOISE_KW,
            "violet" => VIOLET_KW,
            "wheat" => WHEAT_KW,
            "white" => WHITE_KW,
            "whitesmoke" => WHITESMOKE_KW,
            "yellow" => YELLOW_KW,
            "yellowgreen" => YELLOWGREEN_KW,
            "media" => MEDIA_KW,
            "keyframes" => KEYFRAMES_KW,
            "not" => NOT_KW,
            "and" => AND_KW,
            "only" => ONLY_KW,
            "or" => OR_KW,
            "i" => I_KW,
            "important" => IMPORTANT_KW,
            "from" => FROM_KW,
            "to" => TO_KW,
            "var" => VAR_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            SEMICOLON => ";",
            COMMA => ",",
            L_PAREN => "(",
            R_PAREN => ")",
            L_CURLY => "{",
            R_CURLY => "}",
            L_BRACK => "[",
            R_BRACK => "]",
            L_ANGLE => "<",
            R_ANGLE => ">",
            TILDE => "~",
            HASH => "#",
            AMP => "&",
            PIPE => "|",
            PLUS => "+",
            STAR => "*",
            SLASH => "/",
            CARET => "^",
            PERCENT => "%",
            DOT => ".",
            COLON => ":",
            COLON2 => "::",
            EQ => "=",
            BANG => "!",
            NEQ => "!=",
            MINUS => "-",
            LTEQ => "<=",
            GTEQ => ">=",
            PLUSEQ => "+=",
            PIPEEQ => "|=",
            AMPEQ => "&=",
            CARETEQ => "^=",
            SLASHEQ => "/=",
            STAREQ => "*=",
            PERCENTEQ => "%=",
            AT => "@",
            DOLLAR_EQ => "$=",
            TILDE_EQ => "~=",
            CDC => "-->",
            CDO => "<!--",
            ALICEBLUE_KW => "aliceblue",
            ANTIQUEWHITE_KW => "antiquewhite",
            AQUA_KW => "aqua",
            AQUAMARINE_KW => "aquamarine",
            AZURE_KW => "azure",
            BEIGE_KW => "beige",
            BISQUE_KW => "bisque",
            BLACK_KW => "black",
            BLANCHEDALMOND_KW => "blanchedalmond",
            BLUE_KW => "blue",
            BLUEVIOLET_KW => "blueviolet",
            BROWN_KW => "brown",
            BURLYWOOD_KW => "burlywood",
            CADETBLUE_KW => "cadetblue",
            CHARTREUSE_KW => "chartreuse",
            CHOCOLATE_KW => "chocolate",
            CORAL_KW => "coral",
            CORNFLOWERBLUE_KW => "cornflowerblue",
            CORNSILK_KW => "cornsilk",
            CRIMSON_KW => "crimson",
            CYAN_KW => "cyan",
            DARKBLUE_KW => "darkblue",
            DARKCYAN_KW => "darkcyan",
            DARKGOLDENROD_KW => "darkgoldenrod",
            DARKGRAY_KW => "darkgray",
            DARKGREEN_KW => "darkgreen",
            DARKKHAKI_KW => "darkkhaki",
            DARKMAGENTA_KW => "darkmagenta",
            DARKOLIVEGREEN_KW => "darkolivegreen",
            DARKORANGE_KW => "darkorange",
            DARKORCHID_KW => "darkorchid",
            DARKRED_KW => "darkred",
            DARKSALMON_KW => "darksalmon",
            DARKSEAGREEN_KW => "darkseagreen",
            DARKSLATEBLUE_KW => "darkslateblue",
            DARKSLATEGRAY_KW => "darkslategray",
            DARKTURQUOISE_KW => "darkturquoise",
            DARKVIOLET_KW => "darkviolet",
            DEEPPINK_KW => "deeppink",
            DEEPSKYBLUE_KW => "deepskyblue",
            DIMGRAY_KW => "dimgray",
            DODGERBLUE_KW => "dodgerblue",
            FIREBRICK_KW => "firebrick",
            FLORALWHITE_KW => "floralwhite",
            FORESTGREEN_KW => "forestgreen",
            FUCHSIA_KW => "fuchsia",
            GAINSBORO_KW => "gainsboro",
            GHOSTWHITE_KW => "ghostwhite",
            GOLD_KW => "gold",
            GOLDENROD_KW => "goldenrod",
            GRAY_KW => "gray",
            GREEN_KW => "green",
            GREENYELLOW_KW => "greenyellow",
            HONEYDEW_KW => "honeydew",
            HOTPINK_KW => "hotpink",
            INDIANRED_KW => "indianred",
            INDIGO_KW => "indigo",
            IVORY_KW => "ivory",
            KHAKI_KW => "khaki",
            LAVENDER_KW => "lavender",
            LAVENDERBLUSH_KW => "lavenderblush",
            LAWNGREEN_KW => "lawngreen",
            LEMONCHIFFON_KW => "lemonchiffon",
            LIGHTBLUE_KW => "lightblue",
            LIGHTCORAL_KW => "lightcoral",
            LIGHTCYAN_KW => "lightcyan",
            LIGHTGOLDENRODYELLOW_KW => "lightgoldenrodyellow",
            LIGHTGREEN_KW => "lightgreen",
            LIGHTGREY_KW => "lightgrey",
            LIGHTPINK_KW => "lightpink",
            LIGHTSALMON_KW => "lightsalmon",
            LIGHTSEAGREEN_KW => "lightseagreen",
            LIGHTSKYBLUE_KW => "lightskyblue",
            LIGHTSLATEGRAY_KW => "lightslategray",
            LIGHTSTEELBLUE_KW => "lightsteelblue",
            LIGHTYELLOW_KW => "lightyellow",
            LIME_KW => "lime",
            LIMEGREEN_KW => "limegreen",
            LINEN_KW => "linen",
            MAGENTA_KW => "magenta",
            MAROON_KW => "maroon",
            MEDIUMAQUAMARINE_KW => "mediumaquamarine",
            MEDIUMBLUE_KW => "mediumblue",
            MEDIUMORCHID_KW => "mediumorchid",
            MEDIUMPURPLE_KW => "mediumpurple",
            MEDIUMSEAGREEN_KW => "mediumseagreen",
            MEDIUMSLATEBLUE_KW => "mediumslateblue",
            MEDIUMSPRINGGREEN_KW => "mediumspringgreen",
            MEDIUMTURQUOISE_KW => "mediumturquoise",
            MEDIUMVIOLETRED_KW => "mediumvioletred",
            MIDNIGHTBLUE_KW => "midnightblue",
            MINTCREAM_KW => "mintcream",
            MISTYROSE_KW => "mistyrose",
            MOCCASIN_KW => "moccasin",
            NAVAJOWHITE_KW => "navajowhite",
            NAVY_KW => "navy",
            NAVYBLUE_KW => "navyblue",
            OLDLACE_KW => "oldlace",
            OLIVE_KW => "olive",
            OLIVEDRAB_KW => "olivedrab",
            ORANGE_KW => "orange",
            ORANGERED_KW => "orangered",
            ORCHID_KW => "orchid",
            PALEGOLDENROD_KW => "palegoldenrod",
            PALEGREEN_KW => "palegreen",
            PALETURQUOISE_KW => "paleturquoise",
            PALEVIOLETRED_KW => "palevioletred",
            PAPAYAWHIP_KW => "papayawhip",
            PEACHPUFF_KW => "peachpuff",
            PERU_KW => "peru",
            PINK_KW => "pink",
            PLUM_KW => "plum",
            POWDERBLUE_KW => "powderblue",
            PURPLE_KW => "purple",
            RED_KW => "red",
            ROSYBROWN_KW => "rosybrown",
            ROYALBLUE_KW => "royalblue",
            SADDLEBROWN_KW => "saddlebrown",
            SALMON_KW => "salmon",
            SANDYBROWN_KW => "sandybrown",
            SEAGREEN_KW => "seagreen",
            SEASHELL_KW => "seashell",
            SIENNA_KW => "sienna",
            SILVER_KW => "silver",
            SKYBLUE_KW => "skyblue",
            SLATEBLUE_KW => "slateblue",
            SLATEGRAY_KW => "slategray",
            SNOW_KW => "snow",
            SPRINGGREEN_KW => "springgreen",
            STEELBLUE_KW => "steelblue",
            TAN_KW => "tan",
            TEAL_KW => "teal",
            THISTLE_KW => "thistle",
            TOMATO_KW => "tomato",
            TURQUOISE_KW => "turquoise",
            VIOLET_KW => "violet",
            WHEAT_KW => "wheat",
            WHITE_KW => "white",
            WHITESMOKE_KW => "whitesmoke",
            YELLOW_KW => "yellow",
            YELLOWGREEN_KW => "yellowgreen",
            MEDIA_KW => "media",
            KEYFRAMES_KW => "keyframes",
            NOT_KW => "not",
            AND_KW => "and",
            ONLY_KW => "only",
            OR_KW => "or",
            I_KW => "i",
            IMPORTANT_KW => "important",
            FROM_KW => "from",
            TO_KW => "to",
            VAR_KW => "var",
            CSS_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [;] => { $ crate :: CssSyntaxKind :: SEMICOLON } ; [,] => { $ crate :: CssSyntaxKind :: COMMA } ; ['('] => { $ crate :: CssSyntaxKind :: L_PAREN } ; [')'] => { $ crate :: CssSyntaxKind :: R_PAREN } ; ['{'] => { $ crate :: CssSyntaxKind :: L_CURLY } ; ['}'] => { $ crate :: CssSyntaxKind :: R_CURLY } ; ['['] => { $ crate :: CssSyntaxKind :: L_BRACK } ; [']'] => { $ crate :: CssSyntaxKind :: R_BRACK } ; [<] => { $ crate :: CssSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: CssSyntaxKind :: R_ANGLE } ; [~] => { $ crate :: CssSyntaxKind :: TILDE } ; [#] => { $ crate :: CssSyntaxKind :: HASH } ; [&] => { $ crate :: CssSyntaxKind :: AMP } ; [|] => { $ crate :: CssSyntaxKind :: PIPE } ; [+] => { $ crate :: CssSyntaxKind :: PLUS } ; [*] => { $ crate :: CssSyntaxKind :: STAR } ; [/] => { $ crate :: CssSyntaxKind :: SLASH } ; [^] => { $ crate :: CssSyntaxKind :: CARET } ; [%] => { $ crate :: CssSyntaxKind :: PERCENT } ; [.] => { $ crate :: CssSyntaxKind :: DOT } ; [:] => { $ crate :: CssSyntaxKind :: COLON } ; [::] => { $ crate :: CssSyntaxKind :: COLON2 } ; [=] => { $ crate :: CssSyntaxKind :: EQ } ; [!] => { $ crate :: CssSyntaxKind :: BANG } ; [!=] => { $ crate :: CssSyntaxKind :: NEQ } ; [-] => { $ crate :: CssSyntaxKind :: MINUS } ; [<=] => { $ crate :: CssSyntaxKind :: LTEQ } ; [>=] => { $ crate :: CssSyntaxKind :: GTEQ } ; [+=] => { $ crate :: CssSyntaxKind :: PLUSEQ } ; [|=] => { $ crate :: CssSyntaxKind :: PIPEEQ } ; [&=] => { $ crate :: CssSyntaxKind :: AMPEQ } ; [^=] => { $ crate :: CssSyntaxKind :: CARETEQ } ; [/=] => { $ crate :: CssSyntaxKind :: SLASHEQ } ; [*=] => { $ crate :: CssSyntaxKind :: STAREQ } ; [%=] => { $ crate :: CssSyntaxKind :: PERCENTEQ } ; [@] => { $ crate :: CssSyntaxKind :: AT } ; ["$="] => { $ crate :: CssSyntaxKind :: DOLLAR_EQ } ; [~=] => { $ crate :: CssSyntaxKind :: TILDE_EQ } ; [-->] => { $ crate :: CssSyntaxKind :: CDC } ; [<!--] => { $ crate :: CssSyntaxKind :: CDO } ; [aliceblue] => { $ crate :: CssSyntaxKind :: ALICEBLUE_KW } ; [antiquewhite] => { $ crate :: CssSyntaxKind :: ANTIQUEWHITE_KW } ; [aqua] => { $ crate :: CssSyntaxKind :: AQUA_KW } ; [aquamarine] => { $ crate :: CssSyntaxKind :: AQUAMARINE_KW } ; [azure] => { $ crate :: CssSyntaxKind :: AZURE_KW } ; [beige] => { $ crate :: CssSyntaxKind :: BEIGE_KW } ; [bisque] => { $ crate :: CssSyntaxKind :: BISQUE_KW } ; [black] => { $ crate :: CssSyntaxKind :: BLACK_KW } ; [blanchedalmond] => { $ crate :: CssSyntaxKind :: BLANCHEDALMOND_KW } ; [blue] => { $ crate :: CssSyntaxKind :: BLUE_KW } ; [blueviolet] => { $ crate :: CssSyntaxKind :: BLUEVIOLET_KW } ; [brown] => { $ crate :: CssSyntaxKind :: BROWN_KW } ; [burlywood] => { $ crate :: CssSyntaxKind :: BURLYWOOD_KW } ; [cadetblue] => { $ crate :: CssSyntaxKind :: CADETBLUE_KW } ; [chartreuse] => { $ crate :: CssSyntaxKind :: CHARTREUSE_KW } ; [chocolate] => { $ crate :: CssSyntaxKind :: CHOCOLATE_KW } ; [coral] => { $ crate :: CssSyntaxKind :: CORAL_KW } ; [cornflowerblue] => { $ crate :: CssSyntaxKind :: CORNFLOWERBLUE_KW } ; [cornsilk] => { $ crate :: CssSyntaxKind :: CORNSILK_KW } ; [crimson] => { $ crate :: CssSyntaxKind :: CRIMSON_KW } ; [cyan] => { $ crate :: CssSyntaxKind :: CYAN_KW } ; [darkblue] => { $ crate :: CssSyntaxKind :: DARKBLUE_KW } ; [darkcyan] => { $ crate :: CssSyntaxKind :: DARKCYAN_KW } ; [darkgoldenrod] => { $ crate :: CssSyntaxKind :: DARKGOLDENROD_KW } ; [darkgray] => { $ crate :: CssSyntaxKind :: DARKGRAY_KW } ; [darkgreen] => { $ crate :: CssSyntaxKind :: DARKGREEN_KW } ; [darkkhaki] => { $ crate :: CssSyntaxKind :: DARKKHAKI_KW } ; [darkmagenta] => { $ crate :: CssSyntaxKind :: DARKMAGENTA_KW } ; [darkolivegreen] => { $ crate :: CssSyntaxKind :: DARKOLIVEGREEN_KW } ; [darkorange] => { $ crate :: CssSyntaxKind :: DARKORANGE_KW } ; [darkorchid] => { $ crate :: CssSyntaxKind :: DARKORCHID_KW } ; [darkred] => { $ crate :: CssSyntaxKind :: DARKRED_KW } ; [darksalmon] => { $ crate :: CssSyntaxKind :: DARKSALMON_KW } ; [darkseagreen] => { $ crate :: CssSyntaxKind :: DARKSEAGREEN_KW } ; [darkslateblue] => { $ crate :: CssSyntaxKind :: DARKSLATEBLUE_KW } ; [darkslategray] => { $ crate :: CssSyntaxKind :: DARKSLATEGRAY_KW } ; [darkturquoise] => { $ crate :: CssSyntaxKind :: DARKTURQUOISE_KW } ; [darkviolet] => { $ crate :: CssSyntaxKind :: DARKVIOLET_KW } ; [deeppink] => { $ crate :: CssSyntaxKind :: DEEPPINK_KW } ; [deepskyblue] => { $ crate :: CssSyntaxKind :: DEEPSKYBLUE_KW } ; [dimgray] => { $ crate :: CssSyntaxKind :: DIMGRAY_KW } ; [dodgerblue] => { $ crate :: CssSyntaxKind :: DODGERBLUE_KW } ; [firebrick] => { $ crate :: CssSyntaxKind :: FIREBRICK_KW } ; [floralwhite] => { $ crate :: CssSyntaxKind :: FLORALWHITE_KW } ; [forestgreen] => { $ crate :: CssSyntaxKind :: FORESTGREEN_KW } ; [fuchsia] => { $ crate :: CssSyntaxKind :: FUCHSIA_KW } ; [gainsboro] => { $ crate :: CssSyntaxKind :: GAINSBORO_KW } ; [ghostwhite] => { $ crate :: CssSyntaxKind :: GHOSTWHITE_KW } ; [gold] => { $ crate :: CssSyntaxKind :: GOLD_KW } ; [goldenrod] => { $ crate :: CssSyntaxKind :: GOLDENROD_KW } ; [gray] => { $ crate :: CssSyntaxKind :: GRAY_KW } ; [green] => { $ crate :: CssSyntaxKind :: GREEN_KW } ; [greenyellow] => { $ crate :: CssSyntaxKind :: GREENYELLOW_KW } ; [honeydew] => { $ crate :: CssSyntaxKind :: HONEYDEW_KW } ; [hotpink] => { $ crate :: CssSyntaxKind :: HOTPINK_KW } ; [indianred] => { $ crate :: CssSyntaxKind :: INDIANRED_KW } ; [indigo] => { $ crate :: CssSyntaxKind :: INDIGO_KW } ; [ivory] => { $ crate :: CssSyntaxKind :: IVORY_KW } ; [khaki] => { $ crate :: CssSyntaxKind :: KHAKI_KW } ; [lavender] => { $ crate :: CssSyntaxKind :: LAVENDER_KW } ; [lavenderblush] => { $ crate :: CssSyntaxKind :: LAVENDERBLUSH_KW } ; [lawngreen] => { $ crate :: CssSyntaxKind :: LAWNGREEN_KW } ; [lemonchiffon] => { $ crate :: CssSyntaxKind :: LEMONCHIFFON_KW } ; [lightblue] => { $ crate :: CssSyntaxKind :: LIGHTBLUE_KW } ; [lightcoral] => { $ crate :: CssSyntaxKind :: LIGHTCORAL_KW } ; [lightcyan] => { $ crate :: CssSyntaxKind :: LIGHTCYAN_KW } ; [lightgoldenrodyellow] => { $ crate :: CssSyntaxKind :: LIGHTGOLDENRODYELLOW_KW } ; [lightgreen] => { $ crate :: CssSyntaxKind :: LIGHTGREEN_KW } ; [lightgrey] => { $ crate :: CssSyntaxKind :: LIGHTGREY_KW } ; [lightpink] => { $ crate :: CssSyntaxKind :: LIGHTPINK_KW } ; [lightsalmon] => { $ crate :: CssSyntaxKind :: LIGHTSALMON_KW } ; [lightseagreen] => { $ crate :: CssSyntaxKind :: LIGHTSEAGREEN_KW } ; [lightskyblue] => { $ crate :: CssSyntaxKind :: LIGHTSKYBLUE_KW } ; [lightslategray] => { $ crate :: CssSyntaxKind :: LIGHTSLATEGRAY_KW } ; [lightsteelblue] => { $ crate :: CssSyntaxKind :: LIGHTSTEELBLUE_KW } ; [lightyellow] => { $ crate :: CssSyntaxKind :: LIGHTYELLOW_KW } ; [lime] => { $ crate :: CssSyntaxKind :: LIME_KW } ; [limegreen] => { $ crate :: CssSyntaxKind :: LIMEGREEN_KW } ; [linen] => { $ crate :: CssSyntaxKind :: LINEN_KW } ; [magenta] => { $ crate :: CssSyntaxKind :: MAGENTA_KW } ; [maroon] => { $ crate :: CssSyntaxKind :: MAROON_KW } ; [mediumaquamarine] => { $ crate :: CssSyntaxKind :: MEDIUMAQUAMARINE_KW } ; [mediumblue] => { $ crate :: CssSyntaxKind :: MEDIUMBLUE_KW } ; [mediumorchid] => { $ crate :: CssSyntaxKind :: MEDIUMORCHID_KW } ; [mediumpurple] => { $ crate :: CssSyntaxKind :: MEDIUMPURPLE_KW } ; [mediumseagreen] => { $ crate :: CssSyntaxKind :: MEDIUMSEAGREEN_KW } ; [mediumslateblue] => { $ crate :: CssSyntaxKind :: MEDIUMSLATEBLUE_KW } ; [mediumspringgreen] => { $ crate :: CssSyntaxKind :: MEDIUMSPRINGGREEN_KW } ; [mediumturquoise] => { $ crate :: CssSyntaxKind :: MEDIUMTURQUOISE_KW } ; [mediumvioletred] => { $ crate :: CssSyntaxKind :: MEDIUMVIOLETRED_KW } ; [midnightblue] => { $ crate :: CssSyntaxKind :: MIDNIGHTBLUE_KW } ; [mintcream] => { $ crate :: CssSyntaxKind :: MINTCREAM_KW } ; [mistyrose] => { $ crate :: CssSyntaxKind :: MISTYROSE_KW } ; [moccasin] => { $ crate :: CssSyntaxKind :: MOCCASIN_KW } ; [navajowhite] => { $ crate :: CssSyntaxKind :: NAVAJOWHITE_KW } ; [navy] => { $ crate :: CssSyntaxKind :: NAVY_KW } ; [navyblue] => { $ crate :: CssSyntaxKind :: NAVYBLUE_KW } ; [oldlace] => { $ crate :: CssSyntaxKind :: OLDLACE_KW } ; [olive] => { $ crate :: CssSyntaxKind :: OLIVE_KW } ; [olivedrab] => { $ crate :: CssSyntaxKind :: OLIVEDRAB_KW } ; [orange] => { $ crate :: CssSyntaxKind :: ORANGE_KW } ; [orangered] => { $ crate :: CssSyntaxKind :: ORANGERED_KW } ; [orchid] => { $ crate :: CssSyntaxKind :: ORCHID_KW } ; [palegoldenrod] => { $ crate :: CssSyntaxKind :: PALEGOLDENROD_KW } ; [palegreen] => { $ crate :: CssSyntaxKind :: PALEGREEN_KW } ; [paleturquoise] => { $ crate :: CssSyntaxKind :: PALETURQUOISE_KW } ; [palevioletred] => { $ crate :: CssSyntaxKind :: PALEVIOLETRED_KW } ; [papayawhip] => { $ crate :: CssSyntaxKind :: PAPAYAWHIP_KW } ; [peachpuff] => { $ crate :: CssSyntaxKind :: PEACHPUFF_KW } ; [peru] => { $ crate :: CssSyntaxKind :: PERU_KW } ; [pink] => { $ crate :: CssSyntaxKind :: PINK_KW } ; [plum] => { $ crate :: CssSyntaxKind :: PLUM_KW } ; [powderblue] => { $ crate :: CssSyntaxKind :: POWDERBLUE_KW } ; [purple] => { $ crate :: CssSyntaxKind :: PURPLE_KW } ; [red] => { $ crate :: CssSyntaxKind :: RED_KW } ; [rosybrown] => { $ crate :: CssSyntaxKind :: ROSYBROWN_KW } ; [royalblue] => { $ crate :: CssSyntaxKind :: ROYALBLUE_KW } ; [saddlebrown] => { $ crate :: CssSyntaxKind :: SADDLEBROWN_KW } ; [salmon] => { $ crate :: CssSyntaxKind :: SALMON_KW } ; [sandybrown] => { $ crate :: CssSyntaxKind :: SANDYBROWN_KW } ; [seagreen] => { $ crate :: CssSyntaxKind :: SEAGREEN_KW } ; [seashell] => { $ crate :: CssSyntaxKind :: SEASHELL_KW } ; [sienna] => { $ crate :: CssSyntaxKind :: SIENNA_KW } ; [silver] => { $ crate :: CssSyntaxKind :: SILVER_KW } ; [skyblue] => { $ crate :: CssSyntaxKind :: SKYBLUE_KW } ; [slateblue] => { $ crate :: CssSyntaxKind :: SLATEBLUE_KW } ; [slategray] => { $ crate :: CssSyntaxKind :: SLATEGRAY_KW } ; [snow] => { $ crate :: CssSyntaxKind :: SNOW_KW } ; [springgreen] => { $ crate :: CssSyntaxKind :: SPRINGGREEN_KW } ; [steelblue] => { $ crate :: CssSyntaxKind :: STEELBLUE_KW } ; [tan] => { $ crate :: CssSyntaxKind :: TAN_KW } ; [teal] => { $ crate :: CssSyntaxKind :: TEAL_KW } ; [thistle] => { $ crate :: CssSyntaxKind :: THISTLE_KW } ; [tomato] => { $ crate :: CssSyntaxKind :: TOMATO_KW } ; [turquoise] => { $ crate :: CssSyntaxKind :: TURQUOISE_KW } ; [violet] => { $ crate :: CssSyntaxKind :: VIOLET_KW } ; [wheat] => { $ crate :: CssSyntaxKind :: WHEAT_KW } ; [white] => { $ crate :: CssSyntaxKind :: WHITE_KW } ; [whitesmoke] => { $ crate :: CssSyntaxKind :: WHITESMOKE_KW } ; [yellow] => { $ crate :: CssSyntaxKind :: YELLOW_KW } ; [yellowgreen] => { $ crate :: CssSyntaxKind :: YELLOWGREEN_KW } ; [media] => { $ crate :: CssSyntaxKind :: MEDIA_KW } ; [keyframes] => { $ crate :: CssSyntaxKind :: KEYFRAMES_KW } ; [not] => { $ crate :: CssSyntaxKind :: NOT_KW } ; [and] => { $ crate :: CssSyntaxKind :: AND_KW } ; [only] => { $ crate :: CssSyntaxKind :: ONLY_KW } ; [or] => { $ crate :: CssSyntaxKind :: OR_KW } ; [i] => { $ crate :: CssSyntaxKind :: I_KW } ; [important] => { $ crate :: CssSyntaxKind :: IMPORTANT_KW } ; [from] => { $ crate :: CssSyntaxKind :: FROM_KW } ; [to] => { $ crate :: CssSyntaxKind :: TO_KW } ; [var] => { $ crate :: CssSyntaxKind :: VAR_KW } ; [ident] => { $ crate :: CssSyntaxKind :: IDENT } ; [EOF] => { $ crate :: CssSyntaxKind :: EOF } ; [#] => { $ crate :: CssSyntaxKind :: HASH } ; }
