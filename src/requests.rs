use reqwest;
use rand::{self, Rng};
use serde::{Serialize, Deserialize};

pub fn post_data(link: &str, password: &str, matches: bool, points: u8) -> Result<(), reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    if matches == true {
        let mut randomdata: TempData;
        let mut data: Data;
        for i in 1..points+1 {
            for _e in 0..6 {
                randomdata = random_data(i.into());
                data = Data{data: randomdata};
                let res = client.post(link).json(&data).send()?;
                println!("{}", res.status());
            }
        }
    }
    else {
        for _i in 1..points+1 {
            let data = random_data(random_number(1, 100));
            let ndata = Data{data: data};
            // let client = reqwest::blocking::Client::new();
            let res = client.post(link).json(&ndata).header("x-pass-key", password).send()?;
            println!("{}", &res.status());
            // println!("{}", &res.status());
        }
    }
    
    // println!("{}", res.as_ref().);
    // println!("{}", res.status());
    Ok(())

}

pub fn random_data(matchnum: i64) -> TempData {
    TempData {
        team: DataString {content: random_number(1, 2000).to_string(), category: "Category".into()},
        matchnum: DataString{content: matchnum.to_string(), category: "Category".into()},
        absent: DataBool{content: false, category: "Team".into()},
        name: DataString {content: "Test".to_string(), category: "ACategory".into()},
        location: DataString {content: "Red 1".to_string(), category: "Category".into()},
        teamleftcommu: DataBool{ content: random_bool(), category: "Category".into()},
        teamcollected: DataBool { content: random_bool(), category: "Category".into()},
        autochargesta: DataString { content: "yeah".into(), category: "Category".into()},
        topcubes: DataInt {content: random_number(0, 10), category: "Points".into()},
        middlecubes: DataInt {content: random_number(0, 10), category: "Points".into()},
        bottomcubes: DataInt {content: random_number(0, 10), category: "Points".into()},
        missedcubes: DataInt {content: random_number(0, 10), category: "Points".into()},
        topcones: DataInt {content: random_number(0, 10), category: "Points".into()},
        middlecones: DataInt {content: random_number(0, 10), category: "Points".into()},
        bottomcones: DataInt {content: random_number(0, 10), category: "Points".into()},
        missedcones: DataInt {content: random_number(0, 10), category: "Points".into()},

        topcube: DataInt {content: random_number(0, 10), category: "Points".into()},
        middlecube: DataInt {content: random_number(0, 10), category: "Points".into()},
        bottomcube: DataInt {content: random_number(0, 10), category: "Points".into()},
        missedcube: DataInt {content: random_number(0, 10), category: "Points".into()},
        topcone: DataInt {content: random_number(0, 10), category: "Points".into()},
        middlecone: DataInt {content: random_number(0, 10), category: "Points".into()},
        bottomcone: DataInt {content: random_number(0, 10), category: "Points".into()},
        missedcone: DataInt {content: random_number(0, 10), category: "Points".into()},
        defenseplayti: DataFloat { content: random_float(0.0, 20.0), category: "Category".into()},
        defensiverati: DataInt { content: random_number(0, 6), category: "Category".into()},
        teamattemptsc: DataBool { content: random_bool(), category: "Category".into()},
        chargestation: DataString { content: "Who knows".into(), category: "Category".into()},
        links: DataInt { content: random_number(0, 7), category: "Category".into()},
        anyrobotprobl: DataString { content: "Absolutely".into(), category: "Category".into()},
        fouls: DataString { content: "Not much".into(), category: "Category".into()},
        extranotes: DataString { content: "White Woman Jumpscare!".into(), category: "Category".into()},
        driveteamrati: DataString { content: "They sure drove".into(), category: "Category".into()},
        playstylesumm: DataString { content: "Something".into(), category: "Category".into()},
        // password: password.into(),
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
pub struct DataString {
    pub content: String,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataInt {
    pub content: i64,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataFloat {
    pub content: f64,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataBool {
    pub content: bool,
    pub category: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub data: TempData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TempData {
    pub team: DataString,
    pub matchnum: DataString,
    pub absent: DataBool,
    pub name: DataString,
    pub location: DataString,
    pub teamleftcommu: DataBool,
    pub teamcollected: DataBool,
    pub autochargesta: DataString,
    pub topcubes: DataInt,
    pub middlecubes: DataInt,
    pub bottomcubes: DataInt,
    pub missedcubes: DataInt,
    pub topcones: DataInt,
    pub middlecones: DataInt,
    pub bottomcones: DataInt,
    pub missedcones: DataInt,
    pub topcube: DataInt,
    pub middlecube: DataInt,
    pub bottomcube: DataInt,
    pub missedcube: DataInt,
    pub topcone: DataInt,
    pub middlecone: DataInt,
    pub bottomcone: DataInt,
    pub missedcone: DataInt,
    pub defenseplayti: DataFloat,
    pub defensiverati: DataInt,
    pub teamattemptsc: DataBool,
    pub chargestation: DataString,
    pub links: DataInt,
    pub anyrobotprobl: DataString,
    pub fouls: DataString,
    pub extranotes: DataString,
    pub driveteamrati: DataString,
    pub playstylesumm: DataString,
    // pub password: String,
}