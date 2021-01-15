#![allow(non_snake_case)]
use chrono::{NaiveDateTime, NaiveDate, Timelike, Duration};
use std::cell::{RefCell, Cell};
use std::rc::Rc;
use phf::{ phf_set, Set };
use std::collections::HashMap;
use std::hash::Hash;
use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct Prozedur<'li> {
    pub code: String,
    pub kennz: &'li str,
    pub datum: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct ProzedurMitGueltigkeit<'li> {
    pub prozedur: Rc<Prozedur<'li>>,
    pub validity: Cell<i32>,
    pub validity_katalog: i32,
    pub validity_set: String,
    pub treatment_type: String,
    pub validity_group: String,
    pub prozedur_beendet: Cell<Option<NaiveDateTime>>,
    pub entlass_datum: Option<NaiveDateTime>,
}

pub struct Fall<'a> {
    pub ops: RefCell<Vec<Rc<Prozedur<'a>>>>,
    pub adt: Option<NaiveDateTime>,
    pub sdt: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct Table<'a> {
    pub TYPE: &'a str,
    pub CONTENT: Set<&'static str>,
}

pub struct GtTable {
    pub Validity: i32,
    pub ValiditySet: String,
    pub TreatmentType: String,
    pub ValidityGroup: String,
    pub CONTENT: Set<&'static str>,
}


/*
    Init
 */
fn init_gt_tables<'li>() -> Vec<GtTable> {
    let gt1 = GtTable {
        Validity: 7,
        ValiditySet: "TE_Codes".to_string(),
        TreatmentType: "Ohne_TE".to_string(),
        ValidityGroup: "A".to_string(),
        CONTENT: phf_set! {
            "3-334", // unsere (p2)
            "4-443",
            "4-444",
        },
    };

    let gt2 = GtTable {
        Validity: 7,
        ValiditySet: "TE_Codes".to_string(),
        TreatmentType: "Mit_TE".to_string(),
        ValidityGroup: "B".to_string(),
        CONTENT: phf_set! {
            "3-333"  // unsere (p1)
        },
    };

    let gt3 = GtTable {
        Validity: 7,
        ValiditySet: "Therapieart".to_string(),
        TreatmentType: "Blablub".to_string(),
        ValidityGroup: "C".to_string(),
        CONTENT: phf_set! {
            "3-345",
            "4-441",  // unsere (p3)
            "4-449",  // unsere (p4)
        },
    };

    let mut v = vec![gt1, gt2, gt3];

    v
}

fn init_tables<'li>() -> Vec<Table<'li>> {
    let t1 = Table {
        TYPE: "NOR",
        CONTENT: phf_set! {
            "3-333",
            "3-335",
            "3-338",
        },
    };

    let t2 = Table {
        TYPE: "NOR",
        CONTENT: phf_set! {
            "3-334"
        },
    };

    let t3 = Table {
        TYPE: "NOR",
        CONTENT: phf_set! {
            "4-441",
            "4-442",
            "4-443",
            "4-444",
            "4-449",
            "4-450",
        },
    };

    let mut v = Vec::new();

    v.push(t1);
    v.push(t2);
    v.push(t3);

    v
}

fn init_adt(fall: &mut Fall) {
    let x = NaiveDate::from_ymd(2020, 12, 24).and_hms(12, 0, 0);
    fall.adt = Option::from(x);
}

fn init_sdt(fall: &mut Fall) {
    let x = NaiveDate::from_ymd(2020, 12, 31).and_hms(12, 0, 0);
    fall.sdt = Option::from(x);
}

fn init_prozeduren<'a>() -> Vec<Rc<Prozedur<'a>>> {
    let tag1 = NaiveDate::from_ymd(2020, 12, 24).and_hms(12, 0,0);
    let tag2 = NaiveDate::from_ymd(2020, 12, 26).and_hms(12, 0,0);
    let tag3 = NaiveDate::from_ymd(2020, 12, 29).and_hms(12, 0,0);

    let p1 = Prozedur {
        code: "3-333".to_string(),
        kennz: "",
        datum: Option::from(tag1),
    };

    let p2 = Prozedur {
        code: "3-334".to_string(),
        kennz: "",
        datum: Option::from(tag2),
    };


    let p3 = Prozedur {
        code: "4-441".to_string(),
        kennz: "",
        datum: Option::from(tag2),
    };

    let p4 = Prozedur {
        code: "4-449".to_string(),
        kennz: "",
        datum: Option::from(tag3),
    };

    let mut res = Vec::new();
    res.push(Rc::new(p1));
    res.push(Rc::new(p2));
    res.push(Rc::new(p3));
    res.push(Rc::new(p4));

    res
}

