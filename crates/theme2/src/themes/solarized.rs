use gpui::rgba;

use crate::{
    default_color_scales, Appearance, GitStatusColors, PlayerColor, PlayerColors, StatusColors,
    SyntaxTheme, SystemColors, Theme, ThemeColors, ThemeFamily, ThemeStyles,
};

pub fn solarized() -> ThemeFamily {
    ThemeFamily {
        id: "96299b3e-c749-478c-bfc9-c96cdaea8630".into(),
        name: "Solarized".into(),
        author: "Ethan Schoonover (altercation)".into(),
        themes: vec![
            Theme {
                id: "265f93c5-c8e7-4962-b083-8550f4b5c2ff".into(),
                name: "Solarized Dark".into(),
                appearance: Appearance::Dark,
                styles: ThemeStyles {
                    system: SystemColors {
                        transparent: rgba(0x00000000).into(),
                        mac_os_traffic_light_red: rgba(0xec6b5fff).into(),
                        mac_os_traffic_light_yellow: rgba(0xf3bf4dff).into(),
                        mac_os_traffic_light_green: rgba(0x61c454ff).into(),
                    },
                    colors: ThemeColors {
                        border: rgba(0x003847ff).into(),
                        border_variant: rgba(0x003847ff).into(),
                        border_focused: rgba(0x003847ff).into(),
                        border_selected: rgba(0x003847ff).into(),
                        border_transparent: rgba(0x003847ff).into(),
                        border_disabled: rgba(0x003847ff).into(),
                        elevated_surface_background: rgba(0x18191bff).into(),
                        surface_background: rgba(0x18191bff).into(),
                        background: rgba(0x002a35ff).into(),
                        element_background: rgba(0x29a19899).into(),
                        element_hover: rgba(0x272a2dff).into(),
                        element_active: rgba(0x2e3135ff).into(),
                        element_selected: rgba(0x2e3135ff).into(),
                        element_disabled: rgba(0xddeaf814).into(),
                        element_placeholder: rgba(0xb0b4baff).into(),
                        element_drop_target: rgba(0x1166fb18).into(),
                        ghost_element_background: rgba(0x00000000).into(),
                        ghost_element_hover: rgba(0x272a2dff).into(),
                        ghost_element_active: rgba(0x2e3135ff).into(),
                        ghost_element_selected: rgba(0x2e3135ff).into(),
                        ghost_element_disabled: rgba(0xddeaf814).into(),
                        text: rgba(0xedeef0ff).into(),
                        text_muted: rgba(0xb0b4baff).into(),
                        text_placeholder: rgba(0x767a83ff).into(),
                        text_disabled: rgba(0x696e77ff).into(),
                        text_accent: rgba(0x6fb8ffff).into(),
                        icon: rgba(0xb0b4baff).into(),
                        icon_muted: rgba(0x767a83ff).into(),
                        icon_disabled: rgba(0x696e77ff).into(),
                        icon_placeholder: rgba(0x767a83ff).into(),
                        icon_accent: rgba(0x6fb8ffff).into(),
                        status_bar_background: rgba(0x18191bff).into(),
                        title_bar_background: rgba(0x18191bff).into(),
                        toolbar_background: rgba(0x111113ff).into(),
                        tab_bar_background: rgba(0x18191bff).into(),
                        tab_inactive_background: rgba(0x003f51ff).into(),
                        tab_active_background: rgba(0x002a36ff).into(),
                        editor_background: rgba(0x111113ff).into(),
                        editor_gutter_background: rgba(0x111113ff).into(),
                        editor_subheader_background: rgba(0x18191bff).into(),
                        editor_active_line_background: rgba(0xddeaf814).into(),
                        editor_highlighted_line_background: rgba(0xd3edf81d).into(),
                        editor_line_number: rgba(0xddeaf814).into(),
                        editor_active_line_number: rgba(0xddeaf814).into(),
                        editor_invisible: rgba(0xd3edf81d).into(),
                        editor_wrap_guide: rgba(0xd3edf81d).into(),
                        editor_active_wrap_guide: rgba(0xd3edf81d).into(),
                        editor_document_highlight_read_background: rgba(0xd3edf81d).into(),
                        editor_document_highlight_write_background: rgba(0xd3edf81d).into(),
                        terminal_background: rgba(0x111113ff).into(),
                        terminal_ansi_bright_black: rgba(0x586e75ff).into(),
                        terminal_ansi_bright_red: rgba(0xcb4b15ff).into(),
                        terminal_ansi_bright_green: rgba(0x859900ff).into(),
                        terminal_ansi_bright_yellow: rgba(0x657b83ff).into(),
                        terminal_ansi_bright_blue: rgba(0x839496ff).into(),
                        terminal_ansi_bright_magenta: rgba(0x6c71c4ff).into(),
                        terminal_ansi_bright_cyan: rgba(0x93a1a1ff).into(),
                        terminal_ansi_bright_white: rgba(0x839496ff).into(),
                        terminal_ansi_black: rgba(0x063642ff).into(),
                        terminal_ansi_red: rgba(0xdc312eff).into(),
                        terminal_ansi_green: rgba(0x859900ff).into(),
                        terminal_ansi_yellow: rgba(0xb58800ff).into(),
                        terminal_ansi_blue: rgba(0x258ad2ff).into(),
                        terminal_ansi_magenta: rgba(0xd33582ff).into(),
                        terminal_ansi_cyan: rgba(0x29a198ff).into(),
                        terminal_ansi_white: rgba(0x839496ff).into(),
                    },
                    status: StatusColors {
                        conflict: rgba(0xff9592ff).into(),
                        created: rgba(0x70cf82ff).into(),
                        deleted: rgba(0xff9592ff).into(),
                        error: rgba(0xff9592ff).into(),
                        hidden: rgba(0xb0b4baff).into(),
                        ignored: rgba(0xb0b4baff).into(),
                        info: rgba(0x6fb8ffff).into(),
                        modified: rgba(0xf5e147ff).into(),
                        renamed: rgba(0x6fb8ffff).into(),
                        success: rgba(0x70cf82ff).into(),
                        warning: rgba(0xf5e147ff).into(),
                    },
                    git: GitStatusColors {
                        conflict: rgba(0xffa057ff).into(),
                        created: rgba(0x70cf82ff).into(),
                        deleted: rgba(0xff9592ff).into(),
                        ignored: rgba(0xb0b4baff).into(),
                        modified: rgba(0xf5e147ff).into(),
                        renamed: rgba(0x6fb8ffff).into(),
                    },
                    player: PlayerColors(vec![
                        PlayerColor {
                            cursor: rgba(0x000000ff).into(),
                            background: rgba(0x000000ff).into(),
                            selection: rgba(0x000000ff).into(),
                        },
                        PlayerColor {
                            cursor: rgba(0x000000ff).into(),
                            background: rgba(0x000000ff).into(),
                            selection: rgba(0x000000ff).into(),
                        },
                        PlayerColor {
                            cursor: rgba(0x000000ff).into(),
                            background: rgba(0x000000ff).into(),
                            selection: rgba(0x000000ff).into(),
                        },
                        PlayerColor {
                            cursor: rgba(0x000000ff).into(),
                            background: rgba(0x000000ff).into(),
                            selection: rgba(0x000000ff).into(),
                        },
                    ]),
                    syntax: SyntaxTheme {
                        highlights: vec![
                            ("attribute".into(), rgba(0x4ccce6ff).into()),
                            ("boolean".into(), rgba(0xff977dff).into()),
                            ("comment".into(), rgba(0xb0b4baff).into()),
                            ("comment.doc".into(), rgba(0xe0dffeff).into()),
                            ("constant".into(), rgba(0x8c323aff).into()),
                            ("constructor".into(), rgba(0x8c323aff).into()),
                            ("embedded".into(), rgba(0x8c323aff).into()),
                            ("emphasis".into(), rgba(0x8c323aff).into()),
                            ("emphasis.strong".into(), rgba(0x8c323aff).into()),
                            ("enum".into(), rgba(0x8c323aff).into()),
                            ("function".into(), rgba(0x8c323aff).into()),
                            ("hint".into(), rgba(0x8c323aff).into()),
                            ("keyword".into(), rgba(0xffa057ff).into()),
                            ("label".into(), rgba(0x8c323aff).into()),
                            ("link_text".into(), rgba(0x8c323aff).into()),
                            ("link_uri".into(), rgba(0x8c323aff).into()),
                            ("number".into(), rgba(0x8c323aff).into()),
                            ("operator".into(), rgba(0x8c323aff).into()),
                            ("predictive".into(), rgba(0x8c323aff).into()),
                            ("preproc".into(), rgba(0x8c323aff).into()),
                            ("primary".into(), rgba(0x8c323aff).into()),
                            ("property".into(), rgba(0x8c323aff).into()),
                            ("punctuation".into(), rgba(0xb0b4baff).into()),
                            ("punctuation.bracket".into(), rgba(0xb0b4baff).into()),
                            ("punctuation.delimiter".into(), rgba(0xb0b4baff).into()),
                            ("punctuation.list_marker".into(), rgba(0x6fb8ffff).into()),
                            ("punctuation.special".into(), rgba(0x8c323aff).into()),
                            ("string".into(), rgba(0x1ed8a3ff).into()),
                            ("string.escape".into(), rgba(0x8c323aff).into()),
                            ("string.regex".into(), rgba(0xff977dff).into()),
                            ("string.special".into(), rgba(0x8c323aff).into()),
                            ("string.special.symbol".into(), rgba(0x8c323aff).into()),
                            ("tag".into(), rgba(0x8c323aff).into()),
                            ("text.literal".into(), rgba(0x8c323aff).into()),
                            ("title".into(), rgba(0x8c323aff).into()),
                            ("type".into(), rgba(0x8c323aff).into()),
                            ("variable".into(), rgba(0x8c323aff).into()),
                            ("variable.special".into(), rgba(0x8c323aff).into()),
                            ("variant".into(), rgba(0x8c323aff).into()),
                        ],
                    },
                },
            },
            Theme {
                id: "40e2070d-5846-4182-9dc9-bf56badf019f".into(),
                name: "Solarized Light".into(),
                appearance: Appearance::Light,
                styles: ThemeStyles {
                    system: SystemColors {
                        transparent: rgba(0x00000000).into(),
                        mac_os_traffic_light_red: rgba(0xec6b5fff).into(),
                        mac_os_traffic_light_yellow: rgba(0xf3bf4dff).into(),
                        mac_os_traffic_light_green: rgba(0x61c454ff).into(),
                    },
                    colors: ThemeColors {
                        border: rgba(0xddd6c1ff).into(),
                        border_variant: rgba(0xddd6c1ff).into(),
                        border_focused: rgba(0xddd6c1ff).into(),
                        border_selected: rgba(0xddd6c1ff).into(),
                        border_transparent: rgba(0xddd6c1ff).into(),
                        border_disabled: rgba(0xddd6c1ff).into(),
                        elevated_surface_background: rgba(0xf9f9fbff).into(),
                        surface_background: rgba(0xf9f9fbff).into(),
                        background: rgba(0xfdf6e3ff).into(),
                        element_background: rgba(0xab9d56ff).into(),
                        element_hover: rgba(0xe8e8ecff).into(),
                        element_active: rgba(0xe0e1e6ff).into(),
                        element_selected: rgba(0xe0e1e6ff).into(),
                        element_disabled: rgba(0x0000320f).into(),
                        element_placeholder: rgba(0x60646cff).into(),
                        element_drop_target: rgba(0x008bff0b).into(),
                        ghost_element_background: rgba(0x00000000).into(),
                        ghost_element_hover: rgba(0xe8e8ecff).into(),
                        ghost_element_active: rgba(0xe0e1e6ff).into(),
                        ghost_element_selected: rgba(0xe0e1e6ff).into(),
                        ghost_element_disabled: rgba(0x0000320f).into(),
                        text: rgba(0x1c2024ff).into(),
                        text_muted: rgba(0x60646cff).into(),
                        text_placeholder: rgba(0x80838dff).into(),
                        text_disabled: rgba(0x8b8d98ff).into(),
                        text_accent: rgba(0x0c73ceff).into(),
                        icon: rgba(0x60646cff).into(),
                        icon_muted: rgba(0x80838dff).into(),
                        icon_disabled: rgba(0x8b8d98ff).into(),
                        icon_placeholder: rgba(0x80838dff).into(),
                        icon_accent: rgba(0x0c73ceff).into(),
                        status_bar_background: rgba(0xf9f9fbff).into(),
                        title_bar_background: rgba(0xf9f9fbff).into(),
                        toolbar_background: rgba(0xfcfcfdff).into(),
                        tab_bar_background: rgba(0xf9f9fbff).into(),
                        tab_inactive_background: rgba(0xd3cbb7ff).into(),
                        tab_active_background: rgba(0xfdf6e3ff).into(),
                        editor_background: rgba(0xfcfcfdff).into(),
                        editor_gutter_background: rgba(0xfcfcfdff).into(),
                        editor_subheader_background: rgba(0xf9f9fbff).into(),
                        editor_active_line_background: rgba(0x0000320f).into(),
                        editor_highlighted_line_background: rgba(0x00002c17).into(),
                        editor_line_number: rgba(0x0000320f).into(),
                        editor_active_line_number: rgba(0x0000320f).into(),
                        editor_invisible: rgba(0x00002c17).into(),
                        editor_wrap_guide: rgba(0x00002c17).into(),
                        editor_active_wrap_guide: rgba(0x00002c17).into(),
                        editor_document_highlight_read_background: rgba(0x00002c17).into(),
                        editor_document_highlight_write_background: rgba(0x00002c17).into(),
                        terminal_background: rgba(0xfcfcfdff).into(),
                        terminal_ansi_bright_black: rgba(0x657b83ff).into(),
                        terminal_ansi_bright_red: rgba(0xcb4b15ff).into(),
                        terminal_ansi_bright_green: rgba(0x859900ff).into(),
                        terminal_ansi_bright_yellow: rgba(0x657b83ff).into(),
                        terminal_ansi_bright_blue: rgba(0x839496ff).into(),
                        terminal_ansi_bright_magenta: rgba(0x6c71c4ff).into(),
                        terminal_ansi_bright_cyan: rgba(0x93a1a1ff).into(),
                        terminal_ansi_bright_white: rgba(0xeee8d5ff).into(),
                        terminal_ansi_black: rgba(0x657b83ff).into(),
                        terminal_ansi_red: rgba(0xdc312eff).into(),
                        terminal_ansi_green: rgba(0x859900ff).into(),
                        terminal_ansi_yellow: rgba(0xb58800ff).into(),
                        terminal_ansi_blue: rgba(0x258ad2ff).into(),
                        terminal_ansi_magenta: rgba(0xd33582ff).into(),
                        terminal_ansi_cyan: rgba(0x29a198ff).into(),
                        terminal_ansi_white: rgba(0xeee8d5ff).into(),
                    },
                    status: StatusColors {
                        conflict: rgba(0xff9592ff).into(),
                        created: rgba(0x70cf82ff).into(),
                        deleted: rgba(0xff9592ff).into(),
                        error: rgba(0xff9592ff).into(),
                        hidden: rgba(0xb0b4baff).into(),
                        ignored: rgba(0xb0b4baff).into(),
                        info: rgba(0x6fb8ffff).into(),
                        modified: rgba(0xf5e147ff).into(),
                        renamed: rgba(0x6fb8ffff).into(),
                        success: rgba(0x70cf82ff).into(),
                        warning: rgba(0xf5e147ff).into(),
                    },
                    git: GitStatusColors {
                        conflict: rgba(0xffa057ff).into(),
                        created: rgba(0x70cf82ff).into(),
                        deleted: rgba(0xff9592ff).into(),
                        ignored: rgba(0xb0b4baff).into(),
                        modified: rgba(0xf5e147ff).into(),
                        renamed: rgba(0x6fb8ffff).into(),
                    },
                    player: PlayerColors(vec![
                        PlayerColor {
                            cursor: rgba(0x000000ff).into(),
                            background: rgba(0x000000ff).into(),
                            selection: rgba(0x000000ff).into(),
                        },
                        PlayerColor {
                            cursor: rgba(0x000000ff).into(),
                            background: rgba(0x000000ff).into(),
                            selection: rgba(0x000000ff).into(),
                        },
                        PlayerColor {
                            cursor: rgba(0x000000ff).into(),
                            background: rgba(0x000000ff).into(),
                            selection: rgba(0x000000ff).into(),
                        },
                        PlayerColor {
                            cursor: rgba(0x000000ff).into(),
                            background: rgba(0x000000ff).into(),
                            selection: rgba(0x000000ff).into(),
                        },
                    ]),
                    syntax: SyntaxTheme {
                        highlights: vec![
                            ("attribute".into(), rgba(0x4ccce6ff).into()),
                            ("boolean".into(), rgba(0xff977dff).into()),
                            ("comment".into(), rgba(0xb0b4baff).into()),
                            ("comment.doc".into(), rgba(0xe0dffeff).into()),
                            ("constant".into(), rgba(0x8c323aff).into()),
                            ("constructor".into(), rgba(0x8c323aff).into()),
                            ("embedded".into(), rgba(0x8c323aff).into()),
                            ("emphasis".into(), rgba(0x8c323aff).into()),
                            ("emphasis.strong".into(), rgba(0x8c323aff).into()),
                            ("enum".into(), rgba(0x8c323aff).into()),
                            ("function".into(), rgba(0x8c323aff).into()),
                            ("hint".into(), rgba(0x8c323aff).into()),
                            ("keyword".into(), rgba(0xffa057ff).into()),
                            ("label".into(), rgba(0x8c323aff).into()),
                            ("link_text".into(), rgba(0x8c323aff).into()),
                            ("link_uri".into(), rgba(0x8c323aff).into()),
                            ("number".into(), rgba(0x8c323aff).into()),
                            ("operator".into(), rgba(0x8c323aff).into()),
                            ("predictive".into(), rgba(0x8c323aff).into()),
                            ("preproc".into(), rgba(0x8c323aff).into()),
                            ("primary".into(), rgba(0x8c323aff).into()),
                            ("property".into(), rgba(0x8c323aff).into()),
                            ("punctuation".into(), rgba(0xb0b4baff).into()),
                            ("punctuation.bracket".into(), rgba(0xb0b4baff).into()),
                            ("punctuation.delimiter".into(), rgba(0xb0b4baff).into()),
                            ("punctuation.list_marker".into(), rgba(0x6fb8ffff).into()),
                            ("punctuation.special".into(), rgba(0x8c323aff).into()),
                            ("string".into(), rgba(0x1ed8a3ff).into()),
                            ("string.escape".into(), rgba(0x8c323aff).into()),
                            ("string.regex".into(), rgba(0xff977dff).into()),
                            ("string.special".into(), rgba(0x8c323aff).into()),
                            ("string.special.symbol".into(), rgba(0x8c323aff).into()),
                            ("tag".into(), rgba(0x8c323aff).into()),
                            ("text.literal".into(), rgba(0x8c323aff).into()),
                            ("title".into(), rgba(0x8c323aff).into()),
                            ("type".into(), rgba(0x8c323aff).into()),
                            ("variable".into(), rgba(0x8c323aff).into()),
                            ("variable.special".into(), rgba(0x8c323aff).into()),
                            ("variant".into(), rgba(0x8c323aff).into()),
                        ],
                    },
                },
            },
        ],
        scales: default_color_scales(),
    }
}
