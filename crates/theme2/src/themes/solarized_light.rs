use gpui2::rgba;

use crate::{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata};

pub fn solarized_light() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Solarized Light".into(),
            is_light: true,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0x9faaa8ff).into(),
        border_variant: rgba(0x9faaa8ff).into(),
        border_focused: rgba(0xbfd3efff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0xcfd0c4ff).into(),
        surface: rgba(0xf3eddaff).into(),
        background: rgba(0xcfd0c4ff).into(),
        filled_element: rgba(0xcfd0c4ff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0xdbe6f6ff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0xdbe6f6ff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0x002a35ff).into(),
        text_muted: rgba(0x34555eff).into(),
        text_placeholder: rgba(0xdc3330ff).into(),
        text_disabled: rgba(0x6a7f86ff).into(),
        text_accent: rgba(0x288bd1ff).into(),
        icon_muted: rgba(0x34555eff).into(),
        syntax: SyntaxTheme {
            highlights: vec![
                ("string.escape".into(), rgba(0x30525bff).into()),
                ("boolean".into(), rgba(0x849903ff).into()),
                ("comment.doc".into(), rgba(0x30525bff).into()),
                ("string.special".into(), rgba(0xcb4b17ff).into()),
                ("punctuation".into(), rgba(0x04333eff).into()),
                ("emphasis".into(), rgba(0x288bd1ff).into()),
                ("type".into(), rgba(0x2ba198ff).into()),
                ("preproc".into(), rgba(0x002a35ff).into()),
                ("emphasis.strong".into(), rgba(0x288bd1ff).into()),
                ("constant".into(), rgba(0x849903ff).into()),
                ("title".into(), rgba(0x002a35ff).into()),
                ("operator".into(), rgba(0xcb4b17ff).into()),
                ("punctuation.bracket".into(), rgba(0x04333eff).into()),
                ("link_uri".into(), rgba(0x849903ff).into()),
                ("label".into(), rgba(0x288bd1ff).into()),
                ("enum".into(), rgba(0xcb4b17ff).into()),
                ("property".into(), rgba(0x288bd1ff).into()),
                ("predictive".into(), rgba(0x679aafff).into()),
                ("punctuation.special".into(), rgba(0x04333eff).into()),
                ("text.literal".into(), rgba(0xcb4b17ff).into()),
                ("string".into(), rgba(0xcb4b17ff).into()),
                ("string.regex".into(), rgba(0xcb4b17ff).into()),
                ("variable".into(), rgba(0x002a35ff).into()),
                ("tag".into(), rgba(0x288bd1ff).into()),
                ("string.special.symbol".into(), rgba(0xcb4b17ff).into()),
                ("link_text".into(), rgba(0xcb4b17ff).into()),
                ("punctuation.list_marker".into(), rgba(0x04333eff).into()),
                ("keyword".into(), rgba(0x288bd1ff).into()),
                ("constructor".into(), rgba(0x288bd1ff).into()),
                ("attribute".into(), rgba(0x288bd1ff).into()),
                ("variant".into(), rgba(0x288bd1ff).into()),
                ("function".into(), rgba(0xb58903ff).into()),
                ("primary".into(), rgba(0x002a35ff).into()),
                ("hint".into(), rgba(0x5789a3ff).into()),
                ("comment".into(), rgba(0x30525bff).into()),
                ("number".into(), rgba(0x849903ff).into()),
                ("punctuation.delimiter".into(), rgba(0x04333eff).into()),
                ("embedded".into(), rgba(0x002a35ff).into()),
            ],
        },
        status_bar: rgba(0xcfd0c4ff).into(),
        title_bar: rgba(0xcfd0c4ff).into(),
        toolbar: rgba(0xfdf6e3ff).into(),
        tab_bar: rgba(0xf3eddaff).into(),
        editor: rgba(0xfdf6e3ff).into(),
        editor_subheader: rgba(0xf3eddaff).into(),
        editor_active_line: rgba(0xf3eddaff).into(),
        terminal: rgba(0xfdf6e3ff).into(),
        image_fallback_background: rgba(0xcfd0c4ff).into(),
        git_created: rgba(0x849903ff).into(),
        git_modified: rgba(0x288bd1ff).into(),
        git_deleted: rgba(0xdc3330ff).into(),
        git_conflict: rgba(0xb58903ff).into(),
        git_ignored: rgba(0x6a7f86ff).into(),
        git_renamed: rgba(0xb58903ff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x288bd1ff).into(),
                selection: rgba(0x288bd13d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x849903ff).into(),
                selection: rgba(0x8499033d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xd33781ff).into(),
                selection: rgba(0xd337813d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xcb4b17ff).into(),
                selection: rgba(0xcb4b173d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x6c71c3ff).into(),
                selection: rgba(0x6c71c33d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x2ba198ff).into(),
                selection: rgba(0x2ba1983d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xdc3330ff).into(),
                selection: rgba(0xdc33303d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xb58903ff).into(),
                selection: rgba(0xb589033d).into(),
            },
        ],
    }
}
