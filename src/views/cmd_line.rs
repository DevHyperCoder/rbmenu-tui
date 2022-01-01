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
    event::Key,
    traits::{Boxable, Nameable},
    views::{EditView, NamedView, OnEventView, ResizedView},
};

use crate::{actions::execute, herr};

use super::bm_list::BM_LIST;

pub const CMD_LINE: &str = "CMD_LINE";

type CmdLine = OnEventView<ResizedView<NamedView<EditView>>>;

pub fn get_cmd_line() -> CmdLine {
    OnEventView::new(
        EditView::new()
            .disabled()
            .on_submit(|c, a| herr!(c, execute, a))
            .with_name(CMD_LINE)
            .full_width(),
    )
    .on_event(Key::Esc, |c| {
        c.call_on_name(CMD_LINE, |e: &mut EditView| {
            e.set_content("");
            e.disable();
        });
        c.focus_name(BM_LIST).unwrap();
    })
}
