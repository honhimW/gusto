//! Crossterm key & mouse event assertion helper.
//! n: none modifier
//! c: control modifier
//! s: shift modifier
//! a: alt modifier

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub trait KeyHelper {
    fn is_n_char(&self, c: char) -> bool;
    fn is_c_char(&self, c: char) -> bool;
    fn is_s_char(&self, c: char) -> bool;
    fn is_a_char(&self, c: char) -> bool;
    fn is_cs_char(&self, c: char) -> bool;
    fn is_ca_char(&self, c: char) -> bool;
    fn is_sa_char(&self, c: char) -> bool;
    fn is_csa_char(&self, c: char) -> bool;

    fn is_n_fn(&self, num: u8) -> bool;
    fn is_c_fn(&self, num: u8) -> bool;
    fn is_s_fn(&self, num: u8) -> bool;
    fn is_a_fn(&self, num: u8) -> bool;
    fn is_cs_fn(&self, num: u8) -> bool;
    fn is_ca_fn(&self, num: u8) -> bool;
    fn is_sa_fn(&self, num: u8) -> bool;
    fn is_csa_fn(&self, num: u8) -> bool;

    fn is_n_backspace(&self) -> bool;
    fn is_c_backspace(&self) -> bool;
    fn is_s_backspace(&self) -> bool;
    fn is_a_backspace(&self) -> bool;
    fn is_cs_backspace(&self) -> bool;
    fn is_ca_backspace(&self) -> bool;
    fn is_sa_backspace(&self) -> bool;
    fn is_csa_backspace(&self) -> bool;

    fn is_n_enter(&self) -> bool;
    fn is_c_enter(&self) -> bool;
    fn is_s_enter(&self) -> bool;
    fn is_a_enter(&self) -> bool;
    fn is_cs_enter(&self) -> bool;
    fn is_ca_enter(&self) -> bool;
    fn is_sa_enter(&self) -> bool;
    fn is_csa_enter(&self) -> bool;

    fn is_n_left(&self) -> bool;
    fn is_c_left(&self) -> bool;
    fn is_s_left(&self) -> bool;
    fn is_a_left(&self) -> bool;
    fn is_cs_left(&self) -> bool;
    fn is_ca_left(&self) -> bool;
    fn is_sa_left(&self) -> bool;
    fn is_csa_left(&self) -> bool;

    fn is_n_right(&self) -> bool;
    fn is_c_right(&self) -> bool;
    fn is_s_right(&self) -> bool;
    fn is_a_right(&self) -> bool;
    fn is_cs_right(&self) -> bool;
    fn is_ca_right(&self) -> bool;
    fn is_sa_right(&self) -> bool;
    fn is_csa_right(&self) -> bool;

    fn is_n_up(&self) -> bool;
    fn is_c_up(&self) -> bool;
    fn is_s_up(&self) -> bool;
    fn is_a_up(&self) -> bool;
    fn is_cs_up(&self) -> bool;
    fn is_ca_up(&self) -> bool;
    fn is_sa_up(&self) -> bool;
    fn is_csa_up(&self) -> bool;

    fn is_n_down(&self) -> bool;
    fn is_c_down(&self) -> bool;
    fn is_s_down(&self) -> bool;
    fn is_a_down(&self) -> bool;
    fn is_cs_down(&self) -> bool;
    fn is_ca_down(&self) -> bool;
    fn is_sa_down(&self) -> bool;
    fn is_csa_down(&self) -> bool;

    fn is_n_home(&self) -> bool;
    fn is_c_home(&self) -> bool;
    fn is_s_home(&self) -> bool;
    fn is_a_home(&self) -> bool;
    fn is_cs_home(&self) -> bool;
    fn is_ca_home(&self) -> bool;
    fn is_sa_home(&self) -> bool;
    fn is_csa_home(&self) -> bool;

    fn is_n_end(&self) -> bool;
    fn is_c_end(&self) -> bool;
    fn is_s_end(&self) -> bool;
    fn is_a_end(&self) -> bool;
    fn is_cs_end(&self) -> bool;
    fn is_ca_end(&self) -> bool;
    fn is_sa_end(&self) -> bool;
    fn is_csa_end(&self) -> bool;

    fn is_n_page_up(&self) -> bool;
    fn is_c_page_up(&self) -> bool;
    fn is_s_page_up(&self) -> bool;
    fn is_a_page_up(&self) -> bool;
    fn is_cs_page_up(&self) -> bool;
    fn is_ca_page_up(&self) -> bool;
    fn is_sa_page_up(&self) -> bool;
    fn is_csa_page_up(&self) -> bool;

    fn is_n_page_down(&self) -> bool;
    fn is_c_page_down(&self) -> bool;
    fn is_s_page_down(&self) -> bool;
    fn is_a_page_down(&self) -> bool;
    fn is_cs_page_down(&self) -> bool;
    fn is_ca_page_down(&self) -> bool;
    fn is_sa_page_down(&self) -> bool;
    fn is_csa_page_down(&self) -> bool;

    fn is_n_tab(&self) -> bool;
    fn is_c_tab(&self) -> bool;
    fn is_s_tab(&self) -> bool;
    fn is_a_tab(&self) -> bool;
    fn is_cs_tab(&self) -> bool;
    fn is_ca_tab(&self) -> bool;
    fn is_sa_tab(&self) -> bool;
    fn is_csa_tab(&self) -> bool;

    fn is_n_back_tab(&self) -> bool;
    fn is_c_back_tab(&self) -> bool;
    fn is_s_back_tab(&self) -> bool;
    fn is_a_back_tab(&self) -> bool;
    fn is_cs_back_tab(&self) -> bool;
    fn is_ca_back_tab(&self) -> bool;
    fn is_sa_back_tab(&self) -> bool;
    fn is_csa_back_tab(&self) -> bool;

    fn is_n_delete(&self) -> bool;
    fn is_c_delete(&self) -> bool;
    fn is_s_delete(&self) -> bool;
    fn is_a_delete(&self) -> bool;
    fn is_cs_delete(&self) -> bool;
    fn is_ca_delete(&self) -> bool;
    fn is_sa_delete(&self) -> bool;
    fn is_csa_delete(&self) -> bool;

    fn is_n_insert(&self) -> bool;
    fn is_c_insert(&self) -> bool;
    fn is_s_insert(&self) -> bool;
    fn is_a_insert(&self) -> bool;
    fn is_cs_insert(&self) -> bool;
    fn is_ca_insert(&self) -> bool;
    fn is_sa_insert(&self) -> bool;
    fn is_csa_insert(&self) -> bool;

    fn is_n_esc(&self) -> bool;
    fn is_c_esc(&self) -> bool;
    fn is_s_esc(&self) -> bool;
    fn is_a_esc(&self) -> bool;
    fn is_cs_esc(&self) -> bool;
    fn is_ca_esc(&self) -> bool;
    fn is_sa_esc(&self) -> bool;
    fn is_csa_esc(&self) -> bool;
}

