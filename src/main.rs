#![allow(dead_code)]

pub mod pokemon_structs {
    pub struct Pokemon {
        pub name: String,
        pub level: u8,
        pub status: Status,
        pub skils: Vec<Skill>,
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
    #[derive(Clone, Copy)]
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
    #[derive(Clone, Copy)]
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
            target: Target::Ally,
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
        skils: vec![tailwind, thundershock, growl, quick_attack],
    };

    let poppo = Pokemon {
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
        skils: vec![tackle],
    };
}
