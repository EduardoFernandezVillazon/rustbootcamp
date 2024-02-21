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
    let mut navigator = Navigator::new(database);
    // TODO: create database and navigator
    
    loop {
        clearscreen::clear().unwrap();
        let page : Option<&Box<dyn Page>> = navigator.get_current_page();
        match page{
            Some(page) => {
                page.draw_page().unwrap();
                let input = get_user_input();
                println!("{}", input);
                let action = page.handle_input(&input);
                match action {
                    Ok(Some(action)) => {
                        navigator.handle_action(action).unwrap();
                    },
                    Ok(None) => {},
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            },
            None => break
        }
        
        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        // 2. render page
        // 3. get user input
        // 4. pass input to page's input handler
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}