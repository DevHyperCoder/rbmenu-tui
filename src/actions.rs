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

use clipboard::ClipboardProvider;
use cursive::{
    views::{Dialog, EditView, SelectView},
    Cursive,
};
use rbmenu::{bookmark::Bookmark, bookmark_query::BookmarkQuery, data::read_data_file};

use crate::{
    error::{Error, Result},
    state::{CurrentState, GlobalState},
    utils::{get_clipboard, get_state_mut},
    views::{bm_detail::get_bm_view, bm_list::BM_LIST, cmd_line::CMD_LINE, edit::add_edit_dialog},
};

pub fn execute(c: &mut Cursive, cmd_line_content: &str) -> Result<()> {
    c.call_on_name(CMD_LINE, |e: &mut EditView| {
        e.set_content("");
        e.disable()
    });

    c.focus_name(BM_LIST).unwrap();

    let state = get_state_mut(c)?;

    match state.current_state {
        CurrentState::Add => {
            let data = state.data;

            // Check if there is a <space> for the name
            let insert_data = match cmd_line_content.find(' ') {
                Some(idx) => (
                    cmd_line_content[..idx].to_owned(),
                    Some(cmd_line_content[idx + 1..].to_owned()),
                ),
                None => (cmd_line_content.to_owned(), None),
            };

            rbmenu::commands::insert(insert_data.0, data, insert_data.1)?
        }
        CurrentState::Remove => {
            let mut data = state.data;

            let remove_query = match cmd_line_content.chars().next() {
                Some(a) => {
                    // Id is being given
                    if a == ':' {
                        let id_str = &cmd_line_content[1..];
                        match id_str.parse::<u32>() {
                            Ok(id) => BookmarkQuery {
                                id: Some(id),
                                name: None,
                            },
                            Err(_) => return Err(Error::IdParse),
                        }
                    } else {
                        BookmarkQuery {
                            id: None,
                            name: Some(cmd_line_content.to_string()),
                        }
                    }
                }
                None => {
                    c.add_layer(Dialog::info("Nothing to remove!"));
                    return Ok(());
                }
            };

            let removed = rbmenu::commands::remove(&mut data, remove_query)?
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<String>>();

            if removed.is_empty() {
                c.add_layer(Dialog::info("Nothing to remove!"));
                return Ok(());
            }

            c.add_layer(Dialog::info(format!("Removed:\n{}", removed.join("\n"))));
        }
        CurrentState::List => {
            let list_query = match cmd_line_content.chars().next() {
                Some(c) => {
                    if c == ':' {
                        let id_str = &cmd_line_content[1..];
                        match id_str.parse::<u32>() {
                            Ok(id) => BookmarkQuery {
                                id: Some(id),
                                name: None,
                            },
                            Err(_) => return Err(Error::IdParse),
                        }
                    } else {
                        BookmarkQuery {
                            id: None,
                            name: Some(cmd_line_content.to_string()),
                        }
                    }
                }
                None => BookmarkQuery {
                    id: None,
                    name: None,
                },
            };

            c.with_user_data(|c: &mut GlobalState| c.filter = list_query);
        }
    };

    load_bookmarks(c)?;

    Ok(())
}

pub fn remove_selected_bookmark(c: &mut Cursive) -> Result<()> {
    let mut data: GlobalState = get_state_mut(c)?;

    let selected = c
        .call_on_name(BM_LIST, |s: &mut SelectView<u32>| s.selection())
        .unwrap();

    if let Some(id) = selected {
        // Returns the id
        rbmenu::commands::remove(
            &mut data.data,
            BookmarkQuery {
                id: Some(*id.as_ref()),
                name: None,
            },
        )?;

        load_bookmarks(c)?;
    }

    Ok(())
}

pub fn edit_selected_bookmark(c: &mut Cursive) -> Result<()> {
    let data = get_state_mut(c)?;

    let selected = c
        .call_on_name(BM_LIST, |s: &mut SelectView<u32>| s.selection())
        .unwrap();

    if let Some(id) = selected {
        let b = match data.data.get_bookmark(*id.as_ref()) {
            Some(b) => b,
            None => return Err(Error::BookmarkDoesNotExist),
        };

        add_edit_dialog(c, b);
    }

    Ok(())
}

pub fn copy_selected_bookmark(c: &mut Cursive) -> Result<()> {
    let data: GlobalState = get_state_mut(c)?;

    let selected = c
        .call_on_name(BM_LIST, |s: &mut SelectView<u32>| s.selection())
        .unwrap();

    if let Some(id) = selected {
        let b = match data.data.get_bookmark(*id.as_ref()) {
            Some(b) => b,
            None => return Err(Error::BookmarkDoesNotExist),
        };

        return copy_bookmark_link(c, b);
    }

    Ok(())
}

pub fn load_bookmarks(c: &mut Cursive) -> Result<()> {
    let mut state: GlobalState = c.take_user_data().unwrap();

    let mut data = read_data_file()?;

    let data_clone = data.clone();
    let bookmarks = rbmenu::commands::list(&data_clone, state.filter.clone())?;

    data.bookmarks.clear();

    for b in bookmarks {
        data.bookmarks.push(b.clone())
    }

    state.data = data.clone();
    state.current_state = CurrentState::List;

    c.set_user_data(state);

    c.call_on_name(BM_LIST, |e: &mut SelectView<u32>| {
        e.clear();

        for i in data.bookmarks.clone() {
            e.add_item(format!("{} {}", i.id, i.name), i.id)
        }
    });

    Ok(())
}

pub fn show_bookmark_on_l_press(c: &mut Cursive) -> Result<()> {
    let selected = c
        .call_on_name(BM_LIST, |s: &mut SelectView<u32>| s.selection())
        .unwrap();

    if let Some(id) = selected {
        show_bookmark_with_id(c, &id)
    } else {
        Ok(())
    }
}

pub fn show_bookmark_with_id(c: &mut Cursive, s: &u32) -> Result<()> {
    let state = get_state_mut(c)?;

    let b = rbmenu::commands::list(
        &state.data,
        rbmenu::bookmark_query::BookmarkQuery {
            id: Some(*s),
            name: None,
        },
    )?;

    if b.is_empty() {
        return Err(Error::BookmarkDoesNotExist);
    }

    get_bm_view(c, b[0].clone());
    Ok(())
}
pub fn copy_bookmark_link(_c: &Cursive, b: &Bookmark) -> Result<()> {
    let mut ctx = get_clipboard()?;
    if let Err(_e) = ctx.set_contents(b.link.clone()) {
        Err(Error::ClipboardWrite)
    } else {
        Ok(())
    }
}

pub fn refresh_bookmarks(c: &mut Cursive) -> Result<()> {
    load_bookmarks(c)
}
