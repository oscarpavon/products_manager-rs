use gtk4::prelude::*;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Button;
use gtk4::Box;
use gtk4::Orientation;
use gtk4::Entry;
use gtk4::Label;
use gtk4::Window;
use gtk4::ComboBoxText;

use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;


use gtk4::DropDown;

use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct DataBase{
    data: Vec<Product>
}
#[derive(Serialize, Deserialize, Debug)]
struct Product{
    name: String,
    price: u32,
}

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();
    

    app.connect_activate(build_ui);
    // Run the application
    app.run();
}

fn build_ui(app: &Application){
     
    let product_add_window = ApplicationWindow::builder()
        .application(app)
        .title("Agregar Producto")
        .build();

    let button_open_add_window = Button::builder()
        .label("Agregar Producto")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();


    button_open_add_window.connect_clicked(move |button| {
        
        product_add_window.present();        
    });

    let button_add = Button::builder()
        .label("Agregar")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();



    let mybox2 = Box::new(Orientation::Horizontal,2);
    let mybox3 = Box::new(Orientation::Horizontal,2);

    let mybox = Box::new(Orientation::Vertical,14);

    let product_price_entry = Entry::builder()
        .text("")
        .build();
        
    let product_name_entry = Entry::builder()
        .text("")
        .build();
    
    let client_total = Label::new(Some("Total: 0"));


    let product_price_label = Label::new(Some("Precio"));
    let product_name_label = Label::new(Some("Nombre"));
    
    let combo = ComboBoxText::with_entry();
    combo.append_text("lala");
    combo.append_text("lala2");
    combo.append_text("lala4");
    combo.append_text("lala5");
    combo.append_text("lala6");
    combo.append_text("lala6");
    combo.append_text("lala7");
   


  //  let string_json = std::fs::read_to_string("/home/pavon/ice-manager/src/data.txt").unwrap();

    //println!("data is: {:?}",string_json);
    //let products: Vec<Product> = serde_json::from_str(&string_json)
  //      .expect("asdf");
    //println!("data is: {:?}",products[0]);

    let names = ["peroo","gatos","loboos","gallinas","melones"];
    let drop = DropDown::from_strings(&names);
    drop.set_enable_search(true);


    mybox.append(&client_total) ;
    

    mybox2.append(&product_price_label);
    mybox2.append(&product_price_entry);
    
    mybox3.append(&product_name_label);
    mybox3.append(&product_name_entry);

    mybox.append(&mybox2);
    mybox.append(&mybox3);
    

    mybox.append(&button_add);


    mybox.append(&button_open_add_window);

    mybox.append(&combo);
    mybox.append(&drop);

    button_add.connect_clicked(move |button| {
        
    
    let string_json = std::fs::read_to_string("/home/pavon/ice-manager/src/data.txt").unwrap();

    let mut database : DataBase = serde_json::from_str(&string_json)
        .expect("asdf");
       

        let te = product_name_entry.text();
        
            let a = product_price_entry.text();
        let _price = a.parse::<u32>().unwrap();

       

        let new_product = Product {
           name: te.to_string(),
           price: _price 
       };
    
        
        database.data.push(new_product);

        let json = serde_json::to_string(&database).unwrap();


        std::fs::write("/home/pavon/ice-manager/src/data.txt",json);

        button.set_label("Agregado");

    });



   // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Sistema de gestion de productos")
        .build();

    
    window.set_child(Some(&mybox));
    
    // Present window
    window.present();
}
