use gtk::gdk::Display;
use adw::prelude::*;

use adw::{Application, ApplicationWindow, HeaderBar};
use gtk::{Box, ListBox, Orientation, SelectionMode, CssProvider, StyleContext, Label};
use gtk::glib::Type;

mod curl_idos;
mod parse_idos;

const APP_ID: &str = "org.gtk_rs.WrapIdos";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let search_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();

    // Create an EntryCompletion widget
    let from_stations = gtk::EntryCompletion::new();
    from_stations.set_text_column(0);
    from_stations.set_minimum_key_length(1);
    from_stations.set_popup_completion(true);

    let to_stations = gtk::EntryCompletion::new();
    to_stations.set_text_column(0);
    to_stations.set_minimum_key_length(1);
    to_stations.set_popup_completion(true);

    // Create a ListStore of items
    // These will be the source for the autocompletion
    // as the user types into the field
    // For a more evolved example of ListStore see src/bin/list_store.rs
    let ls_from = create_list_model();
    let ls_to = create_list_model();
    from_stations.set_model(Some(&ls_from));
    to_stations.set_model(Some(&ls_to));

    let input_field_from = gtk::Entry::new();
    input_field_from.set_completion(Some(&from_stations));
    input_field_from.set_margin_bottom(10);
    input_field_from.set_buffer(&gtk::EntryBuffer::new(Some("Brno,,Slovanské Náměstí")));

    let input_field_to = gtk::Entry::new();
    input_field_to.set_completion(Some(&to_stations));
    input_field_to.set_margin_bottom(10);
    input_field_to.set_buffer(&gtk::EntryBuffer::new(Some("Brno,,Úvoz")));

    let inputs = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .build();

    let from_line = Label::new(Some("From:"));
    from_line.set_halign(gtk::Align::Start);
    inputs.append(&from_line);
    // Create a title label
    inputs.append(&input_field_from);

    let between_inputs_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    let to_line = Label::new(Some("To:"));
    //to_line.set_halign(gtk::Align::Start);
    between_inputs_box.append(&to_line);

    let swap_button = gtk::Button::builder().label("<-->").margin_start(50).build();
    let input_field_to_copy = input_field_to.clone();
    let input_field_from_copy = input_field_from.clone();
    swap_button.connect_clicked(move |_| {
        let tmp = input_field_to_copy.buffer();
        input_field_to_copy.set_buffer(&input_field_from_copy.buffer());
        input_field_from_copy.set_buffer(&tmp);
    });

    between_inputs_box.append(&swap_button);
    inputs.append(&between_inputs_box);

    inputs.append(&input_field_to);

    search_box.append(&inputs);

    // Create a button with label
    let button = gtk::Button::builder()
        .label("Search!")
        .halign(gtk::Align::Center)
        .build();
    search_box.append(&button);

    let list_box = ListBox::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        .selection_mode(SelectionMode::None)
        // makes the list look nicer
        .css_classes(vec![String::from("boxed-list")])
        .build();


    // Connect to "clicked" signal of `button`
    let list_box_copy = list_box.clone();
    button.connect_clicked(move |_| {
        loop {
            let row = list_box_copy.row_at_index(0);
            match row {
                Some(row) => list_box_copy.remove(&row),
                None => break,
            }
        }
        let html = curl_idos::curl_idos(input_field_from.text().to_string(), input_field_to.text().to_string());
        let vec_of_connections = parse_idos::parse_idos(&html);
        for route in &vec_of_connections {
            list_box_copy.append(&build_route(&route));
        }
    });

    let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(&HeaderBar::new());

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .propagate_natural_height(true)
        .child(&list_box)
        .build();

    content.append(&search_box);
    content.append(&scrolled_window);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("WrapIdos")
        .default_width(300)
        .default_height(600)
        .content(&content)
        .build();

    // Present window
    window.present();
}

fn build_route(route: &Vec<parse_idos::Connection>) -> gtk::Box {
    let full_route_row = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(14)
        .margin_bottom(14)
        .margin_start(14)
        .margin_end(14)
        .build();
    //full_route_row.set_halign(gtk::Align::Start);
    let mut first: bool = true;
    for connection in route {
        if !first {
            full_route_row.append(&gtk::Separator::builder().build());
        }
        let connection_row = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(8)
            .margin_end(8)
            .vexpand_set(true)
            .build();
        //connection_row.set_halign(gtk::Align::Start);
        let name_label = Label::new(Some(&connection.name));
        let from_label = Label::new(Some(&[connection.departure_time.clone(), connection.departure_station.clone()].join(" ")));
        let to_label = Label::new(Some(&[connection.destination_time.clone(), connection.destination_station.clone()].join(" ")));
        name_label.set_halign(gtk::Align::Start);
        name_label.add_css_class("route_title");
        from_label.set_halign(gtk::Align::Start);
        to_label.set_halign(gtk::Align::Start);
        connection_row.append(&name_label);
        connection_row.append(&from_label);
        connection_row.append(&to_label);
        full_route_row.append(&connection_row);
        first = false;
    };

    return full_route_row;
}

fn create_list_model() -> gtk::ListStore {
    let col_types: [Type; 2] = [Type::STRING, Type::STRING];
    let stations = include_str!("zastavky.csv").lines();
    let store = gtk::ListStore::new(&col_types);
    for d in stations {
        let new_string = diacritics::remove_diacritics(d);
        store.set(&store.append(), &[(0, &new_string), (1, &d)]);
    }
    store
}
