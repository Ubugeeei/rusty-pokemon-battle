use std::io::stdout;
use std::io::Write;
use std::{thread, time::Duration};

pub mod pokemon_structs {
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
}

use pokemon_structs::*;

fn attack(
    attacker: &mut pokemon_structs::Pokemon,
    skill: &Skill,
    target: &mut pokemon_structs::Pokemon,
) {
    print_letter_by_letter(&format!("{}の{}！", attacker.name, skill.name));
    thread::sleep(Duration::from_millis(1000));
    match skill.type_ {
        SkillType::PhysicalAttack => {
            let damage = (skill.rbi as f32 * target.level as f32 / 50.0) as u8;
            target.status.h.value = target.status.h.value.saturating_sub(damage);
        }
        SkillType::SpecialAttack => {
            let damage = (skill.rbi as f32 * target.level as f32 / 50.0) as u8;
            target.status.h.value = target.status.h.value.saturating_sub(damage);
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
                            attacker.status.h.buf += efficacy.efficacy.effect_value;
                        }
                        StatusEnum::A => {
                            attacker.status.a.buf += efficacy.efficacy.effect_value;
                            print_letter_by_letter(&format!("{}のこうげきが{}{}", attacker.name, m2, m));
                        }
                        StatusEnum::B => {
                            attacker.status.b.buf += efficacy.efficacy.effect_value;
                            print_letter_by_letter(&format!("{}のぼうぎょが{}{}", attacker.name, m2, m));
                        }
                        StatusEnum::C => {
                            attacker.status.c.buf += efficacy.efficacy.effect_value;
                            print_letter_by_letter(&format!("{}のとくこうが{}{}", attacker.name, m2, m));
                        }
                        StatusEnum::D => {
                            attacker.status.d.buf += efficacy.efficacy.effect_value;
                            print_letter_by_letter(&format!("{}のとくぼうが{}{}", attacker.name, m2, m));
                        }
                        StatusEnum::S => {
                            attacker.status.s.buf += efficacy.efficacy.effect_value;
                            print_letter_by_letter(&format!("{}のすばやさが{}{}", attacker.name, m2, m));
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

/*
 * print
 */

fn print_current_buttle_status(poke1: &Pokemon, poke2: &Pokemon) {
    println!("------------------");
    println!("{}", poke1.name);
    println!("HP: {}", poke1.status.h.value);
    println!("------------------");
    println!("{}", poke2.name);
    println!("HP: {}", poke2.status.h.value);
    println!("------------------");
}

fn print_slill_list(poke: &Pokemon) {
    for (i, skill) in poke.skills.iter().enumerate() {
        println!("{}.{}", i + 1, skill.name);
    }
}

fn print_letter_by_letter(text: &str) {
    for c in text.chars() {
        print!("{}", c);
        let _ = stdout().flush();
        thread::sleep(Duration::from_millis(40));
    }
    println!("");
    thread::sleep(Duration::from_millis(500));
}

fn main() {
    /*
     * slols
     */

    let tailwind = Skill {
        name: "しっぽをふる",
        rbi: 0,
        accuracy: 100,
        efficacy: Some(StatusEffect {
            efficacy: Efficacy {
                target: StatusEnum::B,
                effect_value: -1,
            },
            target: Target::Enemy,
        }),
        type_: SkillType::ChangeStatus,
    };
    let thundershock = Skill {
        name: "でんきショック",
        rbi: 40,
        accuracy: 100,
        efficacy: None,
        type_: SkillType::SpecialAttack,
    };
    let growl = Skill {
        name: "なきごえ",
        rbi: 0,
        accuracy: 100,
        efficacy: Some(StatusEffect {
            efficacy: Efficacy {
                target: StatusEnum::A,
                effect_value: -1,
            },
            target: Target::Enemy,
        }),
        type_: SkillType::ChangeStatus,
    };
    let quick_attack = Skill {
        name: "でんこうせっか",
        rbi: 40,
        accuracy: 100,
        efficacy: None,
        type_: SkillType::PhysicalAttack,
    };
    let tackle = Skill {
        name: "たいあたり",
        rbi: 40,
        accuracy: 100,
        efficacy: None,
        type_: SkillType::PhysicalAttack,
    };

    /*
     * pokemons
     */

    let mut pika = Pokemon {
        name: "ピカチュウ".to_string(),
        level: 10,
        status: Status {
            h: StatusAtom { value: 30, buf: 0 },
            a: StatusAtom { value: 19, buf: 0 },
            b: StatusAtom { value: 16, buf: 0 },
            c: StatusAtom { value: 18, buf: 0 },
            d: StatusAtom { value: 18, buf: 0 },
            s: StatusAtom { value: 26, buf: 0 },
        },
        skills: vec![tailwind, thundershock, growl, quick_attack],
    };

    let mut poppo = Pokemon {
        name: "ポッポ".to_string(),
        level: 10,
        status: Status {
            h: StatusAtom { value: 30, buf: 0 },
            a: StatusAtom { value: 19, buf: 0 },
            b: StatusAtom { value: 16, buf: 0 },
            c: StatusAtom { value: 18, buf: 0 },
            d: StatusAtom { value: 18, buf: 0 },
            s: StatusAtom { value: 26, buf: 0 },
        },
        skills: vec![tackle],
    };

    /*
     * battle
     */
    print_letter_by_letter("やせいのポッポがあられた！");
    print_current_buttle_status(&pika, &poppo);
    while pika.status.h.value > 0 && poppo.status.h.value > 0 {
        thread::sleep(Duration::from_millis(1000));
        print_letter_by_letter("どうする？");
        print_slill_list(&pika);

        // select skill
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let skill = match &*input {
            "1\n" => *pika.skills.get(0).unwrap(),
            "2\n" => *pika.skills.get(1).unwrap(),
            "3\n" => *pika.skills.get(2).unwrap(),
            "4\n" => *pika.skills.get(3).unwrap(),
            _ => {
                unreachable!()
            }
        };

        // attack
        attack(&mut pika, &skill, &mut poppo);
        print_current_buttle_status(&pika, &poppo);
        println!("");

        /*
         * enemy attack
         */
        if poppo.status.h.value > 0 {
            // TODO: randomize
            let skill = *poppo.skills.get(0).unwrap();
            attack(&mut poppo, &skill, &mut pika);
            print_current_buttle_status(&pika, &poppo);
            println!("");
        }

        print!("\n\n\n\n");
    }

    if pika.status.h.value > 0 {
        print_letter_by_letter(&format!("{}とのしょうぶにかった !", poppo.name));
    } else {
        print_letter_by_letter(&format!("めのまえがまっくらになった"));
    }
}