impl KeyHelper for KeyEvent {
    fn is_n_char(&self, c: char) -> bool {
        self.modifiers == KeyModifiers::NONE && if let KeyCode::Char(c0) = self.code && c == c0 { true } else { false }
    }

    fn is_c_char(&self, c: char) -> bool {
        self.modifiers == KeyModifiers::CONTROL && if let KeyCode::Char(c0) = self.code && c == c0 { true } else { false }
    }

    fn is_s_char(&self, c: char) -> bool {
        self.modifiers == KeyModifiers::SHIFT && if let KeyCode::Char(c0) = self.code && c.eq_ignore_ascii_case(&c0) { true } else { false }
    }

    fn is_a_char(&self, c: char) -> bool {
        self.modifiers == KeyModifiers::ALT && if let KeyCode::Char(c0) = self.code && c == c0 { true } else { false }
    }

    fn is_cs_char(&self, c: char) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && if let KeyCode::Char(c0) = self.code && c.eq_ignore_ascii_case(&c0) { true } else { false }
    }

    fn is_ca_char(&self, c: char) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && if let KeyCode::Char(c0) = self.code && c == c0 { true } else { false }
    }

    fn is_sa_char(&self, c: char) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && if let KeyCode::Char(c0) = self.code && c.eq_ignore_ascii_case(&c0) { true } else { false }
    }

    fn is_csa_char(&self, c: char) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && if let KeyCode::Char(c0) = self.code && c.eq_ignore_ascii_case(&c0) { true } else { false }
    }


    fn is_n_fn(&self, num: u8) -> bool {
        self.modifiers == KeyModifiers::NONE && if let KeyCode::F(f) = self.code && num == f { true } else { false }
    }

    fn is_c_fn(&self, num: u8) -> bool {
        self.modifiers == KeyModifiers::CONTROL && if let KeyCode::F(f) = self.code && num == f { true } else { false }
    }

    fn is_s_fn(&self, num: u8) -> bool {
        self.modifiers == KeyModifiers::SHIFT && if let KeyCode::F(f) = self.code && num == f { true } else { false }
    }

    fn is_a_fn(&self, num: u8) -> bool {
        self.modifiers == KeyModifiers::ALT && if let KeyCode::F(f) = self.code && num == f { true } else { false }
    }

    fn is_cs_fn(&self, num: u8) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && if let KeyCode::F(f) = self.code && num == f { true } else { false }
    }

    fn is_ca_fn(&self, num: u8) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && if let KeyCode::F(f) = self.code && num == f { true } else { false }
    }

    fn is_sa_fn(&self, num: u8) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && if let KeyCode::F(f) = self.code && num == f { true } else { false }
    }

    fn is_csa_fn(&self, num: u8) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && if let KeyCode::F(f) = self.code && num == f { true } else { false }
    }


    fn is_n_backspace(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::Backspace
    }

    fn is_c_backspace(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::Backspace
    }

    fn is_s_backspace(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::Backspace
    }

    fn is_a_backspace(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::Backspace
    }

    fn is_cs_backspace(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::Backspace
    }

    fn is_ca_backspace(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::Backspace
    }

    fn is_sa_backspace(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Backspace
    }

    fn is_csa_backspace(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Backspace
    }


    fn is_n_enter(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::Enter
    }

    fn is_c_enter(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::Enter
    }

    fn is_s_enter(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::Enter
    }

    fn is_a_enter(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::Enter
    }

    fn is_cs_enter(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::Enter
    }

    fn is_ca_enter(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::Enter
    }

    fn is_sa_enter(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Enter
    }

    fn is_csa_enter(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Enter
    }


    fn is_n_left(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::Left
    }

    fn is_c_left(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::Left
    }

    fn is_s_left(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::Left
    }

    fn is_a_left(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::Left
    }

    fn is_cs_left(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::Left
    }

    fn is_ca_left(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::Left
    }

    fn is_sa_left(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Left
    }

    fn is_csa_left(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Left
    }


    fn is_n_right(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::Right
    }

    fn is_c_right(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::Right
    }

    fn is_s_right(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::Right
    }

    fn is_a_right(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::Right
    }

    fn is_cs_right(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::Right
    }

    fn is_ca_right(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::Right
    }

    fn is_sa_right(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Right
    }

    fn is_csa_right(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Right
    }

    fn is_n_up(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::Up
    }

    fn is_c_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::Up
    }

    fn is_s_up(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::Up
    }

    fn is_a_up(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::Up
    }

    fn is_cs_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::Up
    }

    fn is_ca_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::Up
    }

    fn is_sa_up(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Up
    }

    fn is_csa_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Up
    }

    fn is_n_down(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::Down
    }

    fn is_c_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::Down
    }

    fn is_s_down(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::Down
    }

    fn is_a_down(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::Down
    }

    fn is_cs_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::Down
    }

    fn is_ca_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::Down
    }

    fn is_sa_down(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Down
    }

    fn is_csa_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Down
    }

    fn is_n_home(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::Home
    }

    fn is_c_home(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::Home
    }

    fn is_s_home(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::Home
    }

    fn is_a_home(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::Home
    }

    fn is_cs_home(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::Home
    }

    fn is_ca_home(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::Home
    }

    fn is_sa_home(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Home
    }

    fn is_csa_home(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Home
    }

    fn is_n_end(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::End
    }

    fn is_c_end(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::End
    }

    fn is_s_end(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::End
    }

    fn is_a_end(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::End
    }

    fn is_cs_end(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::End
    }

    fn is_ca_end(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::End
    }

    fn is_sa_end(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::End
    }

    fn is_csa_end(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::End
    }

    fn is_n_page_up(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::PageUp
    }

    fn is_c_page_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::PageUp
    }

    fn is_s_page_up(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::PageUp
    }

    fn is_a_page_up(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::PageUp
    }

    fn is_cs_page_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::PageUp
    }

    fn is_ca_page_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::PageUp
    }

    fn is_sa_page_up(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::PageUp
    }

    fn is_csa_page_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::PageUp
    }

    fn is_n_page_down(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::PageDown
    }

    fn is_c_page_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::PageDown
    }

    fn is_s_page_down(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::PageDown
    }

    fn is_a_page_down(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::PageDown
    }

    fn is_cs_page_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::PageDown
    }

    fn is_ca_page_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::PageDown
    }

    fn is_sa_page_down(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::PageDown
    }

    fn is_csa_page_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::PageDown
    }

    fn is_n_tab(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::Tab
    }

    fn is_c_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::Tab
    }

    fn is_s_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::Tab
    }

    fn is_a_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::Tab
    }

    fn is_cs_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::Tab
    }

    fn is_ca_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::Tab
    }

    fn is_sa_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Tab
    }

    fn is_csa_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Tab
    }

    fn is_n_back_tab(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::BackTab
    }

    fn is_c_back_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::BackTab
    }

    fn is_s_back_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::BackTab
    }

    fn is_a_back_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::BackTab
    }

    fn is_cs_back_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::BackTab
    }

    fn is_ca_back_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::BackTab
    }

    fn is_sa_back_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::BackTab
    }

    fn is_csa_back_tab(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::BackTab
    }

    fn is_n_delete(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::Delete
    }

    fn is_c_delete(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::Delete
    }

    fn is_s_delete(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::Delete
    }

    fn is_a_delete(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::Delete
    }

    fn is_cs_delete(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::Delete
    }

    fn is_ca_delete(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::Delete
    }

    fn is_sa_delete(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Delete
    }

    fn is_csa_delete(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Delete
    }

    fn is_n_insert(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::Insert
    }

    fn is_c_insert(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::Insert
    }

    fn is_s_insert(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::Insert
    }

    fn is_a_insert(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::Insert
    }

    fn is_cs_insert(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::Insert
    }

    fn is_ca_insert(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::Insert
    }

    fn is_sa_insert(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Insert
    }

    fn is_csa_insert(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Insert
    }

    fn is_n_esc(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && self.code == KeyCode::Esc
    }

    fn is_c_esc(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL) && self.code == KeyCode::Esc
    }

    fn is_s_esc(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT) && self.code == KeyCode::Esc
    }

    fn is_a_esc(&self) -> bool {
        self.modifiers == (KeyModifiers::ALT) && self.code == KeyCode::Esc
    }

    fn is_cs_esc(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && self.code == KeyCode::Esc
    }

    fn is_ca_esc(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && self.code == KeyCode::Esc
    }

    fn is_sa_esc(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Esc
    }

    fn is_csa_esc(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && self.code == KeyCode::Esc
    }
}

