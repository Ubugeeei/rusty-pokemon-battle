#![allow(dead_code)]

pub mod pokemon_structs {
    pub struct Pokemon {
        pub name: String,
        pub level: u8,
        pub status: Status,
        pub skils: [Skill; 4],
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

    pub struct Skill {
        pub name: String,
        pub rbi: u8,
        pub accuracy: u8,
        pub efficacy: Option<StatusEffect>,
        pub type_: SkillType,
    }
    pub enum SkillType {
        PhysicalAttack,
        SpecialAttack,
        ChangeStatus,
    }

    pub struct StatusEffect {
        pub efficacy: Efficacy,
        pub target: Target,
    }
    pub enum Target {
        Self_,
        Ally,
        Enemy,
    }
    pub struct Efficacy {
        pub target: StatusEnum,
        pub effect_value: i8,
    }
    pub enum StatusEnum {
        H,
        A,
        B,
        C,
        D,
        S,
    }
}

fn main() {
    use pokemon_structs::*;

    let tailwind = Skill {
        name: "しっぽをふる".to_string(),
        rbi: 0,
        accuracy: 100,
        efficacy: Some(StatusEffect {
            efficacy: Efficacy {
                target: StatusEnum::B,
                effect_value: -1,
            },
            target: Target::Ally,
        }),
        type_: SkillType::ChangeStatus,
    };

    let thundershock = Skill {
        name: "でんきショック".to_string(),
        rbi: 40,
        accuracy: 100,
        efficacy: None,
        type_: SkillType::SpecialAttack,
    };

    let growl = Skill {
        name: "なきごえ".to_string(),
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
        name: "でんこうせっか".to_string(),
        rbi: 40,
        accuracy: 100,
        efficacy: None,
        type_: SkillType::PhysicalAttack,
    };

    let pika = Pokemon {
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
        skils: [tailwind, thundershock, growl, quick_attack],
    };
}
