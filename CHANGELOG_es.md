## 📝 Changelog — Versión `v0.4.4` (2025-10-25)

### ✨ Nuevas funciones

* 🗑️ **Limpieza Automática de Ejecutables**
  Los ejecutables de procesos falsos ahora se eliminan automáticamente cuando los procesos terminan.

  * **Auto-eliminación** - Archivos `.exe` removidos automáticamente al finalizar proceso.
  * **Ahorro de espacio** - No se necesita limpieza manual de la carpeta `Games/`.
  * **Rastreo de rutas** - ProcessMonitor ahora rastrea las rutas de los ejecutables.
  * **Operación silenciosa** - La limpieza ocurre en segundo plano sin intervención del usuario.

* 🔄 **Sistema Mejorado de Actualización de Presets**
  Renovación completa del mecanismo de actualización de presets para resolver problemas de límites de peticiones.

  * **API de GitHub Releases** - Usa Releases en lugar de archivos raw (no más errores 429).
  * **Caché inteligente** - Verifica actualizaciones cada 6 horas automáticamente.
  * **Control de versiones** - Versionado semántico apropiado para presets.
  * **Verificación de hash** - Asegura integridad de archivos con validación SHA-256.
  * **Protección de timeout** - Timeout de 30 segundos previene congelamiento de la app.
  * **Sistema de respaldo** - Usa versión local si falla la remota.
  * **Workflow automático** - GitHub Actions auto-publica actualizaciones de presets.

---

### 🛠️ Mejoras

* **Rendimiento**: Reducción de llamadas a la API de GitHub con caché inteligente
  * TTL de caché de 6 horas balancea frescura y rendimiento
  * Opción de verificación forzada disponible para verificación manual
  * Metadata almacenada localmente en `presets_metadata.json`

* **Confiabilidad**: Múltiples protecciones contra fallos de actualización
  * Protección de timeout de red (10s para verificar versión, 30s para descarga)
  * Validación de JSON antes de aplicar actualizaciones
  * Respaldo elegante a presets locales en caso de error
  * Header User-Agent para cumplir con mejores prácticas de API de GitHub

* **Experiencia del Desarrollador**: Gestión más fácil de presets
  * Workflow automatizado de GitHub Actions para publicación
  * Documentación clara para contribuir presets
  * Paso de validación en pipeline CI/CD
  * Changelogs auto-generados para releases de presets

---

### 🌐 Traducciones

* No se requieren nuevas claves de traducción para esta versión
* Todos los mensajes usan claves existentes de versiones anteriores

---

### 🐞 Correcciones

* **Límite de Peticiones**: Corregido error "429 Too Many Requests" al actualizar presets
  * Reemplazado acceso a archivos raw con API de GitHub Releases
  * Implementado caché para reducir llamadas a API
  * Agregada lógica de reintento apropiada con backoff exponencial

* **Espacio en Disco**: Corregida acumulación de ejecutables falsos en carpeta Games/
  * Limpieza automática cuando los procesos terminan
  * ProcessMonitor rastrea y elimina archivos huérfanos
  * Implementación eficiente en memoria con Arc<Mutex>

* **Verificación de Actualizaciones**: Mejorada confiabilidad de detección de versión de presets
  * Mejor manejo de errores para fallos de red
  * Validación de caché basada en timestamp
  * Separadas verificaciones forzadas de automáticas

---

### 🔧 Cambios Técnicos

* **Cambios Core**:
  * Modificada estructura `ProcessInfo` para incluir `exe_path: PathBuf`
  * Actualizada función `create_fake_process()` para retornar tupla `(u32, PathBuf)`
  * Mejorada función `check_and_remove_dead_processes()` para eliminar ejecutables
  * Agregado método `add_process()` con parámetro de ruta

* **Sistema de Presets**:
  * Nuevo archivo: `presets_metadata.json` para rastreo de versiones
  * Agregada estructura `PresetsMetadata` con version/timestamp/hash
  * Implementada función `force_check_updates()` para verificación manual
  * Agregada función `is_cache_expired()` para validación de TTL
  * Creada función `calculate_hash()` para verificaciones de integridad
  * Actualizada función `update_presets_file()` para usar API de Releases

* **Integración con GitHub**:
  * Nuevo workflow: `.github/workflows/update-presets.yml`
  * Validación automatizada de presets con `jq`
  * Versionado auto-incremental basado en commits
  * Creación/actualización de releases con gestión apropiada de assets

