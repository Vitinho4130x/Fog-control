// Simple Fog Control (porte minimo para Horizon): distancia da nevoa do terreno.
// Config: config/simple_fog_control.txt, linhas "start=0.6" e "end=0.9"
// (fracoes da distancia de renderizacao). Sem arquivo = nevoa vanilla.
fn fog(data: &mut horizon_client::fog::FogData) {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Mutex;

    static CALLS: AtomicU32 = AtomicU32::new(0);
    static CONFIG: Mutex<(Option<f32>, Option<f32>)> = Mutex::new((None, None));

    fn load() -> (Option<f32>, Option<f32>) {
        let (mut start, mut end) = (None, None);
        if let Ok(text) = std::fs::read_to_string("config/simple_fog_control.txt") {
            for line in text.lines() {
                if let Some((key, value)) = line.split_once('=') {
                    if let Ok(n) = value.trim().parse::<f32>() {
                        match key.trim() {
                            "start" => start = Some(n),
                            "end" => end = Some(n),
                            _ => {}
                        }
                    }
                }
            }
        }
        (start, end)
    }

    // Rele o arquivo a cada ~120 quadros, sem reiniciar o jogo.
    let n = CALLS.fetch_add(1, Ordering::Relaxed);
    let (start, end) = {
        let mut cfg = CONFIG.lock().unwrap_or_else(|p| p.into_inner());
        if n % 120 == 0 {
            *cfg = load();
        }
        *cfg
    };
    if start.is_none() && end.is_none() {
        return;
    }

    let reference = data.render_distance_end;
    if reference <= 0.0 {
        return;
    }
    let vanilla_ratio = data.render_distance_start / reference;
    let new_end = reference * end.unwrap_or(1.0).clamp(0.05, 4.0);
    let new_start = match start {
        Some(s) => reference * s.clamp(0.0, 4.0),
        None => new_end * vanilla_ratio,
    };
    // Valores invalidos (fim <= inicio, NaN) fazem o jogo manter a nevoa original.
    data.render_distance_start = new_start;
    data.render_distance_end = new_end;
}
