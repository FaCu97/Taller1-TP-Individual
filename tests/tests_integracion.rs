use std::fs;

use tp_individual::{evaluar_jugada, obtener_piezas_archivo};

#[test]
fn test_integracion_negras_capturan() -> Result<(), &'static str> {
    let file_path = "tests/data/table_test_1.txt".to_string();

    let content_result = fs::read_to_string(file_path);
    let content = match content_result {
        Ok(file) => file,
        Err(_e) => return Err("Error al leer el archivo"),
    };

    let piezas = obtener_piezas_archivo(&content)?;
    let resultado = evaluar_jugada(&piezas[0], &piezas[1])?;

    assert_eq!(resultado, "N");
    Ok(())
}

#[test]
fn test_integracion_blancas_capturan() -> Result<(), &'static str> {
    let file_path = "tests/data/table_test_2.txt".to_string();

    let content_result = fs::read_to_string(file_path);
    let content = match content_result {
        Ok(file) => file,
        Err(_e) => return Err("Error al leer el archivo"),
    };

    let piezas = obtener_piezas_archivo(&content)?;
    let resultado = evaluar_jugada(&piezas[0], &piezas[1])?;

    assert_eq!(resultado, "B");
    Ok(())
}
#[test]
fn test_integracion_empate() -> Result<(), &'static str> {
    let file_path = "tests/data/table_test_3.txt".to_string();

    let content_result = fs::read_to_string(file_path);
    let content = match content_result {
        Ok(file) => file,
        Err(_e) => return Err("Error al leer el archivo"),
    };

    let piezas = obtener_piezas_archivo(&content)?;

    let resultado = evaluar_jugada(&piezas[0], &piezas[1])?;

    assert_eq!(resultado, "E");
    Ok(())
}

#[test]
fn test_integracion_todos_pierden() -> Result<(), &'static str> {
    let file_path = "tests/data/table_test_4.txt".to_string();

    let content_result = fs::read_to_string(file_path);
    let content = match content_result {
        Ok(file) => file,
        Err(_e) => return Err("Error al leer el archivo"),
    };
    let piezas = obtener_piezas_archivo(&content)?;
    let resultado = evaluar_jugada(&piezas[0], &piezas[1])?;

    assert_eq!(resultado, "P");
    Ok(())
}
