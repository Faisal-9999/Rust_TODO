use chrono::{NaiveDate, Utc};

use crate::custom_err::CustomError;

pub struct Todo {
    pub text : String,
    pub selected_date : NaiveDate,
    pub selected_hour : u32,
    pub selected_minute : u32,
}

impl ToString for Todo {
    fn to_string(&self) -> String {
        format!("{},{},{},{}", &self.text.clone(), &self.selected_date.to_string(), &self.selected_hour.to_string(), &self.selected_minute.to_string())
    }
}

impl Clone for Todo {
    fn clone(&self) -> Self {
        Self {
            text : self.text.clone(),
            selected_date : self.selected_date,
            selected_hour : self.selected_hour,
            selected_minute : self.selected_minute,
        }
    }
}

impl Default for Todo {
    fn default() -> Self {
        Todo {
            text : String::new(),
            selected_date : Utc::now().date_naive(),
            selected_hour : 0,
            selected_minute : 0,
        }
    }
}

impl Todo {

    pub fn from_string(line : &String) -> Self {

        let mut counter: u32 = 1;

        let mut data: (String, String, String, String) = (String::new(), String::new(), String::new(), String::new());

        for character in line.chars() {

            if character == ',' {
                counter += 1;
                continue;
            }

            match counter {
                1 => data.0.push(character),
                2 => data.1.push(character),
                3 => data.2.push(character),
                4 => data.3.push(character),
                _ => (),
            }
        }

        Self {
            text : data.0,
            selected_date : NaiveDate::parse_from_str(&data.1, "%Y-%m-%d").expect(CustomError::StringToDateConversionError.to_string().as_str()),
            selected_hour : data.2.parse::<u32>().expect(CustomError::StringTou32ConversionError.to_string().as_str()),
            selected_minute : data.3.parse::<u32>().expect(CustomError::StringTou32ConversionError.to_string().as_str()),
        }
    }
}