use ratatui::crossterm::event::MouseEventKind::*;
use ratatui::crossterm::event::{MouseButton, MouseEvent};
use MouseButton::{Left, Middle, Right};
use ratatui::layout::{Position, Rect};

#[allow(unused)]
pub trait MouseHelper {
    fn as_position(&self) -> Position;
    fn within(&self, rect: &Rect) -> bool;

    fn is_n_left_down(&self) -> bool;
    fn is_c_left_down(&self) -> bool;
    fn is_s_left_down(&self) -> bool;
    fn is_a_left_down(&self) -> bool;
    fn is_cs_left_down(&self) -> bool;
    fn is_ca_left_down(&self) -> bool;
    fn is_sa_left_down(&self) -> bool;
    fn is_csa_left_down(&self) -> bool;

    fn is_n_left_up(&self) -> bool;
    fn is_c_left_up(&self) -> bool;
    fn is_s_left_up(&self) -> bool;
    fn is_a_left_up(&self) -> bool;
    fn is_cs_left_up(&self) -> bool;
    fn is_ca_left_up(&self) -> bool;
    fn is_sa_left_up(&self) -> bool;
    fn is_csa_left_up(&self) -> bool;

    fn is_n_right_down(&self) -> bool;
    fn is_c_right_down(&self) -> bool;
    fn is_s_right_down(&self) -> bool;
    fn is_a_right_down(&self) -> bool;
    fn is_cs_right_down(&self) -> bool;
    fn is_ca_right_down(&self) -> bool;
    fn is_sa_right_down(&self) -> bool;
    fn is_csa_right_down(&self) -> bool;

