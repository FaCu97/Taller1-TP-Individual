use crate::pieza::Pieza;
use std::{env, fs};
mod alfil;
mod caballo;
mod dama;
mod peon_blanco;
mod peon_negro;
mod pieza;
mod rey;
mod torre;

const LIMITE_TABLERO: usize = 7;
pub enum ColorPieza {
    Blanco,
    Negro,
}
pub trait TipoPieza {
    fn movimientos_posibles(&self, posicion: &(usize, usize)) -> Vec<(usize, usize)>;
    fn posicion_es_invalida(&self, posicion: &(usize, usize)) -> bool {
        posicion.0 > LIMITE_TABLERO || posicion.1 > LIMITE_TABLERO
    }
}
pub fn leer_archivo() -> String {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    fs::read_to_string(file_path).expect("Should have been able to read the file")
}
pub fn validar_archivo(contenido_archivo: &String) {
    // aca lanzo errores
    // Formato incorrecto:
    // - El tablero ingresado tiene una cantidad incorrecta de caracteres.
    //   (debe tener 128)
    // - No hay 2 piezas
    // - Piezas mismo color
    // - Caracter distinto a ' ', '\n', R, D, A, C, T, P, r, d, a, c, t, p

    let cantidad_caracteres_archivo = contenido_archivo.len();
    if cantidad_caracteres_archivo != 128 {
        let descripcion_error =
            String::from("Formato del archivo incorrecto, la cantidad de caracteres debe ser 128.");
        imprimir_error(descripcion_error)
    }
}
fn imprimir_error(error: String) {
    println!("Error: {error}");
    std::process::exit(-1);
}
pub fn obtener_piezas_archivo(contenido_archivo: &str) -> Vec<Pieza> {
    let mut piezas: Vec<Pieza> = Vec::new();

    let mut j: usize;

    for (i, linea) in contenido_archivo.lines().enumerate() {
        j = 0;
        let linea_filtrada = linea.replace(' ', "");
        for caracter in linea_filtrada.chars() {
            let pieza: Pieza;
            if caracter != '_' {
                let posicion = (i, j);
                pieza = Pieza::crear_pieza(&caracter, posicion);
                piezas.push(pieza);
            }
            j += 1;
        }
    }
    piezas
}
pub fn evaluar_jugada(pieza1: &Pieza, pieza2: &Pieza) -> String {
    let pieza_blanca: &Pieza;
    let pieza_negra: &Pieza;

    if pieza1.es_blanca() {
        pieza_blanca = pieza1;
        pieza_negra = pieza2;
    } else {
        pieza_blanca = pieza2;
        pieza_negra = pieza1;
    }

    let blanca_captura = pieza_blanca.puede_capturar_pieza(pieza_negra);
    let negra_captura = pieza_negra.puede_capturar_pieza(pieza_blanca);

    if blanca_captura && negra_captura {
        return String::from("E");
    } else if blanca_captura {
        return String::from("B");
    } else if negra_captura {
        return String::from("N");
    }
    String::from("P")
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_input_valido() {
        assert!(true);
    }
}
