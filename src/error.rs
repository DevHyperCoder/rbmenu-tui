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

use cursive::{traits::Boxable, views::Dialog, Cursive};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // Could not get global state of application from Cursive
    NoState,
    // Invalid 'id' type
    IdParse,
    // Nothing selected to do operation on
    NoSelect,

    // Bookmark by Id / Name does not exist
    BookmarkDoesNotExist,
    // Unable to get system clipboard
    Clipboard,
    // Unable to write to system clipboard
    ClipboardWrite,

    // Rbmenu errors
    RbMenu(rbmenu::error::Error),
}

impl Error {
    pub fn show_dialog(self, c: &mut Cursive) {
        c.add_layer(
            Dialog::info(format!("{:?}", self))
                .title("ERROR")
                .full_screen(),
        );
    }
}

impl From<rbmenu::error::Error> for Error {
    fn from(a: rbmenu::error::Error) -> Self {
        Self::RbMenu(a)
    }
}
