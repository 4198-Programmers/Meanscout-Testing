use reqwest;
use rand::{self, Rng};
use serde::{Serialize, Deserialize};

pub fn post_data(link: &str, password: &str, matches: bool, points: u8) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    if matches == true {
        let mut randomdata: TempData;
        for i in 1..points+1 {
            for _e in 0..6 {
                randomdata = random_data(password, i.into());
                let res = client.post(link).json(&randomdata).send()?;
                println!("{}", res.status());
            }
        }
    }
    else {
        for i in 1..points {
            let data = random_data(password, random_number(1, 100));

            // let client = reqwest::blocking::Client::new();
            let _res = client.post(link).json(&data).send()?;
        }
    }
    
    // println!("{}", res.as_ref().);
    // println!("{}", res.status());
    Ok(())

}

pub fn random_data(password: &str, matchnum: i64) -> TempData {
    TempData {
        team: random_number(1, 20).to_string(),
        matchnum: matchnum.to_string(),
        absent: false,
        name: "Test".to_string(),
        location: "Red 1".to_string(),
        teamleftcommu: random_bool(),
        teamcollected: random_bool(),
        autochargesta: random_bool(),
        topcubes: random_number(0, 10),
        middlecubes: random_number(0, 10),
        bottomcubes: random_number(0, 10),
        missedcubes: random_number(0, 10),
        topcones: random_number(0, 10),
        middlecones: random_number(0, 10),
        bottomcones: random_number(0, 10),
        missedcones: random_number(0, 10),

        topcube: random_number(0, 10),
        middlecube: random_number(0, 10),
        bottomcube: random_number(0, 10),
        missedcube: random_number(0, 10),
        topcone: random_number(0, 10),
        middlecone: random_number(0, 10),
        bottomcone: random_number(0, 10),
        missedcone: random_number(0, 10),
        defenceplayti: random_float(0.0, 20.0),
        defensiverati: random_number(0, 6),
        teamattemptsc: random_bool(),
        chargestation: "Who knows".into(),
        links: random_number(0, 7),
        anyrobotprobl: "Absolutely".into(),
        fouls: "Not much".into(),
        extranotes: "White Woman Jumpscare!".into(),
        driveteamrati: "They sure drove".into(),
        playstylesumm: "Something".into(),
        password: password.into(),
    }

}

pub fn random_number(lower: i64, upper: i64) -> i64 {
    let num = rand::thread_rng().gen_range(lower..upper);
    return num
}

pub fn random_float(lower: f64, upper: f64) -> f64 {
    let num = rand::thread_rng().gen_range(lower..upper);
    return num
}
pub fn random_bool() -> bool {
    let thing = rand::thread_rng().gen_range(0..100);
    if 0 == thing%2 {
        return true
    }
    else {
        return false
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TempData {
    pub team: String,
    pub matchnum: String,
    pub absent: bool,
    pub name: String,
    pub location: String,
    pub teamleftcommu: bool,
    pub teamcollected: bool,
    pub autochargesta: bool,
    pub topcubes: i64,
    pub middlecubes: i64,
    pub bottomcubes: i64,
    pub missedcubes: i64,
    pub topcones: i64,
    pub middlecones: i64,
    pub bottomcones: i64,
    pub missedcones: i64,
    pub topcube: i64,
    pub middlecube: i64,
    pub bottomcube: i64,
    pub missedcube: i64,
    pub topcone: i64,
    pub middlecone: i64,
    pub bottomcone: i64,
    pub missedcone: i64,
    pub defenceplayti: f64,
    pub defensiverati: i64,
    pub teamattemptsc: bool,
    pub chargestation: String,
    pub links: i64,
    pub anyrobotprobl: String,
    pub fouls: String,
    pub extranotes: String,
    pub driveteamrati: String,
    pub playstylesumm: String,
    pub password: String,
}