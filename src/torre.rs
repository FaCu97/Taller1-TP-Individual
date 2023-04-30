use crate::TipoPieza;
pub struct Torre;
const INICIO_TABLERO: usize = 0;
const LIMITE_TABLERO: usize = 7;

impl TipoPieza for Torre {
    fn movimientos_posibles(&self, posicion: &(usize, usize)) -> Result<Vec<(usize, usize)>, &'static str> {
        self.validar_posicion(posicion)?;

        let mut movimientos_posibles_pieza: Vec<(usize, usize)> = Vec::new();

        movimientos_posibles_pieza =
            agregar_movimientos_posibles_hacia_abajo(movimientos_posibles_pieza, posicion);
        movimientos_posibles_pieza =
            agregar_movimientos_posibles_hacia_arriba(movimientos_posibles_pieza, posicion);
        movimientos_posibles_pieza =
            agregar_movimientos_posibles_hacia_derecha(movimientos_posibles_pieza, posicion);
        movimientos_posibles_pieza =
            agregar_movimientos_posibles_hacia_izquierda(movimientos_posibles_pieza, posicion);

        Ok(movimientos_posibles_pieza)
    }
}
fn agregar_movimientos_posibles_hacia_arriba(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut j = posicion.1;
    while j > INICIO_TABLERO {
        j -= 1;
        movimientos_posibles_pieza.push((posicion.0, j));
    }
    movimientos_posibles_pieza
}
fn agregar_movimientos_posibles_hacia_derecha(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut i = posicion.0;
    while i < LIMITE_TABLERO {
        i += 1;
        movimientos_posibles_pieza.push((i, posicion.1));
    }
    movimientos_posibles_pieza
}

fn agregar_movimientos_posibles_hacia_abajo(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    // Hacia abajo
    let mut j = posicion.1;
    while j < LIMITE_TABLERO {
        j += 1;
        movimientos_posibles_pieza.push((posicion.0, j));
    }
    movimientos_posibles_pieza
}

fn agregar_movimientos_posibles_hacia_izquierda(
    mut movimientos_posibles_pieza: Vec<(usize, usize)>,
    posicion: &(usize, usize),
) -> Vec<(usize, usize)> {
    let mut i = posicion.0;
    while i > INICIO_TABLERO {
        i -= 1;
        movimientos_posibles_pieza.push((i, posicion.1));
    }
    movimientos_posibles_pieza
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torre_puede_mover_arriba_desde_4_4() {
        let pieza = Torre;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => {return}
        };
        assert!(movimientos.contains(&(4, 3)));
    }
    #[test]
    fn test_torre_puede_mover_dos_posiciones_arriba_desde_4_4() {
        let pieza = Torre;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => {return}
        };
        assert!(movimientos.contains(&(4, 2)));
    }
    #[test]
    fn test_torre_puede_mover_abajo_desde_4_4() {
        let pieza = Torre;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => {return}
        };
        assert!(movimientos.contains(&(4, 5)));
    }
    #[test]
    fn test_torre_puede_mover_derecha_desde_4_4() {
        let pieza = Torre;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => {return}
        };
        assert!(movimientos.contains(&(5, 4)));
    }
    #[test]
    fn test_torre_puede_mover_izquierda_desde_4_4() {
        let pieza = Torre;
        let posicion = (4, 4);
        let movimientos_result = pieza.movimientos_posibles(&posicion);
        let movimientos = match movimientos_result {
            Ok(movimientos) => movimientos,
            Err(_e) => {return}
        };
        assert!(movimientos.contains(&(3, 4)));
    }
}