    fn is_n_right_up(&self) -> bool;
    fn is_c_right_up(&self) -> bool;
    fn is_s_right_up(&self) -> bool;
    fn is_a_right_up(&self) -> bool;
    fn is_cs_right_up(&self) -> bool;
    fn is_ca_right_up(&self) -> bool;
    fn is_sa_right_up(&self) -> bool;
    fn is_csa_right_up(&self) -> bool;

    fn is_n_middle_down(&self) -> bool;
    fn is_c_middle_down(&self) -> bool;
    fn is_s_middle_down(&self) -> bool;
    fn is_a_middle_down(&self) -> bool;
    fn is_cs_middle_down(&self) -> bool;
    fn is_ca_middle_down(&self) -> bool;
    fn is_sa_middle_down(&self) -> bool;
    fn is_csa_middle_down(&self) -> bool;

    fn is_n_middle_up(&self) -> bool;
    fn is_c_middle_up(&self) -> bool;
    fn is_s_middle_up(&self) -> bool;
    fn is_a_middle_up(&self) -> bool;
    fn is_cs_middle_up(&self) -> bool;
    fn is_ca_middle_up(&self) -> bool;
    fn is_sa_middle_up(&self) -> bool;
    fn is_csa_middle_up(&self) -> bool;

    fn is_n_left_drag(&self) -> bool;
    fn is_c_left_drag(&self) -> bool;
    fn is_s_left_drag(&self) -> bool;
    fn is_a_left_drag(&self) -> bool;
    fn is_cs_left_drag(&self) -> bool;
    fn is_ca_left_drag(&self) -> bool;
    fn is_sa_left_drag(&self) -> bool;
    fn is_csa_left_drag(&self) -> bool;

