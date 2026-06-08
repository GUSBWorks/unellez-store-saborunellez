<script setup>
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

// --- CONTROL DE NAVEGACIÓN ---
const moduloActivo = ref('tienda')

// --- BASE DE DATOS UNELLEZ (GitHub Pages) ---
const appsUnellez = ref([])
const destacadaUnellez = ref(null)
const errorConexion = ref(false)

// --- FUNCIÓN PARA CONECTAR CON GITHUB ---
const cargarCatalogo = async () => {
  errorConexion.value = false;
  try {
    const respuesta = await fetch('https://raw.githubusercontent.com/GUSBWorks/unellez-store-saborunellez/main/catalogo.json?v=' + new Date().getTime())
    const datos = await respuesta.json()
    
    appsUnellez.value = datos.map(app => {
      return { ...app, estado: 'instalar', progreso: 0 }
    })

    if (appsUnellez.value.length > 0) {
      destacadaUnellez.value = appsUnellez.value[0]
    }
  } catch (error) {
    console.warn("Sin internet. Bloqueando la tienda para ahorrar recursos.");
    errorConexion.value = true;
  }
}

onMounted(() => {
  cargarCatalogo()
})

// --- FUNCIÓN DE INSTALACIÓN ---
const instalarApp = (app) => {
  app.estado = 'instalando'; 

  setTimeout(async () => {
    try {
      await invoke('instalar_paquete', { enlace: app.url_deb });
      app.estado = 'instalado'; 
    } catch (error) {
      console.error(error);
      app.estado = 'instalar';
      alert("No se pudo instalar. Verifica tu conexión o tu contraseña.");
    }
  }, 100);
}

// --- FUNCIÓN PARA DESINSTALAR ---
const desinstalarApp = async (app) => {
  if(!confirm(`¿Seguro que deseas desinstalar ${app.nombre}?`)) return;
  
  app.estado = 'desinstalando'; 
  try {
    await invoke('desinstalar_paquete', { paquete: app.paquete });
    app.estado = 'instalar'; 
  } catch (error) {
    console.error(error);
    app.estado = 'instalado';
    alert("Ocurrió un error al intentar desinstalar.");
  }
}

// --- FUNCIÓN PARA ACTUALIZAR REPOSITORIOS ---
const actualizarSistema = async () => {
  alert("Iniciando actualización. Esto ejecutará comandos de Canaima y pedirá tu contraseña.");
  try {
    await invoke('actualizar_sistema');
    alert("¡Repositorios y sistema actualizados correctamente!");
  } catch (error) {
    console.error(error);
    alert("Error al actualizar el sistema.");
  }
}

// --- HISTORIAL DE INSTALADAS ---
const appsInstaladas = computed(() => {
  return appsUnellez.value.filter(app => app.estado === 'instalado');
});
</script>

