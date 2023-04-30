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
pub const LIMITE_TABLERO: usize = 7;

pub enum ColorPieza {
    Blanco,
    Negro,
}
pub trait TipoPieza {
    /// Recibe una posición y devuelve un vector con todas las posiciones posibles de la pieza.
    ///
    /// Devuelve error si la posición recibida es inválida.
    fn movimientos_posibles(
        &self,
        posicion: &(usize, usize),
    ) -> Result<Vec<(usize, usize)>, &'static str>;
    /// Devuelve error si la posición recibida es inválida.
    fn validar_posicion(&self, posicion: &(usize, usize)) -> Result<(), &'static str> {
        if posicion.0 > LIMITE_TABLERO || posicion.1 > LIMITE_TABLERO {
            return Err("Posicion inválida");
        }
        Ok(())
    }
}

/// Lee el archivo cuya ruta se indica en los argumentos.
///
/// Devuelve error si no se puede leer el archivo.
pub fn leer_archivo() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let content_result = fs::read_to_string(file_path);
    let content = match content_result {
        Ok(file) => file,
        Err(_e) => return Err("Error al leer el archivo"),
    };
    Ok(content)
}

/// Realiza las validaciones sobre el string recibido.
/// # Error
/// Devuelve error en los siguientes casos:
/// - La cantidad de caracteres es incorrecta.
/// - Se encuentra un caracter inválido.
/// - No hay dos caracteres de pieza.
/// - Los caracteres no representan piezas de distinto color
pub fn validar_archivo(contenido_archivo: &String) -> Result<(), &'static str> {
    let caracteres_validos = [
        ' ', '\n', '_', 'p', 'P', 'r', 'R', 'd', 'D', 't', 'T', 'a', 'A', 'c', 'C',
    ];
    let caracteres_piezas = ['p', 'P', 'r', 'R', 'd', 'D', 't', 'T', 'a', 'A', 'c', 'C'];
    let cantidad_caracteres_archivo = contenido_archivo.len();
    if cantidad_caracteres_archivo != 128 {
        return Err("Formato del archivo incorrecto, la cantidad de caracteres debe ser 128.");
    }

    let mut piezas: Vec<char> = Vec::new();
    for caracter in contenido_archivo.chars() {
        if !caracteres_validos.contains(&caracter) {
            return Err("Formato del archivo incorrecto, el caracter no es válido.");
        }
        if caracteres_piezas.contains(&caracter) {
            piezas.push(caracter);
        }
    }

    if piezas.len() != 2 {
        return Err("Debe haber 2 piezas en el archivo");
    }

    if (piezas[0].is_lowercase() == (piezas[1]).is_lowercase())
        || piezas[0].is_uppercase() == piezas[1].is_uppercase()
    {
        return Err("Las piezas deben ser de distinto color");
    }
    Ok(())
}

/// Recibe el contenido del archivo y devuelve un array con las piezas contenidas.
///
/// Devuelve error si no se puede crear una pieza válida.
pub fn obtener_piezas_archivo(contenido_archivo: &str) -> Result<Vec<Pieza>, &'static str> {
    let mut piezas: Vec<Pieza> = Vec::new();

    let mut j: usize;

    for (i, linea) in contenido_archivo.lines().enumerate() {
        j = 0;
        let linea_filtrada = linea.replace(' ', "");
        for caracter in linea_filtrada.chars() {
            let pieza: Pieza;
            if caracter != '_' {
                let posicion = (i, j);
                pieza = Pieza::crear_pieza(&caracter, posicion)?;
                piezas.push(pieza);
            }
            j += 1;
        }
    }
    Ok(piezas)
}

/// Recibe dos piezas y evalúa el ganador e imprime el resultado.
///
/// # Resultados pos0ibles:
///  - "E" si hay empate
///  - "B" si la pieza blanca captura
///  - "N" si la pieza negra captura
///  - "P" sin ninguna captura
///
/// # Error
/// Devuelve error si no se pueden obtener los movimientos de una pieza.
pub fn evaluar_jugada(pieza1: &Pieza, pieza2: &Pieza) -> Result<String, &'static str> {
    let pieza_blanca: &Pieza;
    let pieza_negra: &Pieza;

    if pieza1.es_blanca() {
        pieza_blanca = pieza1;
        pieza_negra = pieza2;
    } else {
        pieza_blanca = pieza2;
        pieza_negra = pieza1;
    }

    let blanca_captura = pieza_blanca.puede_capturar_pieza(pieza_negra)?;
    let negra_captura = pieza_negra.puede_capturar_pieza(pieza_blanca)?;

    if blanca_captura && negra_captura {
        return Ok(String::from("E"));
    } else if blanca_captura {
        return Ok(String::from("B"));
    } else if negra_captura {
        return Ok(String::from("N"));
    }
    Ok(String::from("P"))
}

#[cfg(test)]
mod tests {

    use crate::validar_archivo;

    #[test]
    fn test_longitud_archivo_distinto_a_128_lanza_error() {
        let contenido_archivo = String::from("123456");
        let result = validar_archivo(&contenido_archivo);
        assert_eq!(
            result,
            Err("Formato del archivo incorrecto, la cantidad de caracteres debe ser 128.")
        );
    }

    #[test]
    fn test_archivo_con_caracter_invalido_lanza_error() {
        let contenido_archivo = String::from("6").repeat(128);
        let result = validar_archivo(&contenido_archivo);
        assert_eq!(
            result,
            Err("Formato del archivo incorrecto, el caracter no es válido.")
        );
    }

    #[test]
    fn test_archivo_con_una_pieza() {
        let contenido_archivo = String::from("_").repeat(127) + &String::from("R");
        let result = validar_archivo(&contenido_archivo);
        assert_eq!(result, Err("Debe haber 2 piezas en el archivo"));
    }

    #[test]
    fn test_archivo_con_dos_piezas_mismo_color() {
        let contenido_archivo =
            String::from("_").repeat(126) + &String::from("R") + &String::from("P");
        let result = validar_archivo(&contenido_archivo);
        assert_eq!(result, Err("Las piezas deben ser de distinto color"));
    }
}
