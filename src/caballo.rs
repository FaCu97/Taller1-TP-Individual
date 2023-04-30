use crate::TipoPieza;
pub struct Caballo;
const INICIO_TABLERO: usize = 0;
const LIMITE_TABLERO: usize = 7;

impl TipoPieza for Caballo {
    fn movimientos_posibles(
        &self,
        posicion: &(usize, usize),
    ) -> Result<Vec<(usize, usize)>, &'static str> {
        self.validar_posicion(posicion)?;
        let mut movimientos_posibles_pieza: Vec<(usize, usize)> = Vec::new();

        movimientos_posibles_pieza =
            agregar_movimientos_hacia_arriba(movimientos_posibles_pieza, posicion);
        movimientos_posibles_pieza =
            agregar_movimientos_hacia_abajo(movimientos_posibles_pieza, posicion);

        Ok(movimientos_posibles_pieza)
    }
}
fn agregar_movimientos_hacia_arriba(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    let puede_mover_arriba = posicion.1 >= INICIO_TABLERO + 2;
    let puede_mover_vertical_izquierda = posicion.0 > INICIO_TABLERO;
    let puede_mover_vertical_derecha = posicion.0 < LIMITE_TABLERO;
    let puede_mover_lateral_arriba = posicion.1 > INICIO_TABLERO;
    let puede_mover_izquierda = posicion.0 >= INICIO_TABLERO + 2;
    let puede_mover_derecha = posicion.0 <= LIMITE_TABLERO - 2;
    if puede_mover_arriba && puede_mover_vertical_izquierda {
        movimientos_posibles_pieza.push((posicion.0 - 1, posicion.1 - 2));
    }
    if puede_mover_arriba && puede_mover_vertical_derecha {
        movimientos_posibles_pieza.push((posicion.0 + 1, posicion.1 - 2));
    }
    if puede_mover_izquierda && puede_mover_lateral_arriba {
        movimientos_posibles_pieza.push((posicion.0 - 2, posicion.1 - 1));
    }
    if puede_mover_derecha && puede_mover_lateral_arriba {
        movimientos_posibles_pieza.push((posicion.0 + 2, posicion.1 - 1));
    }
    movimientos_posibles_pieza
}
fn agregar_movimientos_hacia_abajo(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    let puede_mover_vertical_izquierda = posicion.0 > INICIO_TABLERO;
    let puede_mover_vertical_derecha = posicion.0 < LIMITE_TABLERO;
    let puede_mover_abajo = posicion.1 <= LIMITE_TABLERO - 2;
    let puede_mover_izquierda = posicion.0 >= INICIO_TABLERO + 2;
    let puede_mover_derecha = posicion.0 <= LIMITE_TABLERO - 2;
    let puede_mover_lateral_abajo = posicion.1 < LIMITE_TABLERO;

    if puede_mover_izquierda && puede_mover_lateral_abajo {
        movimientos_posibles_pieza.push((posicion.0 - 2, posicion.1 + 1));
    }

    if puede_mover_derecha && puede_mover_lateral_abajo {
        movimientos_posibles_pieza.push((posicion.0 + 2, posicion.1 + 1));
    }
    if puede_mover_abajo && puede_mover_vertical_izquierda {
        movimientos_posibles_pieza.push((posicion.0 - 1, posicion.1 + 2));
    }
    if puede_mover_abajo && puede_mover_vertical_derecha {
        movimientos_posibles_pieza.push((posicion.0 + 1, posicion.1 + 2));
    }
    movimientos_posibles_pieza
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caballo_puede_mover_arriba_izquierda_desde_4_4() {
        let pieza = Caballo;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(3, 2)));
    }
    #[test]
    fn test_caballo_puede_mover_arriba_derecha_desde_4_4() {
        let pieza = Caballo;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(5, 2)));
    }
    #[test]
    fn test_caballo_puede_mover_izquierda_arriba_desde_4_4() {
        let pieza = Caballo;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(2, 3)));
    }
    #[test]
    fn test_caballo_puede_mover_izquierda_abajo_desde_4_4() {
        let pieza = Caballo;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(2, 5)));
    }
    #[test]
    fn test_caballo_puede_mover_derecha_arriba_desde_4_4() {
        let pieza = Caballo;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(6, 3)));
    }
    #[test]
    fn test_caballo_puede_mover_derecha_abajo_desde_4_4() {
        let pieza = Caballo;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(6, 5)));
    }
    #[test]
    fn test_caballo_puede_mover_abajo_izquierda_desde_4_4() {
        let pieza = Caballo;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(3, 6)));
    }
    #[test]
    fn test_caballo_puede_mover_abajo_derecha_desde_4_4() {
        let pieza = Caballo;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(5, 6)));
    }
    #[test]
    fn test_caballo_puede_mover_abajo_derecha_desde_1_2() {
        let pieza = Caballo;
        let posicion = (1, 2);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(2, 4)));
    }

    #[test]
    fn test_caballo_puede_mover_arriba_derecha_desde_0_2() {
        let pieza = Caballo;
        let posicion = (0, 2);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(1, 0)));

        assert!(matches!(1, 1));
    }
}
