use tp_individual::evaluar_jugada;
use tp_individual::leer_archivo;
use tp_individual::obtener_piezas_archivo;
use tp_individual::validar_archivo;

fn main() {
    let contenido_archivo = leer_archivo();
    validar_archivo(&contenido_archivo);

    let piezas = obtener_piezas_archivo(&contenido_archivo);

    let resultado = evaluar_jugada(&piezas[0], &piezas[1]);
    println!("{resultado}");
}
