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
    traits::{Boxable, Nameable},
    views::{Dialog, EditView, LinearLayout, TextView},
    Cursive,
};
use rbmenu::bookmark::Bookmark;

use crate::{actions::load_bookmarks, error::Result, herr, utils::get_state_mut};

pub fn add_edit_dialog(c: &mut Cursive, b: &Bookmark) {
    let new_b = b.clone();
    c.add_layer(
        Dialog::new()
            .content(
                LinearLayout::vertical()
                    .child(TextView::new(format!("Editing: {}", b.name)))
                    .child(
                        LinearLayout::horizontal()
                            .child(TextView::new("Name: "))
                            .child(
                                EditView::new()
                                    .content(b.name.clone())
                                    .with_name("EDIT_NAME")
                                    .full_width(),
                            ),
                    )
                    .child(
                        LinearLayout::horizontal()
                            .child(TextView::new("Link: "))
                            .child(
                                EditView::new()
                                    .content(b.link.clone())
                                    .with_name("EDIT_LINK")
                                    .full_width(),
                            ),
                    ),
            )
            .button("Edit", move |c| {
                herr!(c, edit_bookmark, new_b.clone());
                c.pop_layer();
            })
            .dismiss_button("Cancel"),
    )
}

fn edit_bookmark(c: &mut Cursive, b: Bookmark) -> Result<()> {
    let name = c.call_on_name("EDIT_NAME", |e: &mut EditView| {
        e.get_content().as_ref().clone()
    });
    let link = c.call_on_name("EDIT_LINK", |e: &mut EditView| {
        e.get_content().as_ref().clone()
    });

    let mut state = get_state_mut(c)?;

    rbmenu::commands::update(
        &mut state.data,
        rbmenu::bookmark_query::BookmarkUpdateQuery {
            id: b.id,
            name,
            link,
        },
    )?;

    load_bookmarks(c)
}
