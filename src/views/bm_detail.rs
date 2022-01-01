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

use std::fmt::Display;

use cursive::{
    views::{Dialog, LinearLayout, TextView},
    Cursive,
};
use cursive_aligned_view::Alignable;
use rbmenu::bookmark::Bookmark;

use crate::{actions::copy_bookmark_link, herr};

pub fn get_bm_view(c: &mut Cursive, b: Bookmark) {
    c.add_layer(
        Dialog::new()
            .content(
                LinearLayout::vertical()
                    .child(TextView::new(format!("{}", RbMenuBookMark(&b))))
                    //.child()
                    .align_center(),
            )
            .button("Copy", move |c| {
                herr!(c, copy_bookmark_link, &b);
                c.add_layer(Dialog::around(TextView::new("Copied to clipboard")).button(
                    "Ok",
                    |c| {
                        c.pop_layer();
                        c.pop_layer();
                    },
                ));
            })
            .dismiss_button("Ok"),
    )
}

struct RbMenuBookMark<'a>(&'a Bookmark);

impl<'a> Display for RbMenuBookMark<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = &self.0;
        let s = format!(
            "
Id: {}
Name: {}
Link: {}
",
            b.id, b.name, b.link
        );
        write!(f, "{}", s)
    }
}
