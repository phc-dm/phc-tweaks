use gtk::{prelude::*, Box, Button, Label, Orientation, Stack, StackSidebar};
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {

    let stack = Stack::builder()
        .vexpand(true)
        .hexpand(true)
        .build();

    let box_var = &Box::new(Orientation::Vertical, 10);

        // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // add the button to the box
    box_var.append(&button);
    box_var.set_hexpand(true);
    box_var.set_vexpand(true);


    stack.add_titled(
        box_var,
        Some("Tweaks_name"),
        "Tweaksssssssssssss");

    stack.add_titled(
        &Label::new("ciao".into()),
        Some("Tweaks2_name"),
        "Tweaks2");

    let side_bar = StackSidebar::builder()
        .build();

    side_bar.set_stack(&stack);

    // box with sidebar and stack
    let box1 = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .build();

    box1.append(&side_bar);
    box1.append(&stack);

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("PHC Tweaks")
        .child(&box1)
        .width_request(800)
        .height_request(600)
        // .child(&side_bar)
        .build();

    // Present window
    window.present();
}