* **Constantes**:
  * `CACHE_TTL_SECONDS`: 21600 (6 horas)
  * `GITHUB_API_URL`: Apunta a `/releases/tags/presets`
  * Valores de timeout: 10s (verificación), 30s (descarga)

---

### 📚 Documentación

* Agregado `PRESETS_RELEASE_GUIDE.md` - Guía para publicar actualizaciones de presets
* Agregado `PRESETS_UPDATE_SYSTEM.md` - Documentación técnica del nuevo sistema
* Actualizados workflows con generación automática de changelog
* Incluida sección de troubleshooting para problemas comunes

---

### 🔄 Notas de Migración

**Para Usuarios:**
- No se requiere acción - actualiza automáticamente en la próxima verificación
- El antiguo `presets.json` permanece compatible
- Nuevo `presets_metadata.json` creado automáticamente

**Para Contribuidores:**
- Usar nuevo workflow para envío de presets
- Seguir guías de versionado semántico
- Probar validez de JSON antes de enviar PRs

---

### ⚠️ Cambios que Rompen Compatibilidad

Ninguno - totalmente compatible con v0.4.3

---

## 📝 Changelog — Versión `v0.4.3` (2025-10-21)

### ✨ Nuevas funciones

* 🎯 **Prefijo Automático Games/ en Rutas**
  Gestión simplificada de rutas para procesos falsos.

  * **Prefijo automático** - Solo escribe `Fortnite/Win64` en lugar de `Games/Fortnite/Win64`.
  * **Vista previa visual** - Muestra la ruta completa `📁 Games/...` antes de crear el proceso.
  * **Compatible con rutas anteriores** - Sigue aceptando rutas completas con prefijo `Games/`.
  * **Presets actualizados** - Todos los presets oficiales ahora usan rutas simplificadas.

* 🔄 **Reseteo Automático de Rich Presence**
  Rich Presence ahora se actualiza automáticamente cuando terminan los procesos falsos.

  * **Monitoreo de procesos** - Rastrea todos los procesos falsos en ejecución.
  * **Auto-reseteo a "Esperando..."** - Rich Presence se limpia cuando el proceso se cierra.
  * **Sin intervención manual** - Todo ocurre automáticamente en segundo plano.
  * **Verificaciones optimizadas** - Verifica el estado del proceso cada 2 segundos.

---

### 🛠️ Mejoras

* **Gestión de Rutas**: Experiencia de usuario simplificada con prefijo automático `Games/`
  * Los usuarios solo necesitan especificar rutas relativas desde la carpeta Games
  * Ruta completa mostrada como confirmación visual antes de la ejecución
  * Definiciones de presets más limpias sin prefijo repetitivo `Games/`

* **Ciclo de Vida de Rich Presence**: Automatización completa de actualizaciones de presencia en Discord
  * Monitoreo inteligente de procesos generados
  * Limpieza automática cuando los procesos terminan
  * Mejor sincronización entre el estado de la app y el estado de Discord

---

### 🐞 Correcciones

* **Persistencia de Rich Presence**: Corregido problema donde Rich Presence no se reseteaba después de cerrar el proceso falso
  * Implementado sistema `ProcessMonitor` para rastrear procesos activos
  * Agregadas verificaciones periódicas para detectar procesos terminados
  * Rich Presence ahora regresa correctamente al estado "Esperando..."

---

### 🔧 Cambios Técnicos

* Agregada estructura `ProcessMonitor` para gestionar ciclos de vida de procesos falsos
* Implementado sistema de rastreo de PIDs usando la crate `sysinfo`
* Mejorada función `create_fake_process()` para retornar ID del proceso
* Agregado método `check_dead_processes()` con intervalo de verificación de 2 segundos
* Actualizados todos los presets para usar formato de ruta simplificado
* Modificado `process.rs` para agregar automáticamente `Games/` a rutas relativas

---

## 📝 Changelog — Versión `v0.4.2` (2025-10-11)

### ✨ Nuevas funciones

* 🎯 **Gestión de Presets Personalizados**
  Sistema completo para gestionar presets personalizados directamente desde la interfaz.

  * **Agregar presets personalizados** sin editar archivos JSON manualmente.
  * **Editar presets existentes** - modifica nombre, ejecutable o ruta.
  * **Eliminar presets personalizados** con diálogo de confirmación.
  * **Distinción visual** - presets personalizados marcados con ícono ⭐.
  * **Almacenamiento separado** - presets personalizados guardados en `presets_custom.json`.