<template>
  <main class="app-contenedor">
    
    <aside class="sidebar-iconos">
      <div class="sidebar-logo">💻</div>
      <div class="sidebar-menu">
        <button :class="{ activo: moduloActivo === 'tienda' }" @click="moduloActivo = 'tienda'" title="Tienda">⌂</button>
        <button :class="{ activo: moduloActivo === 'descargas' }" @click="moduloActivo = 'descargas'" title="Mis Descargas">📥</button>
        <button :class="{ activo: moduloActivo === 'configuracion' }" @click="moduloActivo = 'configuracion'" title="Configuración">⚙</button>
      </div>
    </aside>

    <div class="area-principal">
      <section class="contenido-scroll">
        <Transition name="fade" mode="out-in">
          
          <div v-if="moduloActivo === 'tienda'" class="modulo-vista">
           <div v-if="errorConexion" style="text-align: center; margin-top: 100px;">
            <h2 style="color: #ff4c4c;">📡 ¡Sin conexión a Internet! 📡</h2>
            <p style="color: #aaa;">La UNELLEZ Store necesita acceso a la red para descargar los repositorios y catálogos.</p>
            <button @click="cargarCatalogo()" class="btn-accion" style="margin-top: 20px; background-color: #ff4c4c;">
              🔄 Reintentar conexión
            </button>
          </div>

          <div v-else-if="!destacadaUnellez" style="text-align: center; margin-top: 100px; color: #aaa;">
            <h2>⏳ Conectando con el repositorio UNELLEZ... ⏳</h2>
            <p>Obteniendo la lista de software disponible</p>
          </div>
            
            <div class="banner-dinamico" v-if="destacadaUnellez" :style="{ background: `linear-gradient(135deg, ${destacadaUnellez.color1}, ${destacadaUnellez.color2})` }">
              <div class="banner-texto">
                <h2>¡Descubre las últimas novedades!</h2>
                <p>Presentando: <strong>{{ destacadaUnellez.nombre }}</strong></p>
              </div>
              <div class="banner-icono">
                 <img :src="destacadaUnellez.icono" alt="Icono" style="width: 80px; height: 80px;" />
              </div>
            </div>

            <div class="grid-tarjetas">
              <div v-for="app in appsUnellez" :key="app.id" class="tarjeta" @click="destacadaUnellez = app" :style="{ '--color-acento': app.color1 }">
                
                <div class="tarjeta-icono" :style="{ color: app.color1 }">
                  <img :src="app.icono" alt="Icono" style="width: 40px; height: 40px;" />
                </div>
                
                <h3>{{ app.nombre }}</h3>
                <p>{{ app.desc }}</p>
               
                <div class="area-acciones">
                  
                  <button 
                    v-if="app.estado === 'instalar'"
                    class="btn-tarjeta" 
                    :style="{ backgroundColor: app.color1 }"
                    @click="instalarApp(app)"
                  >
                    INSTALAR
                  </button>

                  <div v-if="app.estado === 'instalando' || app.estado === 'desinstalando'" style="width: 100%;">
                    <p style="text-align: center; font-size: 12px; margin-bottom: 5px; color: #aaa;">
                      {{ app.estado === 'instalando' ? 'Instalando paquete...' : 'Desinstalando...' }}
                    </p>
                    <div class="contenedor-barra">
                      <div class="barra-luz"></div>
                    </div>
                  </div>

                  <div v-if="app.estado === 'instalado'" class="botones-fila" style="display: flex; gap: 10px; width: 100%;">
                    <button class="btn-tarjeta" style="background-color: #28a745; flex: 1; cursor: default;">
                      INSTALADO ✓
                    </button>
                    <button class="btn-desinstalar" @click="desinstalarApp(app)" title="Desinstalar programa" style="padding: 10px;">
                      ✖
                    </button>
                  </div>

                </div>
              </div>
            </div>
          </div>

          <div v-else-if="moduloActivo === 'descargas'" class="modulo-vista">
            <h2 class="titulo-seccion">Software Instalado 📦</h2>

            <div v-if="appsInstaladas.length === 0">
              <p style="color: #aaa; text-align: center; margin-top: 50px; font-size: 1.2rem;">
                Aún no tienes aplicaciones instaladas en tu equipo. <br>
                ¡Ve al Repositorio UNELLEZ para descubrir nuevas herramientas!
              </p>
            </div>

            <div v-else class="grid-tarjetas">
              <div v-for="app in appsInstaladas" :key="app.id" class="tarjeta" :style="{ '--color-acento': app.color1 }">
                
                <div class="tarjeta-icono" :style="{ color: app.color1 }">
                  <img :src="app.icono" alt="Icono" style="width: 40px; height: 40px;" />
                </div>
                
                <h3>{{ app.nombre }}</h3>
                <p>{{ app.desc }}</p>
                
                <div class="area-acciones">
                  <button 
                    class="btn-desinstalar"
                    @click="desinstalarApp(app)"
                    style="width: 100%; padding: 12px; margin-top: 10px;"
                  >
                    Desinstalar programa ✖
                  </button>
                </div>
              </div>
            </div>
          </div>

          <div v-else-if="moduloActivo === 'configuracion'" class="modulo-vista">
            <h2 class="titulo-seccion">Configuración del Sistema ⚙</h2>
            <div class="tarjeta-config">
              <div>
                <h3>Sincronización de Catálogo</h3>
                <p>Fuerza la actualización de la lista de software y sistema. (apt update)</p>
              </div>
              <button class="btn-accion" @click="actualizarSistema()">Actualizar Ahora</button>
            </div>
            <div class="tarjeta-config" style="margin-top: 20px;">
              <div>
                <h3>Acerca de la UNELLEZ Store</h3>
                <p>Desarrollado por Gustavo Barreto - GUSB Works, para la edición "Sabor UNELLEZ" del S/O Canaima GNU/Linux 8.3 Kavanayén.</p>
                <small style="color: #009cf5;">Licencia: GNU GPLv3</small>
              </div>
              <div style="font-size: 2rem;">🎓</div>
            </div>
          </div>

        </Transition>
      </section>
    </div>
  </main>
</template>

<style>
/* --- VARIABLES BASE --- */
:root {
  --fondo-app: #111111;
  --fondo-sidebar: #151515;
  --fondo-tarjeta: #1e1e1e;
  --texto-blanco: #ffffff;
  --texto-gris: #aaaaaa;
  --acento-azul: #009cf5;
}

* { margin: 0; padding: 0; box-sizing: border-box; }
body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background: var(--fondo-app); color: var(--texto-blanco); overflow: hidden; }

/* --- ESTRUCTURA PRINCIPAL --- */
.app-contenedor { display: flex; height: 100vh; }

