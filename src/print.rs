use std::io::stdout;
use std::io::Write;
use std::{thread, time::Duration};

use crate::pokemon::Pokemon;

#[rustfmt::skip]
pub fn print_current_battle_status(mypoke: &Pokemon, enepoke: &Pokemon) {
  println!("　{}:L{}", enepoke.name, enepoke.level);
  println!("|　HP: {}", get_hp_bar(enepoke.status.current_hp, enepoke.status.h.value, 15));
  println!("------------------------▶");

  println!("");
  println!("");
  
  println!("　　　　　　　　{}:L{}", mypoke.name, mypoke.level);
  println!("　　　　　　　　HP: {}", get_hp_bar(mypoke.status.current_hp, mypoke.status.h.value, 15));
  println!("　　　　　　　　　　　{}/ 　{}　　　　|", mypoke.status.current_hp, mypoke.status.h.value);
  println!("　　　　　　　◀------------------------");

  println!("");
  println!("=============================================");
}

fn get_hp_bar(hp: u8, max_hp: u8, max_bar_len: u8) -> String {
  let mut bar = String::new();
  let bar_len = (hp as f32 / max_hp as f32 * max_bar_len as f32) as u8;
  for _ in 0..bar_len {
    bar.push('=');
  }
  for _ in bar_len..max_bar_len {
    bar.push('-');
  }
  bar
}

pub fn print_slill_list(poke: &Pokemon) {
  let pd = "　　　　　　　　　　　　";
  for (i, skill) in poke.skills.iter().enumerate() {
    println!("{}||　{}.{}", pd, i + 1, skill.name);
  }
  println!("{}========================", pd)
}

pub fn print_letter_by_letter(text: &str) {
  for c in text.chars() {
    print!("{}", c);
    let _ = stdout().flush();
    thread::sleep(Duration::from_millis(40));
  }
  println!("");
}

pub fn clear_screen() {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn clear_and_print_current_battle_status(poke1: &Pokemon, poke2: &Pokemon) {
  clear_screen();
  print_current_battle_status(poke1, poke2);
}