* 🔒 **Actualizaciones Seguras de Presets**
  Las actualizaciones de presets desde GitHub ahora preservan los presets creados por el usuario.

  * **Sistema de archivos dual** - presets oficiales en `presets.json`, personalizados en `presets_custom.json`.
  * **Presets personalizados protegidos** - actualizar desde GitHub solo afecta los presets oficiales.
  * **Sin pérdida de datos** - tus presets personalizados permanecen intactos durante actualizaciones.

---

### 🛠️ Mejoras

* **Optimización de Rendimiento**: Mejoras masivas de rendimiento en la pestaña Principal
  * **Cache de detección de Discord** - verifica solo cada 5 segundos en lugar de cada frame.
  * **Animaciones reducidas** - respuesta de interfaz más rápida y suave.
  * **Eliminación de trompicones** - arrastrar ventana e interacción ahora completamente fluidos.
  * **Invalidación inteligente** - cache se actualiza inmediatamente cuando es necesario (ej: abrir Discord).

* **Mejor UX**: Flujo de trabajo mejorado para gestión de presets
  * **Diálogos intuitivos** - formularios claros para agregar/editar presets.
  * **Prevención de duplicados** - valida nombres de presets antes de guardar.
  * **Diálogos de confirmación** - previene eliminación accidental.
  * **Manejo de errores** - retroalimentación clara para entradas inválidas.

---

### 🌐 Traducciones

* Agregadas nuevas claves de traducción para gestión de presets:
  * `"add_preset"`, `"add_preset_title"`, `"edit_preset"`, `"edit_preset_title"`
  * `"delete_preset"`, `"delete_preset_title"`, `"delete_preset_confirm"`
  * `"preset_name"`, `"save_preset"`, `"delete"`, `"cancel"`
  * `"preset_fields_empty"`, `"preset_added_success"`, `"preset_edited_success"`, `"preset_deleted_success"`

---

### 🐞 Correcciones

* **Problemas de Rendimiento**: Corregidos problemas severos de rendimiento en pestaña Principal
  * Eliminadas verificaciones constantes de procesos de Discord que causaban stuttering en UI
  * Reducidas llamadas innecesarias al sistema durante renderizado
  * Corregidas interacciones lentas con botones y campos de texto

* **Gestión de Presets**: Confiabilidad mejorada
  * Mejores mensajes de error para datos de preset inválidos
  * Corregidas posibles condiciones de carrera al guardar presets
  * Mejorado manejo de archivos para presets personalizados

---

### 🔧 Cambios Técnicos

* Agregado `presets_custom.json` para presets creados por usuarios
* Implementado sistema de cache con `std::time::Instant` para verificaciones de Discord
* Mejorada estructura `Preset` con campo `is_custom`
* Nuevas funciones de gestión de presets: `add_preset()`, `edit_custom_preset()`, `delete_custom_preset()`
* Optimizado tiempo de animaciones de egui para mejor capacidad de respuesta
* Agregado sistema de invalidación de cache para actualizaciones inmediatas cuando sea necesario

---

## 📝 Changelog — Versión `v0.4.1` (2025-09-20)

### 🛠️ Mejoras

* **Arquitectura de Código**: Modularización completa de la UI para mejor mantenibilidad
  * Dividido `src/app/ui.rs` en módulos separados: `main_tab.rs`, `settings_tab.rs`, `about_tab.rs`
  * Agregado módulo `components.rs` para componentes UI reutilizables
  * **Mejor organización del código** con módulos enfocados y de responsabilidad única
  * **Mejor escalabilidad** - más fácil agregar nuevas pestañas y componentes
  * **Mantenibilidad mejorada** - archivos más pequeños y enfocados

---

### 🔧 Cambios Técnicos

* Reorganizada estructura de UI de archivo monolítico a arquitectura modular:
  ```
  src/app/ui/
  ├── main_tab.rs      - Lógica y componentes de la pestaña Principal
  ├── settings_tab.rs  - Lógica y componentes de la pestaña Configuraciones
  ├── about_tab.rs     - Lógica y componentes de la pestaña Sobre
  └── components.rs    - Componentes UI reutilizables
  ```
* **Sin cambios que rompan compatibilidad** - toda la funcionalidad permanece idéntica
* **Mejor experiencia del desarrollador** con base de código más limpia y navegable

---

## 📝 Changelog — Versión `v0.4.0` (2025-09-17)

### ✨ Nuevas funciones

