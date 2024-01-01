use std::panic::Location;

#[derive(Debug, Clone, Copy)]
enum ProtectedLocation {
    All,
    Office,
    Warehouse,
}

impl ProtectedLocation {
    fn require_access_level(&self) -> u16 {
        match self {
            Self::All => 1000,
            Self::Office => 800,
            Self::Warehouse => 500,
        }
    }
}

#[derive(Debug)]
struct Database;

impl Database {
    fn connect() -> Result<Self, String> {
        Ok(Database)
    }

    fn find_employee(&self, name: &str) -> Result<Employee, String> {
        match name {
            "Anita" => Ok(Employee {
                name: "Anita".to_string(),
            }),
            "Brody" => Ok(Employee {
                name: "Brody".to_string(),
            }),
            "Catherine" => Ok(Employee {
                name: "Catherine".to_string(),
            }),
            _ => Err("employee not found".to_owned()),
        }
    }

    fn get_keycard(&self, employee: &Employee) -> Result<KeyCard, String>{
        match employee.name.as_str() {
            "Anita" => Ok(KeyCard {access_level: 1000}),
            "Brody" => Ok(KeyCard {access_level: 500}),
            other => Err(format!("{other} doesn't have a keycard")),
        }
    }
}

struct Employee {
    name: String,
}

struct KeyCard {
    access_level: u16,
}

enum AuthorizationStatus {
    Allow,
    Deny,
}

fn authorize(
    employee_name: &str,
    location: ProtectedLocation,
) -> Result<AuthorizationStatus, String> {
    let db = Database::connect()?;

    let employee = db.find_employee(employee_name)?;

    let keycard = db.get_keycard(&employee);
}

fn main() {
    let anita_authorized = authorize("Anita", ProtectedLocation::Warehouse)
}
