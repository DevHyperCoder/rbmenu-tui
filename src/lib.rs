/*
 * rbmenu-tui - RBMenu TUI
 * Copyright (C) 2022 DevHyperCoder
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{path::PathBuf, process::exit};

use cursive::{
    views::{LinearLayout, TextView},
    Cursive, CursiveExt,
};
use cursive_aligned_view::Alignable;
use home::home_dir;
use keybinds::setup_keybinds;
use rbmenu::data::read_data_file;
use state::GlobalState;
use views::{bm_list::get_bm_list, cmd_line::get_cmd_line};

pub mod actions;
pub mod error;
pub mod keybinds;
pub mod state;
pub mod utils;
pub mod views;

pub fn run() {
    let mut c = Cursive::default();
    let theme_file_path = match get_theme_file() {
        Some(a) => a,
        None => {
            eprintln!("Unable to get home directory");
            exit(1)
        }
    };

    if c.load_theme_file(theme_file_path).is_err() {
        eprintln!("Error loading theme file. Using default theme.");
    }

    let data = match read_data_file() {
        Ok(e) => e,
        Err(e) => {
            return eprintln!("Error: {:?}", e);
        }
    };

    let state = GlobalState {
        data,
        current_state: state::CurrentState::Add,
        filter: rbmenu::bookmark_query::BookmarkQuery {
            id: None,
            name: None,
        },
    };

    c.set_user_data(state.clone());

    setup_keybinds(&mut c);

    c.add_fullscreen_layer(
        LinearLayout::vertical()
            .child(TextView::new("RBMENU").align_center())
            .child(get_bm_list(&state))
            .child(get_cmd_line()),
    );

    c.run()
}

pub fn get_theme_file() -> Option<PathBuf> {
    let home = home_dir()?;

    Some(home.join(".local/share/rbmenu/theme.toml"))
}
