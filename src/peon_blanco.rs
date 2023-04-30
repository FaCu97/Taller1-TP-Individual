use crate::TipoPieza;
pub struct PeonBlanco;
const INICIO_TABLERO: usize = 0;
const LIMITE_TABLERO: usize = 7;

impl TipoPieza for PeonBlanco {
    fn movimientos_posibles(
        &self,
        posicion: &(usize, usize),
    ) -> Result<Vec<(usize, usize)>, &'static str> {
        self.validar_posicion(posicion)?;

        let mut movimientos_posibles_pieza: Vec<(usize, usize)> = Vec::new();

        if posicion.1 == INICIO_TABLERO {
            return Ok(movimientos_posibles_pieza);
        }

        movimientos_posibles_pieza.push((posicion.0, posicion.1 - 1));

        if posicion.1 == LIMITE_TABLERO - 1 {
            movimientos_posibles_pieza.push((posicion.0, posicion.1 - 2));
        }
        Ok(movimientos_posibles_pieza)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peon_blanco_puede_mover_arriba_desde_4_4() {
        let pieza = PeonBlanco;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(4, 3)));
    }
    #[test]
    fn test_peon_blanco_puede_mover_arriba_desde_2_5() {
        let pieza = PeonBlanco;
        let posicion = (2, 5);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(2, 4)));
    }
    #[test]
    fn test_peon_blanco_puede_mover_doble_arriba_desde_4_6() {
        let pieza = PeonBlanco;
        let posicion = (4, 6);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(4, 4)));
    }
    #[test]
    fn test_peon_blanco_no_puede_mover_doble_arriba_desde_4_3() {
        let pieza = PeonBlanco;
        let posicion = (4, 3);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(!movimientos.contains(&(4, 1)));
    }
    #[test]
    fn test_peon_blanco_no_puede_mover_abajo_desde_4_4() {
        let pieza = PeonBlanco;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(!movimientos.contains(&(4, 5)));
    }
}