    fn is_n_right_drag(&self) -> bool;
    fn is_c_right_drag(&self) -> bool;
    fn is_s_right_drag(&self) -> bool;
    fn is_a_right_drag(&self) -> bool;
    fn is_cs_right_drag(&self) -> bool;
    fn is_ca_right_drag(&self) -> bool;
    fn is_sa_right_drag(&self) -> bool;
    fn is_csa_right_drag(&self) -> bool;

    fn is_n_middle_drag(&self) -> bool;
    fn is_c_middle_drag(&self) -> bool;
    fn is_s_middle_drag(&self) -> bool;
    fn is_a_middle_drag(&self) -> bool;
    fn is_cs_middle_drag(&self) -> bool;
    fn is_ca_middle_drag(&self) -> bool;
    fn is_sa_middle_drag(&self) -> bool;
    fn is_csa_middle_drag(&self) -> bool;

    fn is_n_moved(&self) -> bool;
    fn is_c_moved(&self) -> bool;
    fn is_s_moved(&self) -> bool;
    fn is_a_moved(&self) -> bool;
    fn is_cs_moved(&self) -> bool;
    fn is_ca_moved(&self) -> bool;
    fn is_sa_moved(&self) -> bool;
    fn is_csa_moved(&self) -> bool;

    fn is_n_scroll_down(&self) -> bool;
    fn is_c_scroll_down(&self) -> bool;
    fn is_s_scroll_down(&self) -> bool;
    fn is_a_scroll_down(&self) -> bool;
    fn is_cs_scroll_down(&self) -> bool;
    fn is_ca_scroll_down(&self) -> bool;
    fn is_sa_scroll_down(&self) -> bool;
    fn is_csa_scroll_down(&self) -> bool;

