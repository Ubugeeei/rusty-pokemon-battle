#![allow(dead_code)]

pub mod pokemon_structs {
    pub struct Pokemon {
        name: String,
        level: u8,
        status: Status,
        skils: [Skill; 4]
    }
    pub struct Status {
        h: StatusAtom,
        a: StatusAtom,
        b: StatusAtom,
        c: StatusAtom,
        d: StatusAtom,
        s: StatusAtom,
    }
    pub struct StatusAtom {
        value: u8,
        buf: i8,
    }

    pub struct Skill {
        name: String,
        rbi: u8,
        accuracy: u8,
        type_: SkillType,
    }

    enum SkillType {
        Physical,
        Special,
    }
}

fn main() {
    println!("Hello, world!");
}
