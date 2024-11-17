# rust_market

Un mercado de equipos mineros de alto rendimiento construido con Rust, enfocado en confiabilidad, seguridad y escalabilidad.

## Descripción General

`rust_market` es una plataforma especializada de mercado para equipos mineros, construida con Rust. Proporciona una infraestructura robusta para gestionar listados de equipos, pedidos y transacciones, con un fuerte énfasis en la integridad de datos y pruebas.

## Stack Tecnológico

- **Lenguaje**: Rust
- **Framework Web**: Actix-Web
- **Base de Datos**: PostgreSQL 15
- **ORM**: Diesel 2.2.4
- **Pruebas**: Framework de pruebas integrado de Rust
- **Registro**: Sistema de registro personalizado usando el crate `log`
- **Gestión de Entorno**: dotenv
- **Pool de Conexiones**: r2d2

## Características

- **Gestión de Equipos**: Sistema integral para gestionar listados de equipos mineros
- **Procesamiento de Pedidos**: Gestión robusta de pedidos con soporte de transacciones
- **Sistema de Categorías**: Categorización jerárquica de equipos
- **Gestión de Usuarios**: Autenticación y autorización segura de usuarios
- **Manejo de Imágenes**: Soporte para imágenes de equipos y documentos técnicos
- **Registros de Mantenimiento**: Seguimiento del historial de mantenimiento de equipos
- **Sistema de Reseñas**: Reseñas y calificaciones de usuarios para equipos

## Esquema de Base de Datos

La aplicación utiliza una base de datos PostgreSQL bien estructurada con las siguientes tablas principales:

- `users`: Información de cuentas de usuario
- `equipment_categories`: Clasificación de equipos
- `equipment`: Listados de equipos mineros
- `orders`: Órdenes de compra
- `order_items`: Elementos individuales en pedidos
- `equipment_images`: Fotos y diagramas de equipos
- `technical_documents`: Especificaciones y manuales de equipos
- `maintenance_records`: Historial de servicio
- `reviews`: Comentarios y calificaciones de usuarios

## Infraestructura de Pruebas

Nuestra infraestructura de pruebas está diseñada para confiabilidad y cobertura integral:

### Entorno de Pruebas

- Configuración separada de base de datos de prueba vía `DATABASE_URL_TEST`
- Entorno de pruebas aislado para cada ejecución
- Limpieza integral entre pruebas

### Categorías de Pruebas

1. **Pruebas Unitarias**
   - Validación de modelos
   - Transformación de datos
   - Lógica de negocio

2. **Pruebas de Integración**
   - Operaciones de base de datos
   - Endpoints de API
   - Manejo de transacciones

3. **Pruebas de Rendimiento**
   - Operaciones concurrentes de usuarios
   - Comportamiento del pool de conexiones
   - Aislamiento de transacciones

### Ayudantes de Prueba

Ubicados en `src/test_helpers.rs`, proporcionando:

- Limpieza de base de datos con manejo adecuado de claves foráneas
- Gestión de transacciones
- Configuración de registro
- Generación de datos de prueba

### Gestión de Transacciones de Base de Datos

Nuestras pruebas utilizan la API de transacciones de Diesel para asegurar:

- Operaciones atómicas
- Reversión adecuada en caso de fallo
- Respeto de restricciones de clave foránea
- Aislamiento de datos entre pruebas

## Instalación

El proyecto incluye un script de configuración automatizado que maneja todos los pasos de instalación:

```bash
# 1. Clonar el repositorio
git clone [repository-url]
cd rust_market

# 2. Ejecutar el script de configuración
chmod +x scripts/setup_and_test.sh
./scripts/setup_and_test.sh
```

El script de configuración:
- Instala las dependencias del sistema requeridas (PostgreSQL, Rust)
- Configura la base de datos
- Configura las variables de entorno
- Instala Diesel CLI
- Ejecuta las migraciones de la base de datos
- Compila el proyecto
- Ejecuta las pruebas

### Requisitos
- Sistema basado en Linux
- Conexión a Internet para descargar dependencias
- Privilegios de sudo para instalar paquetes del sistema

### Post-Instalación
Después de una instalación exitosa:
1. Ejecutar el servicio: `cargo run`
2. Acceder a la API en: `http://localhost:8080`
3. Ejecutar pruebas: `cargo test`

Para documentación técnica detallada, consulte [technical.md](technical.md).

## Estructura del Proyecto

```
rust_market/
├── src/
│   ├── models/           # Modelos de datos
│   ├── schema/          # Esquema de base de datos
│   ├── handlers/        # Manejadores de solicitudes
│   ├── db/             # Operaciones de base de datos
│   ├── test_helpers.rs # Utilidades de prueba
│   └── main.rs         # Entrada de la aplicación
├── migrations/         # Migraciones de base de datos
├── tests/             # Pruebas de integración
└── scripts/           # Scripts de utilidad
```

## Manejo de Errores

La aplicación implementa un manejo integral de errores:

- Tipos de error personalizados para diferentes escenarios
- Propagación adecuada de errores
- Registro detallado
- Mensajes de error amigables para el usuario

## Registro

El registro está configurado tanto para desarrollo como para pruebas:

- Diferentes niveles de registro (debug, info, error)
- Formato de registro estructurado
- Registros de prueba separados
- Monitoreo de rendimiento

## Contribuir

1. Hacer fork del repositorio
2. Crear rama de características
3. Escribir pruebas para nuevas características
4. Asegurar que todas las pruebas pasen
5. Enviar pull request

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo LICENSE para detalles.

## Agradecimientos

- Comunidad de Rust por la excelente documentación
- Equipo de Diesel por el robusto ORM
- Contribuidores y probadores