    fn is_n_scroll_up(&self) -> bool;
    fn is_c_scroll_up(&self) -> bool;
    fn is_s_scroll_up(&self) -> bool;
    fn is_a_scroll_up(&self) -> bool;
    fn is_cs_scroll_up(&self) -> bool;
    fn is_ca_scroll_up(&self) -> bool;
    fn is_sa_scroll_up(&self) -> bool;
    fn is_csa_scroll_up(&self) -> bool;

    fn is_n_scroll_left(&self) -> bool;
    fn is_c_scroll_left(&self) -> bool;
    fn is_s_scroll_left(&self) -> bool;
    fn is_a_scroll_left(&self) -> bool;
    fn is_cs_scroll_left(&self) -> bool;
    fn is_ca_scroll_left(&self) -> bool;
    fn is_sa_scroll_left(&self) -> bool;
    fn is_csa_scroll_left(&self) -> bool;

    fn is_n_scroll_right(&self) -> bool;
    fn is_c_scroll_right(&self) -> bool;
    fn is_s_scroll_right(&self) -> bool;
    fn is_a_scroll_right(&self) -> bool;
    fn is_cs_scroll_right(&self) -> bool;
    fn is_ca_scroll_right(&self) -> bool;
    fn is_sa_scroll_right(&self) -> bool;
    fn is_csa_scroll_right(&self) -> bool;
}

impl MouseHelper for MouseEvent {

    fn as_position(&self) -> Position {
        Position {
            x: self.column,
            y: self.row,
        }
    }

    fn within(&self, rect: &Rect) -> bool {
        rect.contains(self.as_position())
    }

