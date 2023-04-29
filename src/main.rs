use tp_individual::evaluar_jugada;
use tp_individual::leer_archivo;
use tp_individual::obtener_piezas_archivo;
use tp_individual::validar_archivo;

fn imprimir_error(err: &'static str) {
    println!("Error: {err}");
    std::process::exit(1);
}

fn main() {
    let contenido_archivo_result = leer_archivo();
    
    if let Err(e) = contenido_archivo_result {
        imprimir_error(e);
    }
    let contenido_archivo = match contenido_archivo_result {
        Ok(contenido_archivo) => contenido_archivo,
        Err(e) => {imprimir_error(e);
                        return;}
    };
    
    validar_archivo(&contenido_archivo).unwrap_or_else(|err| {
        imprimir_error(err);
    });

    let piezas = obtener_piezas_archivo(&contenido_archivo);

    let resultado = evaluar_jugada(&piezas[0], &piezas[1]);
    println!("{resultado}");
}
