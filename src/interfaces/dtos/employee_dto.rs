use serde::{Deserialize, Serialize};
use time::Date;

time::serde::format_description!(date_format, Date, "[year]-[month]-[day]");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmployeeDTO {
    pub name: String,
    #[serde(with = "date_format")]
    pub birth_date: Date,
    pub gender: String,
    pub mother_name: String,
    pub father_name: String,
}

#[allow(unused)]
impl EmployeeDTO {
    pub fn new<N, G, M, F>(
        name: N,
        birth_date: Date,
        gender: G,
        mother_name: M,
        father_name: F,
    ) -> Self
    where
        N: Into<String>,
        G: Into<String>,
        M: Into<String>,
        F: Into<String>,
    {
        Self {
            name: name.into(),
            birth_date,
            gender: gender.into(),
            mother_name: mother_name.into(),
            father_name: father_name.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_employee_creation_success() {
        let employee_dto = EmployeeDTO::new(
            "Employee1",
            time::macros::date!(1990 - 01 - 01),
            "Male",
            "Mother Name",
            "Father Name",
        );

        assert_eq!(
            employee_dto,
            EmployeeDTO {
                name: "Employee1".to_string(),
                birth_date: time::macros::date!(1990 - 01 - 01),
                gender: "Male".into(),
                mother_name: "Mother Name".into(),
                father_name: "Father Name".into(),
            }
        );
    }
}