    fn is_n_left_down(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, Down(Left))
    }

    fn is_c_left_down(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, Down(Left))
    }

    fn is_s_left_down(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, Down(Left))
    }

    fn is_a_left_down(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, Down(Left))
    }

    fn is_cs_left_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, Down(Left))
    }

    fn is_ca_left_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, Down(Left))
    }

    fn is_sa_left_down(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Down(Left))
    }

    fn is_csa_left_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Down(Left))
    }

    fn is_n_left_up(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, Up(Left))
    }

    fn is_c_left_up(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, Up(Left))
    }

    fn is_s_left_up(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, Up(Left))
    }

    fn is_a_left_up(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, Up(Left))
    }

    fn is_cs_left_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, Up(Left))
    }

    fn is_ca_left_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, Up(Left))
    }

    fn is_sa_left_up(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Up(Left))
    }

    fn is_csa_left_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Up(Left))
    }

    fn is_n_right_down(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, Down(Right))
    }

    fn is_c_right_down(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, Down(Right))
    }

    fn is_s_right_down(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, Down(Right))
    }

    fn is_a_right_down(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, Down(Right))
    }

    fn is_cs_right_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, Down(Right))
    }

    fn is_ca_right_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, Down(Right))
    }

    fn is_sa_right_down(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Down(Right))
    }

    fn is_csa_right_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Down(Right))
    }

    fn is_n_right_up(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, Up(Right))
    }

    fn is_c_right_up(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, Up(Right))
    }

    fn is_s_right_up(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, Up(Right))
    }

    fn is_a_right_up(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, Up(Right))
    }

    fn is_cs_right_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, Up(Right))
    }

    fn is_ca_right_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, Up(Right))
    }

    fn is_sa_right_up(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Up(Right))
    }

    fn is_csa_right_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Up(Right))
    }

    fn is_n_middle_down(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, Down(Middle))
    }

    fn is_c_middle_down(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, Down(Middle))
    }

    fn is_s_middle_down(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, Down(Middle))
    }

    fn is_a_middle_down(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, Down(Middle))
    }

    fn is_cs_middle_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, Down(Middle))
    }

    fn is_ca_middle_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, Down(Middle))
    }

    fn is_sa_middle_down(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Down(Middle))
    }

    fn is_csa_middle_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Down(Middle))
    }

    fn is_n_middle_up(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, Up(Middle))
    }

    fn is_c_middle_up(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, Up(Middle))
    }

    fn is_s_middle_up(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, Up(Middle))
    }

    fn is_a_middle_up(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, Up(Middle))
    }

    fn is_cs_middle_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, Up(Middle))
    }

    fn is_ca_middle_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, Up(Middle))
    }

    fn is_sa_middle_up(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Up(Middle))
    }

    fn is_csa_middle_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Up(Middle))
    }

    fn is_n_left_drag(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, Drag(Left))
    }

    fn is_c_left_drag(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, Drag(Left))
    }

    fn is_s_left_drag(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, Drag(Left))
    }

    fn is_a_left_drag(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, Drag(Left))
    }

    fn is_cs_left_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, Drag(Left))
    }

    fn is_ca_left_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, Drag(Left))
    }

    fn is_sa_left_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Drag(Left))
    }

    fn is_csa_left_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Drag(Left))
    }

    fn is_n_right_drag(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, Drag(Right))
    }

    fn is_c_right_drag(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, Drag(Right))
    }

    fn is_s_right_drag(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, Drag(Right))
    }

    fn is_a_right_drag(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, Drag(Right))
    }

    fn is_cs_right_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, Drag(Right))
    }

    fn is_ca_right_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, Drag(Right))
    }

    fn is_sa_right_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Drag(Right))
    }

    fn is_csa_right_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Drag(Right))
    }

    fn is_n_middle_drag(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, Drag(Middle))
    }

    fn is_c_middle_drag(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, Drag(Middle))
    }

    fn is_s_middle_drag(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, Drag(Middle))
    }

    fn is_a_middle_drag(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, Drag(Middle))
    }

    fn is_cs_middle_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, Drag(Middle))
    }

    fn is_ca_middle_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, Drag(Middle))
    }

    fn is_sa_middle_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Drag(Middle))
    }

    fn is_csa_middle_drag(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Drag(Middle))
    }

    fn is_n_moved(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, Moved)
    }

    fn is_c_moved(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, Moved)
    }

    fn is_s_moved(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, Moved)
    }

    fn is_a_moved(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, Moved)
    }

    fn is_cs_moved(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, Moved)
    }

    fn is_ca_moved(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, Moved)
    }

    fn is_sa_moved(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Moved)
    }

    fn is_csa_moved(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, Moved)
    }

    fn is_n_scroll_down(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, ScrollDown)
    }

    fn is_c_scroll_down(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, ScrollDown)
    }

    fn is_s_scroll_down(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, ScrollDown)
    }

    fn is_a_scroll_down(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, ScrollDown)
    }

    fn is_cs_scroll_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, ScrollDown)
    }

    fn is_ca_scroll_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, ScrollDown)
    }

    fn is_sa_scroll_down(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, ScrollDown)
    }

    fn is_csa_scroll_down(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, ScrollDown)
    }

    fn is_n_scroll_up(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, ScrollUp)
    }

    fn is_c_scroll_up(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, ScrollUp)
    }

    fn is_s_scroll_up(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, ScrollUp)
    }

    fn is_a_scroll_up(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, ScrollUp)
    }

    fn is_cs_scroll_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, ScrollUp)
    }

    fn is_ca_scroll_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, ScrollUp)
    }

    fn is_sa_scroll_up(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, ScrollUp)
    }

    fn is_csa_scroll_up(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, ScrollUp)
    }

    fn is_n_scroll_left(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, ScrollLeft)
    }

    fn is_c_scroll_left(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, ScrollLeft)
    }

    fn is_s_scroll_left(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, ScrollLeft)
    }

    fn is_a_scroll_left(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, ScrollLeft)
    }

    fn is_cs_scroll_left(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, ScrollLeft)
    }

    fn is_ca_scroll_left(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, ScrollLeft)
    }

    fn is_sa_scroll_left(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, ScrollLeft)
    }

    fn is_csa_scroll_left(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, ScrollLeft)
    }

    fn is_n_scroll_right(&self) -> bool {
        self.modifiers == KeyModifiers::NONE && matches!(self.kind, ScrollRight)
    }

    fn is_c_scroll_right(&self) -> bool {
        self.modifiers == KeyModifiers::CONTROL && matches!(self.kind, ScrollRight)
    }

    fn is_s_scroll_right(&self) -> bool {
        self.modifiers == KeyModifiers::SHIFT && matches!(self.kind, ScrollRight)
    }

    fn is_a_scroll_right(&self) -> bool {
        self.modifiers == KeyModifiers::ALT && matches!(self.kind, ScrollRight)
    }

    fn is_cs_scroll_right(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT) && matches!(self.kind, ScrollRight)
    }

    fn is_ca_scroll_right(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::ALT) && matches!(self.kind, ScrollRight)
    }

    fn is_sa_scroll_right(&self) -> bool {
        self.modifiers == (KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, ScrollRight)
    }

    fn is_csa_scroll_right(&self) -> bool {
        self.modifiers == (KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT) && matches!(self.kind, ScrollRight)
    }
}