* 🎮 **Integración con Discord Rich Presence**
  ¡DSQProcess ahora muestra tu actividad de juego simulado en Discord!

  * Muestra **"Jugando [Nombre del Juego]"** en tu perfil de Discord en lugar de nombres de ejecutables.
  * Indica el **tiempo transcurrido** desde que se inició DSQProcess.
  * **Opción de activación** en Configuraciones para habilitar/deshabilitar Rich Presence (habilitado por defecto).
  * **Detección inteligente de nombres** - muestra "Borderlands 4" en lugar de "Borderlands4.exe".

* 🔍 **Verificación Manual de Presets**
  Verifica actualizaciones de presets sin reiniciar la aplicación.

  * Agregado botón **"Verificar Presets"** en la pestaña Principal.
  * Verificación instantánea de archivos locales vs. remotos de presets.
  * **Confirmación de estado** cuando los presets están actualizados.

* 🎨 **Interfaz de Usuario Mejorada**
  Interfaz completamente rediseñada con mejor organización y centrado.

  * **Elementos centrados** - pestañas, botones y controles ahora correctamente alineados.
  * **Secciones agrupadas** con tarjetas visuales para mejor organización.
  * **Espaciado consistente** y diseño profesional en toda la aplicación.
  * **Iconos agregados** a botones y secciones para mejor retroalimentación visual.

---

### 🛠️ Mejoras

* **Arquitectura de Código**: Sistema de Rich Presence organizado en módulos dedicados (`src/shared/richpresence/`).
* **Rendimiento de UI**: Renderizado optimizado con mejor gestión de widgets.
* **Manejo de Errores**: Mensajes de error mejorados y retroalimentación de estado para Rich Presence.
* **Persistencia de Configuraciones**: Preferencias de Rich Presence guardadas en la configuración.

---

### 🌐 Traducciones

* Agregadas nuevas claves de traducción para Rich Presence y mejoras de UI:
  * `"enable_rich_presence"`, `"rich_presence_connected"`, `"rich_presence_disconnected"`
  * `"rich_presence_error"`, `"check_presets"`, `"presets_up_to_date"`

---

### 🐞 Correcciones

* **Problemas de Centrado**: Corregidas pestañas, combo boxes y botones desalineados.
* **Borrow Checker**: Resueltos errores de compilación de Rust relacionados con préstamos mutables/inmutables.
* **Detección de Presets**: Mejorada extracción de nombres de juegos desde presets para Rich Presence.
* **Gestión de Memoria**: Mejor limpieza de conexiones de Rich Presence al cerrar la aplicación.

---

### 🔧 Cambios Técnicos

* Actualizadas dependencias en `Cargo.toml` para incluir `discord-rich-presence = "1.0.0"`.
* Mejorada estructura `Config` para incluir campo `rich_presence_enabled`.
* Mejor gestión del estado de la aplicación con `RichPresenceManager`.
* Mejor manejo de errores para fallos de conexión Discord IPC.

---

## 📝 Changelog — Versión `v0.3.0` (2025-07-03)

### ✨ Nuevas funciones

* 🔁 **Actualización remota de presets**
  Ahora los presets se pueden mantener actualizados comparando automáticamente tu archivo local con el del repositorio oficial.

  * Se agrega un botón para **actualizar el archivo `presets.json`** directamente desde GitHub.
  * Se muestra una advertencia si tu archivo local está desactualizado.

* 🛡️ **Detección de Discord en ejecución**
  DSQProcess ahora verifica si **Discord, Discord PTB o Canary** están abiertos.

  * Si no están activos, se muestra un mensaje de advertencia.
  * Si hay versiones instaladas, se ofrecen botones para abrirlas directamente desde la app.

* 🧭 **Interfaz organizada con pestañas**
  Se reorganizó la UI principal con un sistema de pestañas:

  * **Principal:** configuración del proceso falso.
  * **Configuraciones:** selector de idioma y ajustes futuros.
  * **Sobre:** versión actual, verificación de actualizaciones y créditos.

---

### 🛠️ Mejoras

* Reorganización del código en **módulos y subcomponentes** para facilitar el mantenimiento.
* La app **recuerda la última configuración** gracias al sistema de persistencia (`config.json`).
* Reestructuración de los textos para soportar traducciones en pestañas y nuevos mensajes.

---

### 🌐 Traducciones

* Se añadieron nuevas claves al sistema de traducción multilenguaje:

  * `"discord_not_running"`, `"start_discord_prompt"`, `"discord_not_installed"`, `"tab_main"`, `"tab_settings"`, `"tab_about"`

---

### 🐞 Correcciones

* Corregido un bug donde la pestaña siempre volvía a "Principal" al intentar cambiar.
* Se mejoró la estabilidad al comparar archivos locales y remotos en la verificación de presets.

---
