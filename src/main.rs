use std::iter::{Iterator, zip};
use std::rc::Rc;
use emojis;
use emojis::Emoji;
use gtk::{Align, Application, ApplicationWindow, Clipboard, EventBox, FlowBox, Label, Orientation, Paned, ScrolledWindow, SearchEntry, SelectionMode};
use gtk::gdk;
use gtk::prelude::*;

#[derive(Debug)]
struct EmojiDescriptor {
    pub emoji: &'static Emoji,
    pub keywords: String,
}

impl EmojiDescriptor {
    fn new(emoji: &'static Emoji, keywords: String) -> EmojiDescriptor {
        EmojiDescriptor {
            emoji,
            keywords,
        }
    }
}

fn build_descriptor() -> Vec<EmojiDescriptor> {
    let mut result = vec![];
    for emoji in emojis::iter() {
        let emoji_name = emoji.name();
        let aliases = emoji.shortcodes().collect::<Vec<_>>().join(" ");
        let keywords = format!("{emoji_name} {aliases}").to_lowercase();
        result.push(EmojiDescriptor::new(emoji, keywords));
    }
    result
}


fn main() {
    let app = Application::new(
        Some("com.avt.app.emoji_picker"),
        gtk::gio::ApplicationFlags::empty(),
    );

    let emojis_descriptor_rc = Rc::new(build_descriptor());

    app.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("🚀 Emoji Picker");
        window.set_default_size(640, 480);

        let search_entry = SearchEntry::new();
        let feedback_label_rc = Rc::new(Label::new(None));

        let upper_content_paned = Paned::new(Orientation::Horizontal);
        upper_content_paned.pack1(&search_entry, true, false);
        upper_content_paned.pack2(feedback_label_rc.as_ref(), true, false);

        let main_horizontal_paned = Paned::new(Orientation::Horizontal);

        let left_content = Label::new(None);

        let right_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        let flow_box_rc = Rc::new(FlowBox::new());
        let flow_box = flow_box_rc.as_ref();
        flow_box.set_max_children_per_line(1000);
        flow_box.set_valign(Align::Start);
        flow_box.set_selection_mode(SelectionMode::None);
        right_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);

        for ed in emojis_descriptor_rc.as_ref() {
            let emoji = ed.emoji;
            let emoji_str = emoji.as_str();
            let emoji_label = Label::new(None);
            emoji_label.set_selectable(false);
            emoji_label.set_can_focus(false);
            emoji_label.set_markup(&format!("<span font=\"emoji 24\">{emoji_str}</span>"));
            let event_box = EventBox::new();
            let feedback_label_context = feedback_label_rc.clone();
            event_box.add(&emoji_label);
            event_box.connect_button_press_event(move |_, _| {
                Clipboard::get(&gdk::SELECTION_CLIPBOARD).set_text(emoji_str);
                Clipboard::get(&gdk::SELECTION_PRIMARY).set_text(emoji_str);
                feedback_label_context.set_text(&format!("{emoji_str} copied to clipboard !"));
                Inhibit(true)
            });
            event_box.set_can_focus(false);
            flow_box.add(&event_box);
        }
        right_window.add(flow_box);

        let flow_box_context = flow_box_rc.clone();
        let emojis_descriptor_context = emojis_descriptor_rc.clone();
        search_entry.connect_text_notify(move |f| {
            let flow_box_ref = flow_box_context.as_ref();
            let search_text = f.text().to_lowercase();
            if search_text.is_empty() {
                for child in &flow_box_ref.children() { child.show(); }
                return;
            }
            for (ed, child) in zip(emojis_descriptor_context.as_ref(), &flow_box_ref.children()) {
                if ed.keywords.contains(&search_text) { child.show(); } else { child.hide(); }
            }
        });

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
