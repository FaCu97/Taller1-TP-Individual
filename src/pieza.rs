use crate::{ColorPieza, TipoPieza};

use crate::{alfil, caballo, dama, peon_blanco, peon_negro, rey, torre};

pub struct Pieza {
    color: ColorPieza,
    posicion: (usize, usize),
    pub tipo_pieza: Box<dyn TipoPieza>,
}

impl Pieza {
    pub fn crear_pieza(caracter: &char, posicion_ficha: (usize, usize)) -> Pieza {
        Pieza {
            color: Self::color_pieza(caracter),
            tipo_pieza: Self::tipo_pieza(caracter),
            posicion: posicion_ficha,
        }
    }

    fn color_pieza(caracter: &char) -> ColorPieza {
        if caracter.is_uppercase() {
            return ColorPieza::Negro;
        }
        ColorPieza::Blanco
    }

    fn tipo_pieza(caracter: &char) -> Box<dyn TipoPieza> {
        match caracter {
            'R' | 'r' => Box::new(rey::Rey),
            'D' | 'd' => Box::new(dama::Dama),
            'C' | 'c' => Box::new(caballo::Caballo),
            'A' | 'a' => Box::new(alfil::Alfil),
            'T' | 't' => Box::new(torre::Torre),
            'p' => Box::new(peon_blanco::PeonBlanco),
            'P' => Box::new(peon_negro::PeonNegro),
            _ => Box::new(peon_blanco::PeonBlanco), //Lanzo error, lo pongo asi para evitar el err
        }
    }
    pub fn es_blanca(&self) -> bool {
        matches!(self.color, ColorPieza::Blanco)
    }
    pub fn puede_capturar_pieza(&self, pieza_enemiga: &Pieza) -> bool {
        let movimientos_posibles = self.tipo_pieza.movimientos_posibles(&self.posicion);
        pieza_enemiga.puede_ser_capturada(movimientos_posibles)
    }
    fn puede_ser_capturada(&self, movimientos_posibles: Vec<(usize, usize)>) -> bool {
        movimientos_posibles.contains(&self.posicion)
    }
}
