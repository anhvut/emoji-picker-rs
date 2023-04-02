use std::rc::Rc;
use emojis;
use gtk::{Application, ApplicationWindow, Clipboard, EventBox, FlowBox, Label, Orientation, Paned, ScrolledWindow, SearchEntry};
use gtk::gdk;
use gtk::prelude::*;

fn main() {
    let app = Application::new(
        Some("com.avt.app.emoji_picker"),
        gtk::gio::ApplicationFlags::empty(),
    );

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("🚀 Emoji Picker");
        window.set_default_size(640, 480);

        let search_entry = SearchEntry::new();
        let feedback_label = Rc::new(Label::new(None));

        let upper_content_paned = Paned::new(Orientation::Horizontal);
        upper_content_paned.pack1(&search_entry, true, false);
        upper_content_paned.pack2(feedback_label.as_ref(), true, false);

        let main_horizontal_paned = Paned::new(Orientation::Horizontal);

        let left_content = Label::new(None);

        let right_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        let flow_box = FlowBox::new();
        right_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);

        for emoji in emojis::iter() {
            let emoji_str = emoji.as_str();
            let emoji_label = Label::new(None);
            emoji_label.set_markup(&format!("<span font=\"emoji 24\">{emoji_str}</span>"));
            let event_box = EventBox::new();
            let feedback_label_context = feedback_label.clone();
            event_box.add(&emoji_label);
            event_box.connect_button_press_event(move |_, _| {
                Clipboard::get(&gdk::SELECTION_CLIPBOARD).set_text(emoji_str);
                Clipboard::get(&gdk::SELECTION_PRIMARY).set_text(emoji_str);
                feedback_label_context.set_text(&format!("{emoji_str} copied to clipboard !"));
                Inhibit(true)
            });
            flow_box.add(&event_box);
        }
        right_window.add(&flow_box);

        main_horizontal_paned.add(&left_content);
        main_horizontal_paned.add(&right_window);

        let main_vertical_paned = Paned::new(Orientation::Vertical);
        main_vertical_paned.pack1(&upper_content_paned, false, false);
        main_vertical_paned.add(&main_horizontal_paned);

        window.add(&main_vertical_paned);
        window.show_all();
    });

    app.run();
}