fn main() {
    let v_prozeduren = init_prozeduren();

    let mut fall = Fall {
        ops: RefCell::new(v_prozeduren),
        adt: None,
        sdt: None,
    };

    // init
    init_adt(&mut fall);
    init_sdt(&mut fall);

    let tables = init_tables();

    let aufnahme = fall.adt;
    let entlass = fall.sdt;
    println!("Aufnahmedatum: {}", aufnahme.unwrap());
    println!("Entlassdatum: {}", entlass.unwrap());

    let mut data_tables = Vec::new();
    let mut data_values = Vec::new();
    let mut start_val: i64 = 10;
    for t in tables.iter() {
        data_tables.push(t);
        data_values.push(start_val);
        start_val += 10;
    }

    DAYS_DAYTABLESCORE_GREATER_EQUALS(fall, data_tables, data_values,50);

}

fn to_prozedur_with_validity<'a>(p: Rc<Prozedur<'a>>, f: &Fall) -> ProzedurMitGueltigkeit<'a> {
    let gt_tables = init_gt_tables();
    let mut v: i32 = 1;
    let mut vs: String = String::new();
    let mut tt: String = String::new();
    let mut vg: String = String::new();

    let mut not_contains: bool = true;
    for t in gt_tables.iter() {
        println!("In Table: {}", t.ValiditySet);
        if t.CONTENT.contains(p.code.as_str()) {
            v = t.Validity.clone();
            vs = t.ValiditySet.clone();
            tt = t.TreatmentType.clone();
            vg = t.ValidityGroup.clone();
            not_contains = false;
            println!("Prozedur {} in Gültigkeitsmenge: {}", p.code, vs);
            break;
        }
    }
    if not_contains {
        v = 1;
        vs = "".to_string();
        tt = "".to_string();
        vg = "".to_string();
    }

    let entlass = p.datum.unwrap() + Duration::days(v as i64);

    let res = ProzedurMitGueltigkeit {
        prozedur: p,
        validity: Cell::new(v),
        validity_katalog: v,
        validity_set: vs,
        treatment_type: tt,
        validity_group: vg,
        prozedur_beendet: Cell::new(Option::from(entlass.clone())),
        entlass_datum: f.sdt.clone()
    };

    res
}

fn new_prozedur_with_validity<'a> (prozedur_mit_gueltigkeit: &'a ProzedurMitGueltigkeit, validity: Cell<i32>) -> ProzedurMitGueltigkeit<'a> {
    // Lesen: Umgang mit immutability

    let mut entlass = prozedur_mit_gueltigkeit.prozedur.datum.unwrap() + Duration::days(validity.get() as i64);

    if entlass.signed_duration_since(prozedur_mit_gueltigkeit.entlass_datum.unwrap()).num_days() > 0 {
        entlass = prozedur_mit_gueltigkeit.entlass_datum.unwrap().clone();
    }

    prozedur_mit_gueltigkeit.validity.replace(validity.get());
    prozedur_mit_gueltigkeit.prozedur_beendet.replace(Option::from(entlass));

        ProzedurMitGueltigkeit {
            prozedur: prozedur_mit_gueltigkeit.prozedur.clone(),
            validity,
            validity_katalog: prozedur_mit_gueltigkeit.validity_katalog,
            validity_set: prozedur_mit_gueltigkeit.validity_set.clone(),
            treatment_type: prozedur_mit_gueltigkeit.treatment_type.clone(),
            validity_group: prozedur_mit_gueltigkeit.validity_group.clone(),
            prozedur_beendet: Cell::new(Option::from(entlass.clone())),
            entlass_datum: prozedur_mit_gueltigkeit.entlass_datum.clone(),
        }


}

fn calc_ueberschneidung(p1: &ProzedurMitGueltigkeit, p2: &ProzedurMitGueltigkeit) -> i64 {
    let end_p1 = p1.prozedur.datum.unwrap() + Duration::days(p1.validity.get() as i64);
    let start_p2 = p2.prozedur.datum.unwrap();
    let duration_between_dates = start_p2 - end_p1;
    if duration_between_dates.num_days() > 0 {
        println!("Duration in days: {}", duration_between_dates.num_days());
        return -1 as i64;
    } else {
        println!("Duration in days: {}", duration_between_dates.num_days());
        let end_p1_neu = p1.validity.get() as i64 + duration_between_dates.num_days();
        println!("Neue Validity = {} (wie p1 zuweisen?)", end_p1_neu);
        println!("end_p1_neu: {}", end_p1_neu);
        return end_p1_neu;
    }
}

