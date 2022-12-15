use gtk4::prelude::*;
use gtk4::*;
use gdk::Display;
mod functs;
use functs::FileDatabase;
use std::format;

const APP_ID: &str = "com.gtkproject.RTOR";
fn main() {
    gtk4::init().expect("Failed to initialize GTK");
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    app.connect_startup(|_| css_loader());
    app.connect_activate(program_main);
    app.run();
}

fn program_main(app: &Application){
    //
    // WIDGET AREA
    //
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
    let button_signup = Button::builder().name("signup_open")
        .visible(true)
        .margin_top(10)
        .margin_start(400)
        .margin_end(400)
        .label("Sign Up")
        .build();
    
    let verlab = Label::builder()
        .label("SNAPSHOT-1.2")
        .width_chars(1)
        .margin_top(175)
        .margin_start(900)
        .build();

    let fb_lab = Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    fb_lab.append(&logo_img);
    fb_lab.append(&input_username);
    fb_lab.append(&input_passwd);
    fb_lab.append(&button_login);
    fb_lab.append(&button_signup);
    fb_lab.append(&verlab);

    let sig_box = Box::builder()
        .name("signup_box")
        .visible(false)
        .orientation(Orientation::Vertical)
        .build();
    let btn_sig = Button::builder()
        .visible(true)
        .margin_top(10)
        .margin_start(400)
        .margin_end(400)
        .label("Create Account")
        .build();
    let btn_back = Button::builder()
        .name("back_cancel")
        .visible(true)
        .label("Back to Login")
        .margin_start(425)
        .margin_top(10)
        .margin_end(425)
        .build();
    let sig_passwd = Entry::builder()
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
    let sig_user = Entry::builder()
        .visible(true)
        .placeholder_text("Username")
        .width_request(19)
        .height_request(10)
        .margin_start(350)
        .margin_end(350)
        .margin_top(215)
        .input_purpose(InputPurpose::Name)
        .build();
        
    sig_box.append(&sig_user);
    sig_box.append(&sig_passwd);
    sig_box.append(&btn_sig);
    sig_box.append(&btn_back);

    //
    //  MAIN THINGS AND CLICK-EVENT
    //
    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    main_box.append(&fb_lab);
    main_box.append(&sig_box);

    button_login.connect_clicked(move |_x| {
        let usr = input_username.text().as_str().to_owned();
        let pwd = input_passwd.buffer().text().as_str().to_owned();
        let db = FileDatabase::new("database.dat");
        if !db.data_exist(usr.as_str()){
            alert_warn("Invalid User", format!("Invalid user, not found: {usr}").as_str());
        }else{
            let mut realpwd = String::new();
            db.getpasswd(usr.as_str(), &mut realpwd).unwrap();
            if pwd.ne(&realpwd){
                alert_info("Wrong", format!("Wrong password for user {usr}").as_str());
            }else{
                alert_info("Success", "Login completed.");
            }
        }
    });

    let bor1 = fb_lab.clone();
    let bor11 = sig_box.clone();
    button_signup.connect_clicked(move |_|{
        sig_men(&bor1, &bor11);
    });

    let bor2 = fb_lab.clone();
    let bor22 = sig_box.clone();
    btn_back.connect_clicked(move |_|{
        sig_log(&bor2, &bor22);
    });

    btn_sig.connect_clicked(move |_|{
        let usrsig = sig_user.text().as_str().to_owned();
        let passwdsig = sig_passwd.buffer().text().as_str().to_owned();
        let db = FileDatabase::new("database.dat");
        let fmts = format!("{usrsig}:{passwdsig}\n");
        if db.data_exist(usrsig.as_str()){
            alert_warn("Found", format!("User {usrsig} already existed.").as_str());
        }else{
            db.write(fmts.as_str(), true);
            alert_info("Success", "User registered successfully, you may return to login page.");
        }
    });
    let window = ApplicationWindow::builder()
        .application(app)
        .title("RTOR - Resources Navigator")
        .child(&main_box)
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


fn sig_men<'a>(logb: &'a Box, sigb: &'a Box){
    logb.hide();
    sigb.show();
}

fn sig_log<'a>(logb: &'a Box, sigb: &'a Box){
    logb.show();
    sigb.hide();
}

fn alert_info(head: &str, body: &str){
    MessageDialog::builder()
        .message_type(MessageType::Info)
        .buttons(ButtonsType::Ok)
        .text(head)
        .secondary_text(body)
        .build()
        .run_async(|obj, _| {
            obj.hide();
        })
}

fn alert_warn(head: &str, body: &str){
    MessageDialog::builder()
        .message_type(MessageType::Warning)
        .buttons(ButtonsType::Ok)
        .text(head)
        .secondary_text(body)
        .build()
        .run_async(|obj, _| {
            obj.hide();
        })
}