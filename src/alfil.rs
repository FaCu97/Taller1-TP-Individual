use crate::TipoPieza;
pub struct Alfil;
const INICIO_TABLERO: usize = 0;
const LIMITE_TABLERO: usize = 7;

impl TipoPieza for Alfil {
    fn movimientos_posibles(
        &self,
        posicion: &(usize, usize),
    ) -> Result<Vec<(usize, usize)>, &'static str> {
        self.validar_posicion(posicion)?;
        let mut movimientos_posibles_pieza: Vec<(usize, usize)> = Vec::new();

        movimientos_posibles_pieza = agregar_movimientos_posibles_diagonal_inferior_derecha(
            movimientos_posibles_pieza,
            posicion,
        );
        movimientos_posibles_pieza = agregar_movimientos_posibles_diagonal_inferior_izquierda(
            movimientos_posibles_pieza,
            posicion,
        );
        movimientos_posibles_pieza = agregar_movimientos_posibles_diagonal_superior_derecha(
            movimientos_posibles_pieza,
            posicion,
        );
        movimientos_posibles_pieza = agregar_movimientos_posibles_diagonal_superior_izquierda(
            movimientos_posibles_pieza,
            posicion,
        );

        Ok(movimientos_posibles_pieza)
    }
}
fn agregar_movimientos_posibles_diagonal_inferior_derecha(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut i = posicion.0;
    let mut j = posicion.1;
    while i < LIMITE_TABLERO && j < LIMITE_TABLERO {
        i += 1;
        j += 1;
        movimientos_posibles_pieza.push((i, j));
    }
    movimientos_posibles_pieza
}
fn agregar_movimientos_posibles_diagonal_inferior_izquierda(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut i = posicion.0;
    let mut j = posicion.1;
    while i > INICIO_TABLERO && j < LIMITE_TABLERO {
        i -= 1;
        j += 1;
        movimientos_posibles_pieza.push((i, j));
    }
    movimientos_posibles_pieza
}
fn agregar_movimientos_posibles_diagonal_superior_derecha(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut i = posicion.0;
    let mut j = posicion.1;
    while i < LIMITE_TABLERO && j > INICIO_TABLERO {
        i += 1;
        j -= 1;
        movimientos_posibles_pieza.push((i, j));
    }
    movimientos_posibles_pieza
}
fn agregar_movimientos_posibles_diagonal_superior_izquierda(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut i = posicion.0;
    let mut j = posicion.1;
    while i > INICIO_TABLERO && j > INICIO_TABLERO {
        i -= 1;
        j -= 1;
        movimientos_posibles_pieza.push((i, j));
    }
    movimientos_posibles_pieza
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alfil_puede_mover_diagonal_superior_izquierda_desde_4_4() {
        let pieza = Alfil;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(3, 3)));
    }
    #[test]
    fn test_alfil_puede_mover_diagonal_superior_derecha_desde_4_4() {
        let pieza = Alfil;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(5, 3)));
    }
    #[test]
    fn test_alfil_puede_mover_diagonal_inferior_izquierda_desde_4_4() {
        let pieza = Alfil;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(3, 5)));
    }
    #[test]
    fn test_alfil_puede_mover_diagonal_inferior_derecha_desde_4_4() {
        let pieza = Alfil;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(5, 5)));
    }
    #[test]
    fn test_alfil_puede_mover_multiple_diagonal_inferior_derecha_desde_4_4() {
        let pieza = Alfil;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => return,
        };
        assert!(movimientos.contains(&(7, 7)));
    }
}
