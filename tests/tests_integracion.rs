use tp_individual::{evaluar_jugada, obtener_piezas_archivo};

#[test]
fn test_integracion_negras_capturan() {
    let input = "_ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ D _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ t _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    ";
    let piezas;
    piezas = obtener_piezas_archivo(&String::from(input));
    let resultado = evaluar_jugada(&piezas[0], &piezas[1]);
    assert_eq!(resultado, "N");
}
#[test]
fn test_integracion_blancas_capturan() {
    let input = "_ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ P _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ a _ _ _ _ _
    _ _ _ _ _ _ _ _
    ";
    let piezas;
    piezas = obtener_piezas_archivo(&String::from(input));
    let resultado = evaluar_jugada(&piezas[0], &piezas[1]);
    assert_eq!(resultado, "B");
}
#[test]
fn test_integracion_empate() {
    let input = "_ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ R _ _ _ _ _
    _ _ t _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    ";
    let piezas;
    piezas = obtener_piezas_archivo(&String::from(input));
    let resultado = evaluar_jugada(&piezas[0], &piezas[1]);
    assert_eq!(resultado, "E");
}
#[test]
fn test_integracion_todos_pierden() {
    let input = "_ _ _ _ _ _ _ _
    _ _ _ _ _ P _ _
    _ _ _ _ _ _ _ _
    _ _ d _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    ";
    let piezas;
    piezas = obtener_piezas_archivo(&String::from(input));
    let resultado = evaluar_jugada(&piezas[0], &piezas[1]);
    assert_eq!(resultado, "P");
}
