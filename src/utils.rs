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

use clipboard::{ClipboardContext, ClipboardProvider};
use cursive::Cursive;

use crate::{error::Error, state::GlobalState};

pub fn get_state_mut(c: &mut Cursive) -> Result<GlobalState, Error> {
    match c.take_user_data::<GlobalState>() {
        Some(data) => {
            c.set_user_data(data.clone());
            Ok(data)
        }
        None => Err(Error::NoState),
    }
}

pub fn get_clipboard() -> Result<ClipboardContext, Error> {
    match ClipboardProvider::new() {
        Ok(c) => Ok(c),
        Err(_) => Err(Error::Clipboard),
    }
}

#[macro_export]
macro_rules! herr {
    ($c:expr,$f:expr) => {{
        if let Err(e) = $f($c) {
            e.show_dialog($c);
            return
        }
    }};
    ($c:expr,$f:expr,$($args:expr),*) => {{
        if let Err(e) = $f($c,$($args),*) {
            e.show_dialog($c);
            return
        }
    }};
}

#[macro_export]
macro_rules! herrcl {
    ($f:expr) => {{
        |c| {
            use crate::herr;
            herr!(c,$f);
        }
    }};
    ($f:expr,$($args:expr),*) => {{
        move |c| {
            use crate::herr;
            herr!(c,$f,$($args),*);
        }
    }};
}
