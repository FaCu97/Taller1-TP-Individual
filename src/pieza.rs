use crate::{ColorPieza, TipoPieza};

use crate::{alfil, caballo, dama, peon_blanco, peon_negro, rey, torre};

const LIMITE_TABLERO: usize = 7;

pub struct Pieza {
    color: ColorPieza,
    posicion: (usize, usize),
    pub tipo_pieza: Box<dyn TipoPieza>,
}

impl Pieza {
    ///Recibe un caracter y posición. Devuelve la pieza correspondiente.
    ///
    /// Devuelve error si no se puede crear una pieza válida.
    pub fn crear_pieza(
        caracter: &char,
        posicion_ficha: (usize, usize),
    ) -> Result<Pieza, &'static str> {
        let tipo_pieza = Self::tipo_pieza(caracter)?;
        Self::validar_posicion(&posicion_ficha)?;
        Ok(Pieza {
            color: Self::color_pieza(caracter),
            tipo_pieza,
            posicion: posicion_ficha,
        })
    }

    ///Recibe un caracter y devuelve el color correspondiente.
    fn color_pieza(caracter: &char) -> ColorPieza {
        if caracter.is_uppercase() {
            return ColorPieza::Negro;
        }
        ColorPieza::Blanco
    }

    /// Recibe un caracter y devuelve el tipo de pieza correspondiente al mismo.
    ///
    /// Devuelve error en caso que el caracter no corresponda a ningún tipo de pieza.
    fn tipo_pieza(caracter: &char) -> Result<Box<dyn TipoPieza>, &'static str> {
        match caracter {
            'R' | 'r' => Ok(Box::new(rey::Rey)),
            'D' | 'd' => Ok(Box::new(dama::Dama)),
            'C' | 'c' => Ok(Box::new(caballo::Caballo)),
            'A' | 'a' => Ok(Box::new(alfil::Alfil)),
            'T' | 't' => Ok(Box::new(torre::Torre)),
            'p' => Ok(Box::new(peon_blanco::PeonBlanco)),
            'P' => Ok(Box::new(peon_negro::PeonNegro)),
            _ => Err("No se puede crear la pieza con el caracter"),
        }
    }

    /// Devuelve true si el color de la pieza es blanco.
    pub fn es_blanca(&self) -> bool {
        matches!(self.color, ColorPieza::Blanco)
    }

    /// Recibe una pieza enemiga y devuelve un booleano dependiendo si dicha pieza puede ser capturada.
    ///
    /// Devuelve error si no se pueden obtener los movimientos de la pieza
    pub fn puede_capturar_pieza(&self, pieza_enemiga: &Pieza) -> Result<bool, &'static str> {
        let movimientos_posibles = self.tipo_pieza.movimientos_posibles(&self.posicion)?;
        Ok(pieza_enemiga.puede_ser_capturada(movimientos_posibles))
    }

    /// Recibe un un vector de posiciones y devuelve un booleano dependiendo si la ubicación de
    /// la pieza se encuentra en el vector.
    fn puede_ser_capturada(&self, movimientos_posibles: Vec<(usize, usize)>) -> bool {
        movimientos_posibles.contains(&self.posicion)
    }

    /// Recibe una posición y devuelve error en caso que sea inválida.
    fn validar_posicion(posicion: &(usize, usize)) -> Result<(), &'static str> {
        if posicion.0 > LIMITE_TABLERO || posicion.1 > LIMITE_TABLERO {
            return Err("Posicion inválida");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_se_puede_obtener_tipo_pieza_con_caracter_invalido() {
        let caracter = 'G';
        let tipo_pieza = Pieza::tipo_pieza(&caracter);
        assert!(tipo_pieza.is_err());
    }

    #[test]
    fn test_no_se_puede_crear_pieza_con_caracter_invalido() {
        let caracter = 'G';
        let pieza = Pieza::crear_pieza(&caracter, (0, 0));
        assert!(pieza.is_err());
    }

    #[test]
    fn test_no_se_puede_crear_pieza_con_posicion_invalida() {
        let caracter = 'R';
        let posicion = (0, 42);
        let pieza = Pieza::crear_pieza(&caracter, posicion);
        assert!(pieza.is_err());
    }
}
