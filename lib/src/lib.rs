#[derive(PartialEq, serde::Serialize)]
pub struct Passport {
    pub number: String,
    pub country: String,
}

/// Check if a passport is valid.
pub fn check_passport(passport: &Passport) -> bool {
    let passport_can = Passport {
        number: "F2014".to_string(),
        country: "Canada".to_string(),
    };

    let passport_us = Passport {
        number: "D9993".to_string(),
        country: "United States".to_string(),
    };

    let passports = vec![passport_can, passport_us];

    passports.contains(passport)
}
