use crate::TipoPieza;
pub struct Rey;
const INICIO_TABLERO: usize = 0;
const LIMITE_TABLERO: usize = 7;

impl TipoPieza for Rey {
    fn movimientos_posibles(
        &self,
        posicion: &(usize, usize),
    ) -> Result<Vec<(usize, usize)>, &'static str> {
        self.validar_posicion(posicion)?;

        let mut movimientos_posibles_pieza: Vec<(usize, usize)> = Vec::new();

        movimientos_posibles_pieza =
            agregar_movimientos_laterales(movimientos_posibles_pieza, posicion);
        movimientos_posibles_pieza =
            agregar_movimientos_diagonales(movimientos_posibles_pieza, posicion);
        Ok(movimientos_posibles_pieza)
    }
}
fn agregar_movimientos_laterales(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    let puede_mover_arriba = posicion.1 != INICIO_TABLERO;
    let puede_mover_abajo = posicion.1 != LIMITE_TABLERO;
    let puede_mover_izquierda = posicion.0 != INICIO_TABLERO;
    let puede_mover_derecha = posicion.0 != LIMITE_TABLERO;

    if puede_mover_arriba {
        movimientos_posibles_pieza.push((posicion.0, posicion.1 - 1));
    }
    if puede_mover_abajo {
        movimientos_posibles_pieza.push((posicion.0, posicion.1 + 1));
    }
    if puede_mover_izquierda {
        movimientos_posibles_pieza.push((posicion.0 - 1, posicion.1));
    }
    if puede_mover_derecha {
        movimientos_posibles_pieza.push((posicion.0 + 1, posicion.1));
    }

    movimientos_posibles_pieza
}

fn agregar_movimientos_diagonales(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    let puede_mover_arriba = posicion.1 != INICIO_TABLERO;
    let puede_mover_abajo = posicion.1 != LIMITE_TABLERO;
    let puede_mover_izquierda = posicion.0 != INICIO_TABLERO;
    let puede_mover_derecha = posicion.0 != LIMITE_TABLERO;

    if puede_mover_derecha && puede_mover_arriba {
        movimientos_posibles_pieza.push((posicion.0 + 1, posicion.1 - 1));
    }
    if puede_mover_izquierda && puede_mover_arriba {
        movimientos_posibles_pieza.push((posicion.0 - 1, posicion.1 - 1));
    }
    if puede_mover_derecha && puede_mover_abajo {
        movimientos_posibles_pieza.push((posicion.0 + 1, posicion.1 + 1));
    }
    if puede_mover_izquierda && puede_mover_abajo {
        movimientos_posibles_pieza.push((posicion.0 - 1, posicion.1 + 1));
    }
    movimientos_posibles_pieza
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rey_puede_mover_arriba_desde_1_1() {
        let pieza = Rey;
        let posicion = (1, 1);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(1, 0)));
    }
    #[test]
    fn test_rey_puede_mover_arriba_desde_4_3() {
        let pieza = Rey;
        let posicion = (4, 3);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(4, 2)));
        assert!(!movimientos.contains(&(1, 0)));
    }
    #[test]
    fn test_rey_puede_mover_abajo() {
        let pieza = Rey;
        let posicion = (1, 1);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(1, 2)));
    }
    #[test]
    fn test_rey_puede_mover_izquierda() {
        let pieza = Rey;
        let posicion = (1, 1);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(0, 1)));
    }
    #[test]
    fn test_rey_puede_mover_derecha() {
        let pieza = Rey;
        let posicion = (1, 1);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(2, 1)));
    }
    #[test]
    fn test_rey_puede_mover_diagonal_arriba_derecha() {
        let pieza = Rey;
        let posicion = (1, 1);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(2, 0)));
    }
    #[test]
    fn test_rey_puede_mover_diagonal_arriba_izquierda() {
        let pieza = Rey;
        let posicion = (1, 1);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(0, 0)));
    }
    #[test]
    fn test_rey_puede_mover_diagonal_abajo_derecha() {
        let pieza = Rey;
        let posicion = (1, 1);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(2, 2)));
    }
    #[test]
    fn test_rey_puede_mover_diagonal_abajo_izquierda() {
        let pieza = Rey;
        let posicion = (1, 1);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(0, 2)));
    }
    #[test]
    fn test_posicion_invalida() {
        let pieza = Rey;
        let posicion = (1, 10);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        assert!(movimientos_result.is_err());
    }

    #[test]
    fn test_rey_no_puede_mover_abajo_desde_0_7() {
        let pieza = Rey;
        let posicion = (0, 7);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(!movimientos.contains(&(0, 8)));
    }

    #[test]
    fn test_rey_no_puede_mover_derecha_desde_7_0() {
        let pieza = Rey;
        let posicion = (7, 0);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(!movimientos.contains(&(8, 0)));
    }
}