#[cfg(test)]
mod test {
    use ratatui::crossterm::event::{KeyEvent, MouseEventKind};
    use super::*;

    #[test]
    fn test_char_helper() {
        let event = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL);
        assert!(event.is_c_char('a'));
        let event = KeyEvent::new(KeyCode::Char('0'), KeyModifiers::empty());
        assert!(event.is_n_char('0'));
        let event = KeyEvent::new(KeyCode::Char('z'), KeyModifiers::CONTROL | KeyModifiers::SHIFT);
        assert!(event.is_cs_char('z'));
    }

    #[test]
    fn test_key_helper() {
        let event = KeyEvent::new(KeyCode::Left, KeyModifiers::SHIFT);
        assert!(event.is_s_left());
        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        assert!(event.is_n_enter());
        let event = KeyEvent::new(KeyCode::Home, KeyModifiers::CONTROL | KeyModifiers::SHIFT);
        assert!(event.is_cs_home());
        let event = KeyEvent::new(KeyCode::F(5), KeyModifiers::CONTROL);
        assert!(event.is_c_fn(5));
        let event = KeyEvent::new(KeyCode::Insert, KeyModifiers::CONTROL | KeyModifiers::SHIFT | KeyModifiers::ALT);
        assert!(event.is_csa_insert());
    }

    #[test]
    fn test_mouse_helper() {
        let event = MouseEvent {
            kind: MouseEventKind::Moved,
            column: 0,
            row: 0,
            modifiers: KeyModifiers::NONE,
        };
        assert!(event.is_n_moved());
        let event = MouseEvent {
            kind: MouseEventKind::Down(Left),
            column: 0,
            row: 0,
            modifiers: KeyModifiers::CONTROL,
        };
        assert!(event.is_c_left_down());
    }

}
