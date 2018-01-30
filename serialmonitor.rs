#![feature(proc_macro)]

extern crate gdk;
extern crate gtk;
#[macro_use]
extern crate relm;
extern crate relm_attributes;
#[macro_use]
extern crate relm_derive;

// use gdk::prelude::*;
use gtk::prelude::*;
use relm::{Relm, Update, Widget};
use gtk::Window;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct Model {}

pub struct Win {
    window: Window,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;
    fn model(_relm: &Relm<Self>, _param: Self::ModelParam) -> Model {
        Model {}
    }
    fn update(&mut self, event: Self::Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}
impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(_relm: &Relm<Self>, _model: Self::Model) -> Self {
        let builder = gtk::Builder::new();
        let _ = builder.add_from_string(include_str!("main.glade"));

        let window: gtk::Window = builder.get_object("window").unwrap();

        window.show_all();

        Win { window }
    }
}

fn main() {
    Win::run(()).unwrap();
}