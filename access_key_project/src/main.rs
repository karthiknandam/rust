#[derive(Debug, Clone, Copy)]
enum AccessUser {
    Allow,
    Denied,
}

fn access(name: &str, location: ProtocolLocations) -> Result<AccessUser, String> {
    let db = Db::connect("karthik")?;
    let user = Db::find_user(&db, name)?;
    let get_key = Db::get_key(&db, &user)?;
    if get_key.action >= location.get_value() {
        Ok(AccessUser::Allow)
    } else {
        Ok(AccessUser::Denied)
    }
}
struct Db;
impl Db {
    fn connect(key: &str) -> Result<Self, String> {
        if key == "karthik" {
            Ok(Db)
        } else {
            Err("Unauthorized connection".to_owned())
        }
    }
    fn find_user(&self, name: &str) -> Result<Employee, String> {
        match name {
            "karthik" => Ok(Employee {
                name: name.to_owned(),
            }),
            "nandam" => Ok(Employee {
                name: name.to_owned(),
            }),
            "luffy" => Ok(Employee {
                name: name.to_owned(),
            }),
            _ => Err("Cannot Find User".to_owned()),
        }
    }
    fn get_key(&self, _employee: &Employee) -> Result<KeyCard, String> {
        match _employee.name.as_str() {
            "karthik" => Ok(KeyCard { action: 1000 }),
            "nandam" => Ok(KeyCard { action: 500 }),
            // "luffy" => Ok(KeyCard { action: 500 }),
            _other => Err(format!("{} sorry you are not allowed here", _other)),
        }
    }
}
enum ProtocolLocations {
    All,
    WareHouse,
    Office,
}
impl ProtocolLocations {
    fn get_value(&self) -> u16 {
        match self {
            ProtocolLocations::All => 1000,
            ProtocolLocations::Office => 800,
            ProtocolLocations::WareHouse => 500,
        }
    }
}
struct Employee {
    name: String,
}
struct KeyCard {
    action: u16,
}

fn main() {
    // let karthik = access("karthik", ProtocolLocations::Office);
    let nandam = access("nandam", ProtocolLocations::WareHouse);
    let karthik = access("karthik", ProtocolLocations::All);
    let luffy: Result<AccessUser, String> = access("luffy", ProtocolLocations::Office);
    let arr = vec![nandam, karthik, luffy];
    for a in arr {
        match a {
            Ok(e) => println!("Access Permision : {:?}", e),
            Err(e) => println!("{e}"),
        }
        println!("+----------------+")
    }
}
