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

use cursive::{
    views::{Dialog, EditView, SelectView, TextView},
    Cursive,
};

use crate::{
    actions::{
        copy_selected_bookmark, edit_selected_bookmark, remove_selected_bookmark,
        show_bookmark_on_l_press,
    },
    herr, herrcl,
    state::{CurrentState, GlobalState},
    views::{bm_list::BM_LIST, cmd_line::CMD_LINE},
};

pub fn setup_keybinds(c: &mut Cursive) {
    c.add_global_callback('?', help);
    c.add_global_callback('a', add_new_bmark);
    c.add_global_callback('d', remove_bmark);
    c.add_global_callback('x', herrcl!(remove_selected_bookmark));
    c.add_global_callback('y', herrcl!(copy_selected_bookmark));
    c.add_global_callback('l', herrcl!(show_bookmark_on_l_press));
    c.add_global_callback('e', herrcl!(edit_selected_bookmark));
    c.add_global_callback('j', go_down);
    c.add_global_callback('k', go_up);
    c.add_global_callback('/', filter_bmark);
    c.add_global_callback('q', quit);
}

fn quit(c: &mut Cursive) {
    c.quit()
}

fn help(c: &mut Cursive) {
    c.add_layer(
        Dialog::new()
            .title("HELP")
            .content(TextView::new(include_str!("../help.md")))
            .dismiss_button("Close"),
    )
}

fn add_new_bmark(c: &mut Cursive) {
    c.with_user_data(|state: &mut GlobalState| {
        state.current_state = CurrentState::Add;
    });
    start_cmd_line(c);
}

fn remove_bmark(c: &mut Cursive) {
    c.with_user_data(|state: &mut GlobalState| {
        state.current_state = CurrentState::Remove;
    });
    start_cmd_line(c);
}

fn filter_bmark(c: &mut Cursive) {
    c.with_user_data(|state: &mut GlobalState| {
        state.current_state = CurrentState::List;
    });
    start_cmd_line(c);
}

fn start_cmd_line(c: &mut Cursive) {
    c.call_on_name(CMD_LINE, |e: &mut EditView| {
        e.enable();
    });
    c.focus_name(CMD_LINE).unwrap(); // Safe
}

fn go_up(c: &mut Cursive) {
    c.call_on_name(BM_LIST, |s: &mut SelectView<u32>| {
        s.select_up(1);
    });
}

fn go_down(c: &mut Cursive) {
    c.call_on_name(BM_LIST, |s: &mut SelectView<u32>| {
        s.select_down(1);
    });
}