fn gueltigkeit_anpassen<'a>(proc_with_validities: &'a mut Vec<&ProzedurMitGueltigkeit<'a>>) -> Vec<ProzedurMitGueltigkeit<'a>> {
    // Prozeduren aus einem ValiditySet
    // Hier paarweise durch den Vektor gehen und die Gültigkeitstage berechnen.
    // Eine Zahl müsste für Gültigkeit reichen von 26.12. - 28.12. => v = 2
    proc_with_validities.sort_by(|&a, &b| a.prozedur.datum.cmp(&b.prozedur.datum));

    let mut result: Vec<ProzedurMitGueltigkeit> = Vec::new();

    for i in 0..proc_with_validities.len() {
        if i < proc_with_validities.len() - 1 {
            let end_of_p1: i64 = calc_ueberschneidung(proc_with_validities.get(i).unwrap(), proc_with_validities.get(i + 1).unwrap());

            if end_of_p1 > -1 {
                println!("!!!! end_of_p1 {}", end_of_p1);
                result.push(new_prozedur_with_validity(proc_with_validities.get(i).unwrap(), Cell::new(end_of_p1 as i32)));
            } else {
                result.push(new_prozedur_with_validity(
                    proc_with_validities.get(i).unwrap(),
                    proc_with_validities.get(i).unwrap().validity.clone()));
            }

            println!("{} und {} überschneiden sich", proc_with_validities.get(i).unwrap().prozedur.code,
                     proc_with_validities.get(i + 1).unwrap().prozedur.code
            );
            println!("{} mit {}", proc_with_validities.get(i).unwrap().prozedur.code, proc_with_validities.get(i).unwrap().prozedur.datum.unwrap());
            println!("{} mit {}", proc_with_validities.get(i+1).unwrap().prozedur.code, proc_with_validities.get(i+1).unwrap().prozedur.datum.unwrap());
        } else {
            result.push(new_prozedur_with_validity(
                proc_with_validities.get(i).unwrap(),
                proc_with_validities.get(i).unwrap().validity.clone()));
        }
    }

    result

}

fn DAYS_DAYTABLESCORE_GREATER_EQUALS(fall: Fall, tables: Vec<&Table>, values: Vec<i64>, value: i64) -> i64 {
    let first_day = fall.adt.unwrap();
    let last_day = fall.sdt.unwrap();

    /* ####################
        Schritt 1: Generiere ProzedurMitGueltigkeit Vektor
       #################### */

    let mut with_validity = Vec::new();

    for prozedur in fall.ops.borrow().iter() {
        let proc_with_validity = to_prozedur_with_validity(prozedur.clone(), &fall);
        with_validity.push(proc_with_validity);
    }

    /* ####################
        Schritt 2: Erstelle HashMap
       #################### */

    let mut hm: HashMap<&str, Vec<&ProzedurMitGueltigkeit>> = HashMap::new();

    // Erstelle Gülitgkeits-Map
    for proc_with_val in with_validity.iter() {
        // if proc_with_val.validity_set == "" {
        //     // Normale Prozedur, ohne Sets und Groups
        //     continue;
        // }

        if !hm.contains_key(proc_with_val.validity_set.as_str()) {
            hm.insert(proc_with_val.validity_set.as_str(), vec![proc_with_val]);
        } else {
            let val = hm.get_mut(proc_with_val.validity_set.as_str()).unwrap();
            val.push(proc_with_val);
        }
    }

    /* ####################
        Schritt 3: Gültigkeiten bei Überschneidungen anpassen
       #################### */

    let mut korrigierte_proc_validity = Vec::new();

    for (validity_set, proc_with_validities) in hm.iter_mut() {
        println!("Gültigkeit der Menge: {}", validity_set);
        let mut res_vector = gueltigkeit_anpassen(proc_with_validities);
        korrigierte_proc_validity.append(&mut res_vector)
    }

    println!("************** Neues Objekt ****************");
    for p in korrigierte_proc_validity.iter() {
        println!("Neue Gültigkeit von {} ist {}", p.prozedur.code, p.validity.get());
        println!("Von: {}", p.prozedur.datum.unwrap());
        println!("Bis: {}", p.prozedur_beendet.get().unwrap());
    }

    println!();

    println!("************** Altes Objekt: mit Cells ****************");
    for p in with_validity.iter() {
        println!("Neue Gültigkeit von {} ist {}", p.prozedur.code, p.validity.get());
        println!("Von: {}", p.prozedur.datum.unwrap());
        println!("Bis: {}", p.prozedur_beendet.get().unwrap());
    }

    0
}