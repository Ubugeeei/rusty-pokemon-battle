use std::io::stdout;
use std::io::Write;
use std::{thread, time::Duration};

use crate::pokemon::Pokemon;

pub fn print_current_buttle_status(poke1: &Pokemon, poke2: &Pokemon) {
  println!("------------------");
  println!("{}", poke1.name);
  println!("HP: {}", poke1.status.h.value);
  println!("------------------");
  println!("{}", poke2.name);
  println!("HP: {}", poke2.status.h.value);
  println!("------------------");
}

pub fn print_slill_list(poke: &Pokemon) {
  for (i, skill) in poke.skills.iter().enumerate() {
      println!("{}.{}", i + 1, skill.name);
  }
}

pub fn print_letter_by_letter(text: &str) {
  for c in text.chars() {
      print!("{}", c);
      let _ = stdout().flush();
      thread::sleep(Duration::from_millis(40));
  }
  println!("");
  thread::sleep(Duration::from_millis(500));
}

pub fn clear_screen() {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn clear_and_print_current_buttle_status(poke1: &Pokemon, poke2: &Pokemon) {
  clear_screen();
  print_current_buttle_status(poke1, poke2);
}