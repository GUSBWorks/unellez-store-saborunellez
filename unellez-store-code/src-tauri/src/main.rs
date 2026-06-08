#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn instalar_paquete(enlace: String) -> Result<String, String> {
    println!("Descargando e instalando desde: {}", enlace);
    let ruta_temp = "/tmp/instalador_unellez.deb";

    let descarga = std::process::Command::new("wget")
        .arg("-q")
        .arg("-O")
        .arg(ruta_temp)
        .arg(&enlace)
        .output();

    if let Err(_) = descarga {
        return Err("Error al descargar el archivo desde GitHub".to_string());
    }

    let comando_instalacion = format!("dpkg -i {} ; apt-get install -f -y", ruta_temp);
    
    let salida = std::process::Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(&comando_instalacion)
        .output();

    let _ = std::fs::remove_file(ruta_temp);

    match salida {
        Ok(res) => {
            if res.status.success() {
                Ok("¡Instalación exitosa!".to_string())
            } else {
                let error_real = String::from_utf8_lossy(&res.stderr);
                println!("⚠️ ERROR DE LINUX: {}", error_real);
                Err("Error en la instalación. Revisa la terminal.".to_string())
            }
        }
        Err(_) => Err("Fallo al contactar con el sistema".to_string()),
    }
}


// --- FUNCIÓN PARA DESINSTALAR ---
#[tauri::command]
fn desinstalar_paquete(paquete: String) -> Result<String, String> {
    println!("Vue pidió desinstalar: {}", paquete);
    
    let salida = std::process::Command::new("pkexec")
        .arg("apt-get")
        .arg("remove")
        .arg("-y")
        .arg(&paquete)
        .output();

    match salida {
        Ok(res) => if res.status.success() { Ok(format!("{} eliminado", paquete)) } else { Err("Error al desinstalar".to_string()) },
        Err(_) => Err("Fallo del sistema".to_string()),
    }
}

// --- FUNCIÓN PARA ACTUALIZAR CATÁLOGO ---
#[tauri::command]
fn actualizar_sistema() -> Result<String, String> {
    println!("Actualizando lista de repositorios...");
    
    let salida = std::process::Command::new("pkexec")
        .arg("apt-get")
        .arg("update")
        .output();

    match salida {
        Ok(res) => if res.status.success() { Ok("Catálogo actualizado".to_string()) } else { Err("Error en update".to_string()) },
        Err(_) => Err("Fallo del sistema".to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        
        .invoke_handler(tauri::generate_handler![instalar_paquete, desinstalar_paquete, actualizar_sistema])
        .run(tauri::generate_context!())
        .expect("Ocurrió un error al iniciar la tienda");
}