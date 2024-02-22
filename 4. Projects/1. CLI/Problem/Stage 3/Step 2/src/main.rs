use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;
use ui::*;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;



fn main() {
    let database = Rc::new(JiraDatabase::new("./data/db.json".to_string()));
    let mut navigator = Navigator::new(Rc::clone(&database));
    // TODO: create database and navigator


    loop {
        clearscreen::clear().unwrap();
        println!("Clearscreen");
        if let Some(page) = navigator.get_current_page() 
            {
                if let Err(error) = page.draw_page(){
                    println!("Error rendering page: {}\nPress any key to continue...", error);
                    wait_for_key_press();
                }

            let user_input = get_user_input();
            
            println!("{}", user_input);
            
            wait_for_key_press();
            let action = page.handle_input(user_input.trim());
            match action {
                Ok(Some(action)) => {
                    navigator.handle_action(action);
                },
                Ok(None) => {},
                Err(e) => {
                    println!("Error: {}", e);
                }
                }
            }else{
                break
            }
           

        
        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}