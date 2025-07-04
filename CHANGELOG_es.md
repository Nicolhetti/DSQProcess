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
