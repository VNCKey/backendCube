//! Gestión mínima de tareas.

/// Una tarea pendiente o terminada.
pub struct Tarea {
    pub id: u32,
    pub titulo: String,
    pub hecha: bool,
}

/// Crea una tarea nueva, sin terminar.
pub fn crear_tarea(id: u32, titulo: &str) -> Tarea {
    Tarea {
        id,
        titulo: titulo.to_string(),
        hecha: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn una_tarea_nueva_no_esta_hecha() {
        assert!(!crear_tarea(1, "leer").hecha);
    }
}
