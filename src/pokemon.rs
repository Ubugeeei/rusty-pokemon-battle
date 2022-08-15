use super::utils::random::*;
use crate::print::print_letter_by_letter;
use std::{thread, time::Duration};

pub struct Pokemon {
  pub name: String,
  pub level: u8,
  pub status: Status,
  pub skills: Vec<Skill>,
}

pub struct Status {
  pub h: StatusAtom,
  pub a: StatusAtom,
  pub b: StatusAtom,
  pub c: StatusAtom,
  pub d: StatusAtom,
  pub s: StatusAtom,
  pub current_hp: u8,
}

pub struct StatusAtom {
  pub value: u8,
  pub buf: i8,
}

#[derive(Clone, Copy)]
pub struct Skill {
  pub name: &'static str,
  pub rbi: u8,
  pub accuracy: u8,
  pub efficacy: Option<StatusEffect>,
  pub type_: SkillType,
}
#[derive(Clone, Copy, PartialEq)]
pub enum SkillType {
  PhysicalAttack,
  SpecialAttack,
  ChangeStatus,
}

#[derive(Clone, Copy)]
pub struct StatusEffect {
  pub efficacy: Efficacy,
  pub target: Target,
}
#[derive(Clone, Copy)]
pub enum Target {
  Self_,
  Ally,
  Enemy,
}
#[derive(Clone, Copy)]
pub struct Efficacy {
  pub target: StatusEnum,
  pub effect_value: i8,
}
#[derive(Clone, Copy, PartialEq)]
pub enum StatusEnum {
  H,
  A,
  B,
  C,
  D,
  S,
}

impl Pokemon {
  pub fn attack(&mut self, skill_index: usize, target: &mut Pokemon) {
    let skill = *self.skills.get(skill_index).unwrap();
    print_letter_by_letter(&format!("{}の{}！", self.name, skill.name));
    thread::sleep(Duration::from_millis(1000));
    match skill.type_ {
      SkillType::PhysicalAttack => {
        let damage = self.compute_physical_damage(&skill, self, target);
        target.status.current_hp = target.status.current_hp.saturating_sub(damage);
      }
      SkillType::SpecialAttack => {
        let damage = self.compute_special_damage(&skill, self, target);
        target.status.current_hp = target.status.current_hp.saturating_sub(damage);
      }
      SkillType::ChangeStatus => {
        let efficacy = skill.efficacy.unwrap();
        match efficacy.target {
          Target::Self_ => {
            let m = if efficacy.efficacy.effect_value > 0 {
              "あがった"
            } else {
              "さがった"
            };
            let m2 = if efficacy.efficacy.effect_value.abs() == 2 {
              "ぐーんと"
            } else {
              ""
            };
            // TODO: function
            match efficacy.efficacy.target {
              StatusEnum::H => {
                self.status.h.buf += efficacy.efficacy.effect_value;
              }
              StatusEnum::A => {
                self.status.a.buf += efficacy.efficacy.effect_value;
                print_letter_by_letter(&format!("{}のこうげきが{}{}", self.name, m2, m));
              }
              StatusEnum::B => {
                self.status.b.buf += efficacy.efficacy.effect_value;
                print_letter_by_letter(&format!("{}のぼうぎょが{}{}", self.name, m2, m));
              }
              StatusEnum::C => {
                self.status.c.buf += efficacy.efficacy.effect_value;
                print_letter_by_letter(&format!("{}のとくこうが{}{}", self.name, m2, m));
              }
              StatusEnum::D => {
                self.status.d.buf += efficacy.efficacy.effect_value;
                print_letter_by_letter(&format!("{}のとくぼうが{}{}", self.name, m2, m));
              }
              StatusEnum::S => {
                self.status.s.buf += efficacy.efficacy.effect_value;
                print_letter_by_letter(&format!("{}のすばやさが{}{}", self.name, m2, m));
              }
            }
          }
          Target::Enemy => {
            let m = if efficacy.efficacy.effect_value > 0 {
              "あがった"
            } else {
              "さがった"
            };
            let m2 = if efficacy.efficacy.effect_value.abs() == 2 {
              "ぐーんと"
            } else {
              ""
            };
            match efficacy.efficacy.target {
              StatusEnum::H => {
                target.status.h.buf += efficacy.efficacy.effect_value;
              }
              StatusEnum::A => {
                target.status.a.buf += efficacy.efficacy.effect_value;
                print_letter_by_letter(&format!("{}のこうげきが{}{}", target.name, m2, m));
              }
              StatusEnum::B => {
                target.status.b.buf += efficacy.efficacy.effect_value;
                print_letter_by_letter(&format!("{}のぼうぎょが{}{}", target.name, m2, m));
              }
              StatusEnum::C => {
                target.status.c.buf += efficacy.efficacy.effect_value;
                print_letter_by_letter(&format!("{}のとくこうが{}{}", target.name, m2, m));
              }
              StatusEnum::D => {
                target.status.d.buf += efficacy.efficacy.effect_value;
                print_letter_by_letter(&format!("{}のとくぼうが{}{}", target.name, m2, m));
              }
              StatusEnum::S => {
                target.status.s.buf += efficacy.efficacy.effect_value;
                print_letter_by_letter(&format!("{}のすばやさが{}{}", target.name, m2, m));
              }
            }
          }
          Target::Ally => {
            // pika.status.h.value += efficacy.effect_value;
            // println!("{}のHPが{}増加した", pika.name, efficacy.effect_value);
          }
        }
      }
    }
    thread::sleep(Duration::from_millis(1000));
  }

  fn compute_physical_damage(&self, skill: &Skill, attacker: &Pokemon, target: &Pokemon) -> u8 {
    let mut damage = attacker.level as f32 * 2.0 / 5.0 + 2.0;
    damage = damage.floor();

    damage =
      damage * skill.rbi as f32 * attacker.status.a.value as f32 / target.status.b.value as f32;
    damage = damage.floor();
    
    damage = damage / 50.0 + 2.0;
    damage = damage.floor();

    // let rand = (1 / xor_shift_rand(42)) as f32;

    damage as u8
  }

  fn compute_special_damage(&self, skill: &Skill, attacker: &Pokemon, target: &Pokemon) -> u8 {
    let mut damage = attacker.level as f32 * 2.0 / 5.0 + 2.0;
    damage = damage.floor();

    damage =
      damage * skill.rbi as f32 * attacker.status.c.value as f32 / target.status.d.value as f32;
    damage = damage.floor();
    
    damage = damage / 50.0 + 2.0;
    damage = damage.floor();

    // let rand = (1 / xor_shift_rand(42)) as f32;

    damage as u8
  }
}
