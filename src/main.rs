use std::{thread, time::Duration};

mod pokemon;
mod print;

use pokemon::*;
use print::*;

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
        let skill_idx = match &*input {
            "1\n" => 0,
            "2\n" => 1,
            "3\n" => 2,
            "4\n" => 3,
            _ => {
                continue;
            }
        };

        // attack
        pika.attack(skill_idx, &mut poppo);
        print_current_buttle_status(&pika, &poppo);
        println!("");

        /*
         * enemy attack
         */
        if poppo.status.h.value > 0 {
            // TODO: randomize
            poppo.attack(0, &mut pika);
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
