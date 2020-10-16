extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{ApplicationWindow, Builder, Button, Label,
    ListStore, TreeView, TreeViewColumn, CellRendererText};

use std::env::args;
use std::collections::HashMap;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use serde_json::Error;
use serde::{de};

// Get data
#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
struct Isotope {  // Example: Hydrogen
    atomic_number: String,  // "1"
    symbol: String,  // "H"
    mass_number: String,  // "1"
    relative_atomic_mass: String,  // "1.00782503223(9)"
    isotopic_composition: Option<String>,  // "0.999885(70)"
    standard_atomic_weight: String,  // "[1.00784,1.00811]"
    // notes: String,  // "m"
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
struct Spin {
    nucleus: String,
    elevel: String,  // "Elevel(keV)"
    spin: String,
    thalf: String,  // "T1/2"
}

fn serde_get<T: de::DeserializeOwned>(jsonstring: &str) -> Result<Vec<T>, Error> {
    let v: Vec<T> = serde_json::from_str(jsonstring)?;
    Ok(v)
}

// TREEVIEW
fn create_and_fill_model(isos: &Vec<Isotope>, spins: &Vec<Spin>) -> ListStore {
    // Creation of a model with two rows.
    let model = ListStore::new(&[String::static_type(); 6]);
    for (idx, iso) in isos.iter().enumerate() {
        model.insert_with_values(None, &[0, 1, 2, 3, 4, 5], &[
                &iso.mass_number,
                &iso.relative_atomic_mass,
                &iso.isotopic_composition,
                &iso.standard_atomic_weight,
                &spins[idx].spin,
                &spins[idx].thalf,
                // &spins[idx].elevel,
            ]);
    }

    model
}

fn append_column(tree: &TreeView, id: i32, title: &str) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    column.set_min_width(100);
    column.set_resizable(true);
    column.set_title(title);

    // Association of the view's column with the model's `id` column.
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

fn get_button(symbol: &str, builder: &Builder) -> Button {
    builder
        .get_object(&["button_", symbol].join(""))
        .expect(&["Cannot find ", symbol].join(""))
}

fn get_button_map(builder: &Builder) -> HashMap<String, Button> {
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

    let mut button_map: HashMap<String, Button> = HashMap::with_capacity(118);

    for el in elements.iter() {
        button_map.insert(el.to_string(), get_button(el, builder));
    };

    button_map
}

fn get_isotopes(
    symbol: &str,
    data: &Result<Vec<Isotope>, Error>) -> Vec<Isotope> {
        let mut isotopes: Vec<Isotope> = Vec::new();

        for i in data.as_ref().unwrap().iter() {
            if i.symbol == symbol {
                isotopes.push(i.clone());
            }
        }
        isotopes
}

fn get_spins(
    symbol: &str,
    my_isos: &Vec<Isotope>,
    spin_data: &Result<Vec<Spin>, Error>) -> Vec<Spin>
    {

        let mut spins: Vec<Spin> = Vec::new();
        // Get mass numbers
        for iso in my_isos.iter() {
            let my_nucleus: String = [iso.mass_number.clone(), symbol.to_ascii_uppercase()].join("");
            let mut found = false;

            for i in spin_data.as_ref().unwrap().iter() {
                if i.nucleus == my_nucleus {
                    found = true;
                    spins.push(i.clone());
                    break;
                }
            }

            if !found {
                let void_spin = Spin {
                    nucleus: String::from("/"),
                    elevel: String::from("/"),  // "Elevel(keV)"
                    spin: String::from("/"),
                    thalf: String::from("/"),
                };

                spins.push(void_spin);
            }
        }

        spins
    }

fn build_ui(application: &gtk::Application) {
    // Get data
    let isotopes_data: Result<Vec<Isotope>, Error> = serde::export::Ok(serde_get(
        include_str!("common_isotopes.min.json")
    ).unwrap());

    let spin_data: Result<Vec<Spin>, Error> = serde::export::Ok(serde_get(
        include_str!("spins.json")
    ).unwrap());

    let builder = Builder::from_string(include_str!("gotosi.glade"));

    // Widgets
    let win: ApplicationWindow =
        builder
            .get_object("application_window")
            .expect("Can't build win from Glade");

    win.set_title("gElements");
    win.set_application(Some(application));

    // Static UI elements
    let label_symbol: Label = builder.get_object("label_symbol")
        .expect("Cannot find label");
    let label_atomic_number: Label = builder.get_object("label_atomic_number")
        .expect("Cannot find label_atomic_number");

    // Build Tree
    let treeview: TreeView = builder.get_object("treeview").expect("Cannot find treeview");

    // From isotope data
    append_column(&treeview, 0, "Mass Number");
    append_column(&treeview, 1, "% Rel. Atomic Mass");
    append_column(&treeview, 2, "Isotopic Composition");
    append_column(&treeview, 3, "Std. Atomic Weight");
    // From spin data
    append_column(&treeview, 4, "Nuclear Spin");
    append_column(&treeview, 5, "T_1/2");
    // append_column(&treeview, 6, "Elevel");

    treeview.set_headers_visible(true);

    // Collect element buttons in HashMap
    let button_map: HashMap<String, Button> = get_button_map(&builder);

    for (symbol, btn) in button_map {
        // Get relative isotopes
        let isos: Vec<Isotope> = get_isotopes(&symbol, &isotopes_data);
        let spins: Vec<Spin> = get_spins(&symbol, &isos, &spin_data);

        // Clone vars to pass them in connect_clicked:
        // https://gtk-rs.org/docs-src/tutorial/closures
        let lbl = label_symbol.clone();  // Symbol lbl
        let lbl_an = label_atomic_number.clone();  // Atomic Number lbl
        let tree = treeview.clone();

        // On clicked button
        btn.connect_clicked(move |_| {
            // Change labels
            lbl.set_text(&symbol);
            lbl_an.set_text(&isos[0].atomic_number);

            // Change treeview
            let model = create_and_fill_model(&isos, &spins);
            tree.set_model(Some(&model));
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
