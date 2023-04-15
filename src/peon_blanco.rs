use crate::TipoPieza;
pub struct PeonBlanco;
const INICIO_TABLERO: usize = 0;
const LIMITE_TABLERO: usize = 7;

impl TipoPieza for PeonBlanco {
    fn movimientos_posibles(&self, posicion: &(usize, usize)) -> Vec<(usize, usize)> {
        if self.posicion_es_invalida(posicion) {
            return Vec::new(); //Lanzo error
        }
        let mut movimientos_posibles_pieza: Vec<(usize, usize)> = Vec::new();

        if posicion.1 == INICIO_TABLERO {
            return movimientos_posibles_pieza;
        }

        movimientos_posibles_pieza.push((posicion.0, posicion.1 - 1));

        if posicion.1 == LIMITE_TABLERO - 1 {
            movimientos_posibles_pieza.push((posicion.0, posicion.1 - 2));
        }
        movimientos_posibles_pieza
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peon_blanco_puede_mover_arriba_desde_4_4() {
        let pieza = PeonBlanco;
        let posicion = (4, 4);
        let movimientos = pieza.movimientos_posibles(&posicion);
        assert!(movimientos.contains(&(4, 3)));
    }
    #[test]
    fn test_peon_blanco_puede_mover_arriba_desde_2_5() {
        let pieza = PeonBlanco;
        let posicion = (2, 5);
        let movimientos = pieza.movimientos_posibles(&posicion);
        assert!(movimientos.contains(&(2, 4)));
    }
    #[test]
    fn test_peon_blanco_puede_mover_doble_arriba_desde_4_6() {
        let pieza = PeonBlanco;
        let posicion = (4, 6);
        let movimientos = pieza.movimientos_posibles(&posicion);
        assert!(movimientos.contains(&(4, 4)));
    }
    #[test]
    fn test_peon_blanco_no_puede_mover_doble_arriba_desde_4_3() {
        let pieza = PeonBlanco;
        let posicion = (4, 3);
        let movimientos = pieza.movimientos_posibles(&posicion);
        assert!(!movimientos.contains(&(4, 1)));
    }
    #[test]
    fn test_peon_blanco_no_puede_mover_abajo_desde_4_4() {
        let pieza = PeonBlanco;
        let posicion = (4, 4);
        let movimientos = pieza.movimientos_posibles(&posicion);
        assert!(!movimientos.contains(&(4, 5)));
    }
    #[test]
    fn test_peon_blanco_no_puede_mover_arriba_desde_0_0() {
        let pieza = PeonBlanco;
        let posicion = (0, 0);
        let _movimientos = pieza.movimientos_posibles(&posicion);
        //assert!(!movimientos.contains(&(0, -1)));
        //capturo error
    }
}
