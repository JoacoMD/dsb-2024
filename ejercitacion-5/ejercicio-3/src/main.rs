#[derive(PartialEq, Eq, Clone)]
enum Color {
    Rojo,
    Azul,
    Verde,
    Amarillo, 
    Blanco,
    Negro
}

struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    capacidad: u32,
    autos: Vec<Auto>,
}

#[derive(Clone)]
struct Auto {
    marca: String,
    modelo: String,
    anio: u32,
    precio_bruto: f32,
    color: Color 
}

impl Auto {
    fn new(marca: String, modelo: String, anio: u32, precio_bruto: f32, color: Color) -> Auto {
        Auto {
            marca,
            modelo,
            anio,
            precio_bruto,
            color
        }
    }

    fn calcular_precio(&self) -> f32 {
        let mut porcentaje_a_recargar = 0;
        if self.anio < 2000 {
            porcentaje_a_recargar -= 5;
        }
        if self.marca.eq_ignore_ascii_case("BMW") {
            porcentaje_a_recargar += 15;
        }
        match self.color {
            Color::Amarillo | Color::Azul | Color::Rojo => {
                porcentaje_a_recargar += 25;
            },
            _ => {
                porcentaje_a_recargar -= 10;
            }
        }
        return (self.precio_bruto * (1.0 + porcentaje_a_recargar as f32 / 100.0)).round();
    }

    fn equals(&self, auto: &Auto) -> bool {
        self.marca.eq_ignore_ascii_case(&auto.marca) && self.modelo.eq_ignore_ascii_case(&auto.modelo) && self.anio == auto.anio && self.precio_bruto == auto.precio_bruto && self.color == auto.color
    }
}

impl ConcesionarioAuto {
    fn new(nombre: String, direccion: String, capacidad: u32) -> ConcesionarioAuto {
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad,
            autos: Vec::new()
        }
    }

    fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.len() < self.capacidad as usize {
            self.autos.push(auto);
            true
        } else {
            println!("No hay espacio para agregar el auto");
            false
        }
    }

    fn eliminar_auto(&mut self, auto: &Auto) -> bool {
        if let Some(pos) = self.autos.iter().position(|a| a.equals(&auto)) {
            self.autos.remove(pos);
            true
        } else {
            println!("No se encontró el auto");
            false
        } 
    }

    fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        for a in &self.autos {
            if a.marca.eq_ignore_ascii_case(&auto.marca) && a.modelo.eq_ignore_ascii_case(&auto.modelo) {
                return Some(a);
            }
        }
        None
    }

}

fn main() {

}

#[test]
fn test_concesionario_agregar() {
    let auto1 = Auto::new("Ford".to_string(), "Fiesta".to_string(), 2014, 100000.0, Color::Rojo);
    let auto2 = Auto::new("BMW".to_string(), "Serie 3".to_string(), 2018, 200000.0, Color::Negro);
    let mut concesionario = ConcesionarioAuto::new("Carros".to_string(), "Calle 1".to_string(), 1);
    assert!(concesionario.agregar_auto(auto1));
    assert!(!concesionario.agregar_auto(auto2));
}

#[test]
fn test_concesionario_buscar() {
    let auto1 = Auto::new("Ford".to_string(), "Fiesta".to_string(), 2014, 100000.0, Color::Rojo);
    let mut concesionario = ConcesionarioAuto::new("Carros".to_string(), "Calle 1".to_string(), 1);
    concesionario.agregar_auto(auto1.clone());
    assert!(concesionario.buscar_auto(&auto1).is_some());
    let buscar = Auto::new("Nissan".to_string(), "Kicks".to_string(), 2014, 100000.0, Color::Rojo);
    assert!(concesionario.buscar_auto(&buscar).is_none())
}

#[test]
fn test_concesionario_eliminar() {
    let auto1 = Auto::new("Ford".to_string(), "Fiesta".to_string(), 2014, 100000.0, Color::Rojo);
    let mut concesionario = ConcesionarioAuto::new("Carros".to_string(), "Calle 1".to_string(), 1);
    assert!(concesionario.agregar_auto(auto1.clone()));
    assert!(concesionario.eliminar_auto(&auto1));
    assert!(!concesionario.eliminar_auto(&auto1));
}

#[test]
fn test_precio_auto_bmw_negro_2018() {
    let auto = Auto::new("BMW".to_string(), "Serie 3".to_string(), 2018, 100.0, Color::Negro);
    assert_eq!(auto.calcular_precio(), 105.0)
}

#[test]
fn test_precio_auto_toyota_rojo_1999() {
    let auto = Auto::new("Toyota".to_string(), "RAV".to_string(), 1999, 100.0, Color::Rojo);
    assert_eq!(auto.calcular_precio(), 120.0)
}

#[test]
fn test_auto_equals() {
    let a = Auto::new("Toyota".to_string(), "RAV".to_string(), 1999, 100.0, Color::Rojo);
    let b = Auto::new("Toyota".to_string(), "RAV".to_string(), 1999, 100.0, Color::Rojo);
    assert!(a.equals(&b))
}

#[test]
fn test_auto_not_equals() {
    let a = Auto::new("Toyota".to_string(), "RAV".to_string(), 1999, 100.0, Color::Rojo);
    let b = Auto::new("Toyota".to_string(), "RAV".to_string(), 2019, 100.0, Color::Negro);
    assert!(!a.equals(&b))
}


