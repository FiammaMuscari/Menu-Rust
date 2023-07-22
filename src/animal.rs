pub trait Animal {
    fn to_string(&self) -> String;
}

#[derive(Debug)]
pub struct Perro {
    pub color_ojos: String,
    pub pelaje: String,
    pub edad: u8,
    pub fecha_guardado: String,
    pub nombre: String,
}

#[derive(Debug)]
pub struct Gato {
    pub color_ojos: String,
    pub pelaje: String,
    pub edad: u8,
    pub fecha_guardado: String,
    pub nombre: String,
}

impl Animal for Perro {
    fn to_string(&self) -> String {
        format!(
            "Perro: Nombre: {}, Color pelaje:{}, Tipo pelaje: {}, Edad: {}, Fecha de ingreso: {}",
            self.nombre, self.color_ojos, self.pelaje, self.edad, self.fecha_guardado
        )
    }
}

impl Animal for Gato {
    fn to_string(&self) -> String {
        format!(
            "Gato: Nombre: {}, Color pelaje:{}, Tipo pelaje: {}, Edad: {}, Fecha de ingreso: {}",
            self.nombre, self.color_ojos, self.pelaje, self.edad, self.fecha_guardado
        )
    }
}
