use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

pub fn create_gui() {
    let application = gtk::Application::new(Some("com.example.tarkov-cheat"), Default::default());
    application.connect_activate(|app| {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Tarkov Cheat");
        window.set_default_size(300, 200);

        let button_aimbot = Button::with_label("Toggle Aimbot");
        let button_esp = Button::with_label("Toggle ESP");
        let button_no_recoil = Button::with_label("Toggle No Recoil");

        button_aimbot.connect_clicked(|_| {
            // Toggle aimbot
        });

        button_esp.connect_clicked(|_| {
            // Toggle ESP
        });

        button_no_recoil.connect_clicked(|_| {
            // Toggle no recoil
        });

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        vbox.pack_start(&button_aimbot, true, true, 0);
        vbox.pack_start(&button_esp, true, true, 0);
        vbox.pack_start(&button_no_recoil, true, true, 0);

        window.add(&vbox);
        window.show_all();
    });

    application.run();
}