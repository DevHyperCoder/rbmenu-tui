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

use crate::{actions::show_bookmark_with_id, herr, state::GlobalState};
use cursive::{
    traits::{Boxable, Nameable},
    views::{NamedView, ResizedView, SelectView},
};

pub const BM_LIST: &str = "BM_LIST";
type BmList = ResizedView<NamedView<SelectView<u32>>>;

pub fn get_bm_list(c: &GlobalState) -> BmList {
    let mut l = SelectView::new();

    for i in c.data.bookmarks.clone() {
        l.add_item(format!("{} {}", i.id, i.name), i.id)
    }

    l.set_on_submit(|c, s| herr!(c, show_bookmark_with_id, s));

    l.with_name(BM_LIST).full_height()
}
