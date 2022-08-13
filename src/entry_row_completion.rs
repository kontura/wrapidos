use adw::prelude::*;
use gtk::Label;

pub fn setup_completion_for_entry_row(erw: adw::EntryRow,
                                      com_list: gtk::ListBox,
                                      search_box: gtk::Box,
                                      signal_id: std::rc::Rc::<std::cell::RefCell<Option<gtk::glib::SignalHandlerId>>>) {
    let com_list_for_scope = com_list.clone();
    erw.connect_changed(move |_| {
        com_list_for_scope.invalidate_filter();
    });

    let erw_for_scope = erw.clone();
    erw.connect_state_flags_changed(move |me, previous_flags| {
        let flags = me.state_flags();
        let removed_flags = previous_flags - flags;
        let added_flags = flags - previous_flags;
        if removed_flags.contains(gtk::StateFlags::FOCUS_WITHIN) {
            com_list.set_visible(false);
        }
        if added_flags.contains(gtk::StateFlags::FOCUS_WITHIN) {
            search_box.reorder_child_after(&com_list, Some(me));
            com_list.set_visible(true);
            let input_field_to_for_scope2 = erw_for_scope.clone();
            com_list.set_filter_func(move |row| {
                let input = String::from(input_field_to_for_scope2.text()).to_ascii_lowercase();
                let row_text = row.child().unwrap().downcast_ref::<Label>().unwrap().text();
                if row_text.starts_with(&input) && !row_text.eq(&input) {
                    true
                } else {
                    false
                }
            });
            let input_field_to_for_scope2 = erw_for_scope.clone();
            match signal_id.take() {
                Some(id) => com_list.disconnect(id),
                None => ()
            }
            signal_id.replace(Some(com_list.connect_row_activated(move |list, row| {
                input_field_to_for_scope2.set_text(&row.child().unwrap().downcast_ref::<Label>().unwrap().text());
                list.set_visible(false);
            })));
        }
    });
}
