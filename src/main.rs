use gtk::{prelude::*, Align, Box, Button, FileChooserAction, FileChooserDialog, Label, Orientation, ResponseType, Stack, StackSidebar};
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


fn print_utils(window: &ApplicationWindow) -> Box {
    let box_var = Box::new(Orientation::Vertical, 10);

    let title = Label::new(None);
    title.set_markup("<span font='17' weight='bold'>\nUtilità per la stampa</span>");

    box_var.append(&title);

    // Create a button with label and margins
    let button = Button::builder()
        .label("Seleziona il file")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let window_clone = window.clone();

    // non so veramente cosa ho fatto qui, quel @weak button => è un po' magia per me. Scusa Antonio
    button.connect_clicked(glib::clone!(@weak button => move |_| {

        let file_chooser = FileChooserDialog::new( // forse non è filechooserdialog, ma filechooser, ma non funziona quello
            Some("Import File"),
            Some(&window_clone), // manco questo so bene che cazzo è
            FileChooserAction::Open,
            &[("Open", ResponseType::Ok), ("Cancel", ResponseType::Cancel)]
            // Il problema qui è che non so come fare per fargli fare qualcosa. Quando premo Open non succede nulla, vorrei che salvasse in una variabile il path del file selezionato
            // Quando premo Cancel vorrei che chiudesse la finestra, ma non la chiude. Ho provato pure altre opzioni come ResponseType::Close, ma non funziona
        );

        // questa cosa qui sotto non funziona, ma il concetto è questo. Se clicca Ok deve prendere il file path

        // if file_chooser.response(ResponseType::Ok) == ResponseType::Ok {
        //     let filename = file_chooser.filename(); }

        file_chooser.show();

    }));


    box_var.append(&Label::new(Some("Seleziona il file da stampare cliccando il pulsante qui sotto")));

    // add the button to the box
    box_var.append(&button);

    box_var.append(&Label::new(Some("Hai selezionato il file: {file_path}"))); // vorrei che questa stringa spuntasse solo dopo aver selezionato il file

    let box_aux = Box::new(Orientation::Horizontal, 10);
    let subtitle = Label::new(None);
    subtitle.set_markup("<span font='12' weight='bold'>\nProprietà di stampa</span>");
    box_aux.append(&subtitle);
    box_aux.set_hexpand(true);
    box_var.append(&box_aux);

    // Add a combo box
    let combo_box = gtk::ComboBoxText::new();
    // add 3 items to the combo box
    combo_box.append_text("Aula 3");
    combo_box.append_text("Aula 4");
    combo_box.append_text("Corridoio Piano Terra");

    let seleziona_stampante = Label::new(Some("Seleziona la stampante:"));
    let box2 = Box::new(Orientation::Horizontal, 10);

    // put the combo box to the right of the label
    box2.append(&seleziona_stampante);
    box2.append(&combo_box);

    let fronte_retro = gtk::Switch::new();
    let fronte_retro_label = Label::new(Some("\tFronte retro")); // dovrei mettere Aling::End, ma non funziona

    box2.append(&fronte_retro_label);
    box2.append(&fronte_retro);

    box_var.append(&box2);

    box_var
}

fn create_tweaks2() -> Label {
    Label::new("ciao".into())
}

fn build_ui(app: &Application) {
    let stack = Stack::builder()
        // .vexpand(true)
        // .hexpand(true)
        .build();

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("PHC Tweaks")
        .width_request(800)
        .height_request(600)
        .build();

    let print_sidebar = print_utils(&window);
    let tweaks2 = create_tweaks2();

    stack.add_titled(
        &print_sidebar,
        Some("Utility per la stampa"),
        "Stampa");

    stack.add_titled(
        &tweaks2,
        Some("Tweaks2_name"),
        "TO DO");

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

    window.set_child(Some(&box1));

    // Present window
    window.present();
}