/* --- BARRA LATERAL (IZQUIERDA) --- */
.sidebar-iconos {
  width: 70px;
  background-color: var(--fondo-sidebar);
  display: flex; flex-direction: column; align-items: center; padding: 20px 0;
  border-right: 1px solid #2a2a2a;
}
.sidebar-logo { font-size: 1.8rem; margin-bottom: 40px; color: var(--acento-azul); }
.sidebar-menu { display: flex; flex-direction: column; gap: 20px; width: 100%; align-items: center; }
.sidebar-menu button {
  background: transparent; border: none; font-size: 1.5rem; color: #555;
  cursor: pointer; padding: 10px; border-radius: 12px; transition: 0.3s;
}
.sidebar-menu button:hover { color: var(--texto-blanco); background: #2a2a2a; }
.sidebar-menu button.activo { color: var(--acento-azul); background: rgba(0, 156, 245, 0.1); }

/* --- ÁREA PRINCIPAL --- */
.area-principal { flex: 1; display: flex; flex-direction: column; }

/* --- CONTENIDO SCROLL --- */
.contenido-scroll { flex: 1; padding: 30px 50px; overflow-y: auto; }
.modulo-vista { max-width: 1200px; margin: 0 auto; }
.titulo-seccion { font-size: 2rem; margin-bottom: 20px; border-bottom: 1px solid #333; padding-bottom: 10px; }

/* --- BANNER DINÁMICO --- */
.banner-dinamico {
  display: flex; justify-content: space-between; align-items: center;
  padding: 40px 50px; border-radius: 20px; margin-bottom: 40px;
  transition: background 0.5s ease; box-shadow: 0 10px 30px rgba(0,0,0,0.3);
}
.banner-texto h2 { font-size: 2.5rem; margin-bottom: 10px; text-shadow: 0 2px 4px rgba(0,0,0,0.3); }
.banner-texto p { font-size: 1.2rem; opacity: 0.9; }
.banner-icono { font-size: 6rem; filter: drop-shadow(0 5px 15px rgba(0,0,0,0.4)); }

/* --- GRID Y TARJETAS --- */
.grid-tarjetas { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 25px; }
.tarjeta {
  background: var(--fondo-tarjeta); padding: 30px 20px; border-radius: 15px;
  text-align: center; cursor: pointer; display: flex; flex-direction: column;
  border: 1px solid #333; transition: all 0.3s ease;
}
.tarjeta:hover { transform: translateY(-5px); border-color: var(--color-acento); box-shadow: 0 10px 20px rgba(0,0,0,0.4); }
.tarjeta-icono { font-size: 3rem; margin-bottom: 15px; }
.tarjeta h3 { font-size: 1.2rem; margin-bottom: 10px; }
.tarjeta p { font-size: 0.9rem; color: var(--texto-gris); flex: 1; margin-bottom: 20px; }
.btn-tarjeta {
  border: none; color: white; font-weight: bold; padding: 12px; border-radius: 8px;
  cursor: pointer; width: 100%; transition: opacity 0.3s;
}
.btn-tarjeta:hover { opacity: 0.8; }

/* --- TARJETAS CONFIGURACIÓN --- */
.tarjeta-config {
  background: var(--fondo-tarjeta); padding: 25px; border-radius: 12px;
  display: flex; justify-content: space-between; align-items: center; border: 1px solid #333;
}
.tarjeta-config h3 { margin-bottom: 5px; }
.tarjeta-config p { color: var(--texto-gris); font-size: 0.9rem; margin-bottom: 5px; }
.btn-accion { background: #333; color: white; border: 1px solid #555; padding: 10px 20px; border-radius: 8px; cursor: pointer; transition: 0.3s; }
.btn-accion:hover { background: var(--acento-azul); border-color: var(--acento-azul); }

/* --- ANIMACIONES --- */
.fade-enter-active, .fade-leave-active { transition: opacity 0.3s, transform 0.3s; }
.fade-enter-from, .fade-leave-to { opacity: 0; transform: translateY(10px); }

/* --- ACCIONES Y PROGRESO --- */
.area-acciones {
  margin-top: auto; 
  display: flex;
  flex-direction: column;
  gap: 10px;
}
.botones-fila {
  display: flex;
  gap: 10px;
}
.btn-desinstalar {
  background: #dc3545;
  border: none;
  border-radius: 8px;
  color: white;
  padding: 0 15px;
  cursor: pointer;
  font-size: 1.2rem;
  transition: opacity 0.3s;
}
.btn-desinstalar:hover {
  opacity: 0.8;
}
.barra-fondo {
  width: 100%;
  height: 8px;
  background: #333;
  border-radius: 4px;
  overflow: hidden;
}
.barra-relleno {
  height: 100%;
  transition: width 0.3s ease;
}

/* --- BARRA DE CARGA INFINITA --- */
.contenedor-barra {
  width: 100%;
  height: 6px;
  background-color: #333; 
  border-radius: 4px;
  overflow: hidden;
  position: relative;
  margin-top: 10px;
}

.barra-luz {
  position: absolute;
  width: 40%;
  height: 100%;
  background-color: #009cf5; 
  border-radius: 4px;
  animation: cargando-loop 1.2s infinite ease-in-out;
}

@keyframes cargando-loop {
  0% { left: -40%; }
  100% { left: 100%; }
}
</style>