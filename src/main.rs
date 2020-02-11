extern crate gtk;
extern crate gio;
extern crate gdk;
extern crate rand;

use gtk::prelude::*;
use gio::prelude::*;

use std::borrow::Borrow;

use gtk::{ApplicationWindow, WindowPosition};
use gdk::enums::key;
use std::cell::RefCell;
use std::rc::Rc;

mod game;

const STYLE: &str = "
#btn {
    font-weight: bold;
    font-size: 30px;
}

#hidden {
    opacity: 0;
}
";

macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}


fn show_end_of_game_info(window: ApplicationWindow) {
    let app = window.get_application().unwrap();
    let info_window = ApplicationWindow::new(&app);
    info_window.set_title("Koniec gry!");
    info_window.set_position(WindowPosition::Center);
    info_window.set_size_request(200, 200);
    let label = gtk::Label::new(None);
    label.set_text("Koniec gry!");
    info_window.add(&label);
    info_window.show_all();
}


fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    window.set_title("2048");
    window.set_position(WindowPosition::Center);
    window.set_size_request(400, 400);
    let mut blocks = Vec::new();
    game::generate_board(&mut blocks);
    let board: gtk::Grid = gtk::Grid::new();
    board.set_column_homogeneous(true);
    board.set_row_homogeneous(true);

    for i in 0..16 {
        let x: i32 = i / 4;
        let y: i32 = i % 4;
        board.attach(&blocks[x as usize][y as usize].button, x, y, 1, 1);
    }
    game::update_board(&mut blocks);

    let blocks_ = Rc::new(RefCell::new(blocks));
    window.borrow().connect_key_press_event(clone!(blocks_ => move |window, gdk| {
        let mut board = blocks_.borrow_mut();

        match gdk.get_keyval() {
            key::Up => {
                game::move_up(&mut board);
            }
            key::Down => {
                game::move_down(&mut board);
            }
            key::Left => {
                game::move_left(&mut board);
            }
            key::Right => {
                game::move_right(&mut board);
            }
            _ => {}
        }
        if game::end_of_game(&mut board) {
            show_end_of_game_info(window.clone());
        }
        Inhibit(false)
    }));

    window.add(&board);
    window.show_all();
}


fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.menu_bar"),
        Default::default(),
    ).expect("Initialization failed...");

    application.connect_activate(|app| {
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        build_ui(app);
    });

    application.run(&[]);
}