extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{ApplicationWindow, Builder, Button};

use std::env::args;
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Isotope {  // Example: Hydrogen
    atomic_number: String,  // "1"
    symbol: String,  // "H"
    mass_number: String,  // "1"
    relative_atomic_mass: String,  // "1.00782503223(9)"
    isotopic_composition: String,  // "0.999885(70)"
    standard_atomic_weight: String,  // "[1.00784,1.00811]"
    notes: String,  // "m"
}

// Serde
fn get_data() -> Result<(), Error> {
    let data = include_str!("all_isotopes.min.json");

    //let v: Vec<Foo> = serde_json::from_str(data)?;
    let v: Vec<Isotope> = serde_json::from_str(data)?;

    for elem in v.iter() {
        println!("{:?}", elem);
    }
    Ok(())
}

// Funzioni ausiliarie
fn get_button(symbol: &str, builder: &Builder) -> Button {
    builder
        .get_object(&["button_", symbol].join(""))  // Is there a more elegant way?
        .expect(&["Cannot find ", symbol].join(""))
}

fn get_button_map(builder: &Builder) -> HashMap<&str, Button> {
    let elements: [&str; 118] = [
        "H", "Li", "Na", "K", "Rb", "Cs", "Fr",
        "Be", "Mg", "Ca", "Sr", "Ba", "Ra",
        "Sc", "Y",
        "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb", "Dy", "Ho", "Er", "Tm", "Yb", "Lu",
        "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk", "Cf", "Es", "Fm", "Md", "No", "Lr",
        "Ti", "Zr", "Hf", "Rf",
        "V", "Nb", "Ta", "Db",
        "Cr", "Mo", "W", "Sg",
        "Mn", "Tc", "Re", "Bh",
        "Fe", "Ru", "Os", "Hs",
        "Co", "Rh", "Ir", "Mt",
        "Ni", "Pd", "Pt", "Ds",
        "Cu", "Ag", "Au", "Rg",
        "Zn", "Cd", "Hg", "Cn",
        "B", "Al", "Ga", "In", "Tl", "Nh",
        "C", "Si", "Ge", "Sn", "Pb", "Fl",
        "N", "P", "As", "Sb", "Bi", "Mc",
        "O", "S", "Se", "Te", "Po", "Lv",
        "F", "Cl", "Br", "I", "At", "Ts",
        "He", "Ne", "Ar", "Kr", "Xe", "Rn", "Og",
    ];

    let mut button_map: HashMap<&str, Button> = HashMap::with_capacity(118);

    for el in elements.iter() {
        button_map.insert(el, get_button(el, builder));
    };

    button_map
}

// fn display_el() {}

// GUI

fn build_ui(application: &gtk::Application) {
    let _foo: Result<(), Error> = serde::export::Ok(get_data().unwrap());

    let builder = Builder::from_string(include_str!("gelements.glade"));

    // Widgets
    let win: ApplicationWindow =
        builder
            .get_object("application_window")
            .expect("Can't build win from Glade");

    win.set_title("gElements");
    win.set_application(Some(application));

    // Collect element buttons in HashMap
    let button_map: HashMap<&str, Button> = get_button_map(&builder);

    // Associate a func to the buttons
    for btn in button_map.values() {
        // Clone vars to pass them in here:
        // https://gtk-rs.org/docs-src/tutorial/closures
        btn.connect_clicked(move |btn| {
            println!{"{}", btn.get_label().unwrap()};  // Can return the `el`
        });
    }


    // Make all the widgets within the UI visible.
    win.show_all();
}

fn main() {

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let application =
        gtk::Application::new(
            Some("com.github.gtk-rs.examples.basic"),
            Default::default(),
        )
        .expect("Failed to start application");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
