extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{stdin, stdout, Write};

fn main() {
   let stdin = stdin(); 
   let mut stdout = stdout().into_raw_mode().unwrap();
   
   write!(stdout,
      "{}{}To-Do List:\nPress 'q' to quit\nPress 'a' to start a new To-Do\n\n",
      termion::clear::All,
      termion::cursor::Goto(1,1)
      )
      .unwrap();
   stdout.flush().unwrap();

   let mut todos: Vec<String> = Vec::new();
   let mut input = String::new();
   let mut pos: usize = 0;

   for k in stdin.keys(){
      write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

      match k.unwrap(){
         Key::Char('q') => break,
         Key::Char('a') => {
            write!(stdout, "{}{}Add a new To-Do:\n\r", termion::clear::All,termion::cursor::Goto(1,1)).unwrap();
            stdout.flush().unwrap();
            
            drop(stdout);

            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            input.push_str(" []");
            todos.push(input.clone());
            stdout = std::io::stdout().into_raw_mode().unwrap();


         },
         Key::Up => {
            if pos <= 0{
               pos = todos.len() - 1;
            } else {
               pos -= 1;
            }

         },
         Key::Down => {
            if pos >= todos.len() - 1 {
               pos = 0;
            } else {
               pos += 1;
            }

         },
         Key::Char('d') => {
            if todos.len() > 0 {
               todos.remove(pos);
               if pos >= todos.len(){
                  pos = todos.len() - 1;
               }
            }
         },
         Key::Char('m') => {
            if todos.len() > 0{
               if todos[pos].ends_with(" []"){
                  todos[pos] = todos[pos].replace(" []", " [x]");
               } else {
                  todos[pos] = todos[pos].replace(" [x]", " []");
               }
            }
         },
         _ => {}
      }
      write!(stdout, "{}{}To-Do List: \n\ra = Add | 🡩/🡫 = Move | d = Delete | m = Complete | q = Quit \n\r",termion::clear::All, termion::cursor::Goto(1,1)).unwrap();
      if todos.len() == 0 {
         write!(stdout, "No To-Dos, good job!").unwrap();
      }else{
         for (i, todo) in todos.iter().enumerate() {
            if i == pos {
                write!(stdout, "> {}.{}\n\r", i+1,todo).unwrap(); // Mark current item with ">"
            } else {
                write!(stdout, "  {}.{}\n\r",i+1, todo).unwrap(); // Normal formatting for other items
            }
         }
      }
      stdout.flush().unwrap();
      
   }
}
