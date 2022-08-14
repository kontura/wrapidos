use gtk::gdk::Display;
use adw::prelude::*;

use adw::{Application, ApplicationWindow, HeaderBar};
use gtk::{Box, ListBox, Orientation, SelectionMode, CssProvider, StyleContext, Label};

mod curl_idos;
mod parse_idos;
mod entry_row_completion;

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
    let list_of_stations: std::rc::Rc<String> = std::rc::Rc::new(get_list_of_stations());

    let input_field_from = adw::EntryRow::builder()
        .title("From:")
        .text("Brno, Slovanské Náměstí")
        .build();

    let input_field_to = adw::EntryRow::builder()
        .title("To:")
        .text("Brno, Úvoz")
        .build();

    let input_field_time = adw::EntryRow::builder()
        .title("Time:")
        .input_purpose(gtk::InputPurpose::Digits)
        .build();

    let swap_button = gtk::Button::builder()
        .label("<-->")
        .halign(gtk::Align::Center)
        .build();

    let input_field_to_for_scope = input_field_to.clone();
    let input_field_from_copy = input_field_from.clone();
    swap_button.connect_clicked(move |_| {
        let tmp = input_field_to_for_scope.text();
        input_field_to_for_scope.set_text(&input_field_from_copy.text());
        input_field_from_copy.set_text(&tmp);
    });

    let search_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(24)
        .margin_end(24)
        .build();

    search_box.append(&input_field_from);
    search_box.append(&input_field_to);

    let button_cont = adw::ButtonContent::builder()
        .label("Search")
        .icon_name("system-search-symbolic")
        .build();

    // Create a button with label
    let button = gtk::Button::builder()
        .child(&button_cont)
        .halign(gtk::Align::Center)
        .build();

    let button_row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .margin_top(10)
        .halign(gtk::Align::Center)
        .build();

    button_row.append(&button);
    button_row.append(&swap_button);

    let routes_list = ListBox::builder()
        .margin_end(32)
        .margin_start(32)
        .margin_bottom(10)
        .selection_mode(SelectionMode::None)
        .css_classes(vec![String::from("boxed-list")])
        .build();
    let station_completion_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .visible(false)
        .show_separators(true)
        .build();
    let station_completion_list_current_clicked_handler_id: std::rc::Rc::<std::cell::RefCell<Option<gtk::glib::SignalHandlerId>>> = std::rc::Rc::new(std::cell::RefCell::new(None));
    for station in list_of_stations.lines() {
        let sl = Label::builder()
            .label(&station)
            .margin_top(10)
            .margin_bottom(10)
            .halign(gtk::Align::Start)
            .margin_start(10)
            .margin_end(10)
            .build();
        station_completion_list.append(&sl);
    }

    search_box.append(&station_completion_list);
    search_box.append(&input_field_time);
    search_box.append(&button_row);

    entry_row_completion::setup_completion_for_entry_row(input_field_to.clone(),
                                                         station_completion_list.clone(),
                                                         search_box.clone(),
                                                         station_completion_list_current_clicked_handler_id.clone());
    entry_row_completion::setup_completion_for_entry_row(input_field_from.clone(),
                                                         station_completion_list.clone(),
                                                         search_box.clone(),
                                                         station_completion_list_current_clicked_handler_id.clone());

    // Connect to "clicked" signal of `button`
    let list_box_copy = routes_list.clone();
    button.connect_clicked(move |_| {
        loop {
            let row = list_box_copy.row_at_index(0);
            match row {
                Some(row) => list_box_copy.remove(&row),
                None => break,
            }
        }
        let html = curl_idos::curl_idos(input_field_from.text().to_string(), input_field_to.text().to_string(), input_field_time.text().to_string());
        let vec_of_connections = parse_idos::parse_idos(&html);
        for route in &vec_of_connections {
            list_box_copy.append(&build_route(&route));
        }
    });

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&HeaderBar::new());
    content.append(&search_box);
    content.append(&routes_list);

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .propagate_natural_height(true)
        .child(&content)
        .build();

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("WrapIdos")
        .default_width(300)
        .default_height(600)
        .content(&scrolled_window)
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
            full_route_row.append(&gtk::Separator::builder()
                                  .opacity(0.3)
                                  .build());
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

fn get_list_of_stations() -> String {
    let stations = include_str!("zastavky.csv").to_string();
    diacritics::remove_diacritics(&stations).to_ascii_lowercase()
}
