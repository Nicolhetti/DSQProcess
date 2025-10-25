# 🔄 Sistema de Actualización de Presets - v0.4.4

## 🎯 Problema Resuelto

**Antes (v0.4.3 y anteriores):**
- ❌ Error 429: Too Many Requests al actualizar presets
- ❌ Descarga directa de archivos raw de GitHub
- ❌ Sin control de versiones
- ❌ Sin caché, verificaba cada vez

**Ahora (v0.4.4+):**
- ✅ Usa GitHub Releases API (sin límites de rate)
- ✅ Sistema de caché inteligente (6 horas)
- ✅ Versionado de presets
- ✅ Validación de integridad con hash
- ✅ Timeout configurable
- ✅ Workflow automático para publicar presets

---

## 🏗️ Arquitectura del Sistema

```
DSQProcess App
    ↓
Check Presets (cada 6h o manual)
    ↓
GitHub Releases API
    ↓
Tag: "presets"
    ↓
Descarga presets.json
    ↓
Validación + Hash
    ↓
Actualización Local
```

---

## 📁 Archivos Nuevos

### 🐌 Cache no actualiza después de 6 horas

**Solución:**
1. Borrar `presets_metadata.json` manualmente
2. Hacer click en "Check presets" (fuerza verificación)
3. Reiniciar DSQProcess

### ⏱️ Timeout en descarga

**Solución:**
- El timeout está configurado a 30 segundos
- Verificar conexión a internet
- Intentar más tarde
- El sistema usará versión local como fallback

---

## 🔐 Seguridad

### Validaciones Implementadas:

1. **JSON Schema**: Valida estructura antes de aplicar
2. **Hash Verification**: Detecta archivos corruptos
3. **Timeout Protection**: Evita bloqueos indefinidos
4. **Fallback Local**: Siempre usa versión local si falla remota
5. **HTTPS Only**: Todas las peticiones por HTTPS

---

## 🤝 Contribuir Presets

### Formato de Preset:

```json
{
  "name": "Nombre del Juego",
  "executable": "JuegoEjecutable.exe",
  "path": "JuegoFolder/Win64"
}
```

### Checklist para PR:

- [ ] Nombre del juego correcto y completo
- [ ] Ejecutable verificado (case-sensitive)
- [ ] Ruta relativa desde `Games/`
- [ ] JSON válido
- [ ] Orden alfabético mantenido
- [ ] No duplicados

### Cómo Proponer Nuevo Preset:

1. Fork del repositorio
2. Edita `presets.json`
3. Agrega tu preset en orden alfabético
4. Crea PR con título: `feat: Add [Nombre del Juego] preset`
5. En la descripción incluye:
   - Screenshot del juego detectado en Discord

---

## 📚 Referencias

- [GitHub Releases API](https://docs.github.com/en/rest/releases)
- [Semantic Versioning](https://semver.org/)
- [JSON Schema](https://json-schema.org/)

---

## 📞 Soporte

¿Problemas con el sistema de actualización?

- 🐛 **Bugs**: [GitHub Issues](https://github.com/Nicolhetti/DSQProcess/issues)
- 💬 **Preguntas**: [GitHub Discussions](https://github.com/Nicolhetti/DSQProcess/discussions)
- 📧 **Contacto**: [@Nicolhetti](https://github.com/Nicolhetti)

---

**DSQProcess v0.4.4** - Sistema de Actualización Mejorado 🚀`presets_metadata.json` (Auto-generado)

```json
{
  "version": "1.0.0",
  "last_check": 1735123456,
  "hash": "a1b2c3d4e5f6"
}
```

**Campos:**
- `version`: Versión actual de presets instalada
- `last_check`: Timestamp de última verificación (Unix epoch)
- `hash`: Hash del contenido para detectar cambios

---

## 🔧 Funciones Principales

### `is_presets_outdated()` - Con Cache
```rust
// Verifica si hay actualizaciones disponibles
// Solo hace petición remota si el cache expiró (6 horas)
let outdated = is_presets_outdated();
```

### `force_check_updates()` - Sin Cache
```rust
// Fuerza verificación remota ignorando cache
// Usado cuando el usuario hace click en "Check presets"
let outdated = force_check_updates();
```

### `update_presets_file()` - Descarga
```rust
// Descarga y actualiza presets.json desde GitHub Release
update_presets_file()?;
```

---

## ⏱️ Sistema de Caché

### TTL (Time To Live): 6 horas

**¿Por qué 6 horas?**
- ⚖️ Balance entre frescura y rendimiento
- 🌐 Reduce carga en GitHub API
- 🔋 Ahorra ancho de banda del usuario
- ⚡ Inicio más rápido de la aplicación

### Flujo de Verificación:

```
Usuario inicia DSQProcess
    ↓
¿Cache expirado? (> 6h)
    ├─ NO → Usar versión cacheada
    └─ SÍ → Verificar GitHub API
            ↓
        ¿Hay actualización?
            ├─ NO → Actualizar timestamp
            └─ SÍ → Mostrar alerta
```

---

## 🚀 Para Desarrolladores

### Publicar Nuevos Presets

#### Opción 1: Automático (Recomendado)

1. Edita `presets.json` en el repo
2. Commit y push a `master`
3. GitHub Actions automáticamente:
   - Valida el JSON
   - Crea/actualiza release `presets`
   - Publica el archivo

#### Opción 2: Manual

```bash
# 1. Crear release con tag "presets"
gh release create presets \
  --title "Presets Update - v1.1.0" \
  --notes "Nuevos juegos agregados" \
  presets.json

# 2. O actualizar existente
gh release upload presets presets.json --clobber
```

#### Opción 3: GitHub UI (Más Fácil)

1. Ve a **Releases → Draft new release**
2. Tag: `presets` (⚠️ **IMPORTANTE**)
3. Title: `Presets Update - v1.X.0`
4. Sube `presets.json` como asset
5. Publish release

---

## 🧪 Testing

### Probar Actualización Local

```rust
// En tu código de test
use crate::core::presets::{update_presets_file, force_check_updates};

#[test]
fn test_presets_update() {
    // Verificar si hay actualización
    let outdated = force_check_updates();
    assert!(!outdated || update_presets_file().is_ok());
}
```

### Simular Cache Expirado

```bash
# Edita presets_metadata.json y cambia last_check
{
  "version": "1.0.0",
  "last_check": 0,  # <- Forzar expiración
  "hash": "abc123"
}
```

---

## 📊 Monitoreo

### Logs de Actualización

La app registra:
- ✅ Verificaciones exitosas
- ⚠️ Errores de red (timeout, 404)
- 🔄 Actualizaciones aplicadas
- ⏱️ Uso de caché

### Métricas de GitHub

Ver uso de API en:
- **Settings → Releases → presets**
- Download count de `presets.json`

---

## 🆘 Troubleshooting

### ❌ "429 Too Many Requests"

**Solución**: Este error ya NO debería ocurrir con el nuevo sistema. Si aparece:
1. Verifica que estés usando v0.4.4+
2. Confirma que `GITHUB_API_URL` apunta a `/releases/tags/presets`
3. Revisa que el caché funcione correctamente

### ❌ "404 Release not found"

**Causas posibles:**
- No existe release con tag `presets`
- Release está en draft

**Solución:**
```bash
# Crear release inicial
gh release create presets \
  --title "Presets v1.0.0" \
  --notes "Initial presets release" \
  presets.json
```

### ⚠️ "presets.json not found in release"

**Solución:**
1. Abre el release en GitHub
2. Verifica que `presets.json` esté en Assets
3. Re-sube si falta: `gh release upload presets presets.json --clobber`
