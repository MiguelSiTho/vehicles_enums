#[allow(dead_code)]
#[derive(Debug)]
enum TypeNational {
    Yellow,
    Blue,
    Gray,
    Black,
    Green,
    Red,
}
#[allow(dead_code)]
#[derive(Debug)]
enum TypeMercosur {
    Yellow,
    Blue,
    Gray,
    Black,
    Green,
    Red,
}
#[derive(Debug)]
#[allow(dead_code)]
enum SignType {
    National{
        sign_type: TypeNational,
        indentification: String,
        fu: String,
        city: String,
    },
    Mercosur{
        sign_type: TypeMercosur,
        indentification: String,
        country: String,
    },
}
#[derive(Debug)]
#[allow(dead_code)]
enum VehicleType {
    Car,
    Truck,
    MotorCycle,
    Bus,
}

#[allow(dead_code)]
struct Vehicle {
    model: String,
    year: i32,
    sign: Option<SignType>,
    vehicletype: VehicleType,
}



fn main(){
    let fusca = Vehicle {
        model: "Fusca".to_string(),
        year: 1943,
        sign: Some(SignType::National {
            sign_type: TypeNational::Gray,
            indentification: "ABC0123".to_string(),
            fu: "ES".to_string(),
            city: "Linhares".to_string(),
        }),
        vehicletype: VehicleType::Car,
    };
    let uno = Vehicle {
        model: "Uno".to_string(),
        year: 1985,
        sign: Some(SignType::Mercosur{
            sign_type: TypeMercosur::Blue,
            indentification: "ABC0D12".to_string(),
            country: "Brazil".to_string(),
        }),
        vehicletype: VehicleType::Car,
    };
   
    println!("\n{}", fusca.resume_vehicle());
    println!("\n{}", uno.resume_vehicle());
    
}
#[allow(unused_variables)]
impl Vehicle {
    fn resume_vehicle(&self) -> String {
        format!(
            "This is a {:?}, model {}, year {}, with the sign {:#?}. \nWhat's the type of the sign? \nIt's {:?}",
            &self.vehicletype,
            &self.model,
            &self.year,
            &self.sign,
            match &self.sign {
                Some(SignType::National { sign_type, indentification, fu, city }) => {format!("National")},
                Some(SignType::Mercosur { sign_type, indentification, country }) => {format!("Mercosur")},
                None => {format!("Mercosur")}
            }
        )
    }

}
