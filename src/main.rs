use gtk4::prelude::*;
use gtk4::*;
use gdk::Display;

const APP_ID: &str = "com.gtkproject.HelloWorldTest";
fn main() {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_startup(|_| css_loader());
    app.connect_activate(program_main);
    app.run();
}

fn program_main(app: &Application){
    let logo_img = Image::builder()
        .name("logo_app")
        .file("assets/logo-modified.png")
        .pixel_size(150)
        .margin_top(115)
        .build();

    let input_username = Entry::builder().name("user")
        .visible(true)
        .placeholder_text("Username")
        .width_request(19)
        .height_request(10)
        .margin_start(350)
        .margin_end(350)
        .margin_top(45)
        .input_purpose(InputPurpose::Name)
        .build();
    
    let input_passwd = Entry::builder().name("passwd")
        .visible(true)
        .placeholder_text("Password")
        .width_request(19)
        .height_request(10)
        .margin_start(350)
        .margin_end(350)
        .margin_top(9)
        .visibility(false)
        .invisible_char('*' as u32)
        .input_purpose(InputPurpose::Password)
        .build();
    
    let button_login = Button::builder().name("data_receiver")
        .visible(true)
        .margin_top(10)
        .margin_start(400)
        .margin_end(400)
        .label("Login")
        .build();
    
    let verlab = Label::builder()
        .label("SNAPSHOT-1.2")
        .width_chars(1)
        .margin_top(185)
        .margin_start(900)
        .build();

    let fb_lab = Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    fb_lab.append(&logo_img);
    fb_lab.append(&input_username);
    fb_lab.append(&input_passwd);
    fb_lab.append(&button_login);
    fb_lab.append(&verlab);

    button_login.connect_clicked(move |_x| {
        let usr = input_username.text().as_str().to_owned();
        let pwd = input_passwd.buffer().text().as_str().to_owned();
        if usr.eq("root") && pwd.ne("220514"){
            MessageDialog::builder()
                .message_type(MessageType::Info)
                .buttons(ButtonsType::Ok)
                .text("Invalid")
                .secondary_text("Invalid input for user \"root\"")
                .build()
                .run_async(|obj, _| {
                    obj.hide();
            });
        }
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("RTOR - Resources Navigator")
        .child(&fb_lab)
        .default_width(1024)
        .default_height(600)
        .resizable(false)
        .build();
    window.present();
    
}

fn css_loader(){
    let prov = CssProvider::new();
    prov.load_from_data(include_bytes!("loader.css"));
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not load provider"),
        &prov,
        STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}