# rust_market

Un análisis detallado del proyecto `rust_market` al 25 de octubre de 2024.

## Introducción

`rust_market` es una aplicación web **de alto rendimiento**, **segura** y **escalable** construida con Rust. Sirve como una plataforma de mercado en tiempo real que procesa información a una velocidad vertiginosa, permitiendo a los usuarios ver instantáneamente los niveles de stock, precios actuales y disponibilidad de productos. La aplicación aprovecha el modelo de concurrencia de Rust, sus garantías de seguridad y frameworks web modernos para ofrecer una experiencia de usuario ultra-responsiva.

## ¿Por Qué Rust?

Elegimos Rust por varias razones convincentes:

- **Rendimiento**: El rendimiento de Rust es comparable al de C y C++, ofreciendo control de bajo nivel sobre los recursos del sistema sin sacrificar la velocidad. Esto lo hace ideal para construir aplicaciones web de alta velocidad.

- **Seguridad**: El modelo de propiedad de Rust garantiza seguridad en memoria sin necesidad de un recolector de basura. Elimina errores comunes como la desreferenciación de punteros nulos y las condiciones de carrera, lo cual es crucial para un mercado seguro.

- **Concurrencia**: Rust proporciona concurrencia sin miedo, permitiéndonos escribir código concurrente libre de condiciones de carrera. Esto es esencial para manejar eficientemente múltiples flujos de datos en tiempo real.

- **Ecosistema**: Rust tiene un ecosistema en crecimiento con potentes bibliotecas y frameworks como Actix-Web y Diesel, que utilizamos ampliamente en este proyecto.

## Características

- **Procesamiento de Datos en Tiempo Real**: Actualizaciones instantáneas sobre niveles de stock, precios y disponibilidad de productos mediante un manejo y procesamiento eficiente de datos.

- **Alto Rendimiento y Escalabilidad**: Optimizado para velocidad y escalabilidad, manejando cambios rápidos de datos y un alto volumen de solicitudes de usuarios.

- **Framework Actix-Web**: Utiliza el framework Actix-Web para construir servicios web rápidos, confiables y asíncronos.

- **Comunicación WebSocket**: Implementa comunicación en tiempo real utilizando WebSockets, permitiendo actualizaciones en vivo sin necesidad de sondeo del cliente.

- **Integración de Redis**: Utiliza Redis para almacenamiento en caché de datos accedidos frecuentemente y gestión de flujos de datos en tiempo real a través de mecanismos de pub/sub.

- **PostgreSQL con Diesel ORM**: Gestiona datos con PostgreSQL, usando Diesel ORM para interacciones de base de datos eficientes y seguras.

- **Preparado para Integración Blockchain (Web3)**: Estructura modelos y manejo de datos para ser compatibles con estándares Web3, preparándose para una integración sin problemas con tecnologías blockchain.

- **Integración de IA vía APIs**: Implementa funcionalidades de IA a través de APIs con un manejo de errores robusto, aprovechando `reqwest` para solicitudes HTTP.

- **Sistema de Registro Personalizado**: Integra un sistema de registro personalizado utilizando `flexi_logger` para registros estructurados y basados en niveles, ayudando en depuración y monitoreo.

- **Amplia Suite de Pruebas**: Incluye pruebas unitarias, de integración y de rendimiento para asegurar la confiabilidad, corrección y rendimiento bajo carga.

## Hitos del Proyecto

1. **Inicialización del Proyecto**

   - Inicializado el proyecto Rust usando `cargo init`.
   - Configuradas dependencias esenciales en `Cargo.toml`, incluyendo `actix-web`, `diesel`, `dotenv`, `flexi_logger` y otros.

2. **Configuración del Esquema de Base de Datos**

   - Creadas migraciones de base de datos para establecer el esquema, incluyendo tablas para `users`, `products`, `orders` y `order_items`.
   - Asegurada la integridad de datos con restricciones de clave foránea e indexado de campos críticos para optimización de rendimiento.

3. **Definición de Modelos**

   - Definidas structs de Rust en `src/models.rs` correspondientes a las tablas de la base de datos.
   - Derivados traits como `Queryable`, `Insertable`, `Serialize` y `Deserialize` para operaciones eficientes de base de datos y compatibilidad con Web3.

4. **Pool de Conexión a la Base de Datos**

   - Establecido un pool de conexiones en `src/db.rs` usando `r2d2` y `diesel`.
   - Implementado manejo de errores robusto usando tipos `Result` y `Option`, evitando `unwrap()` en favor de `expect()` con mensajes significativos.

5. **Implementación del Sistema de Registro**

   - Implementado un sistema de registro personalizado en `src/logging.rs` utilizando `flexi_logger`.
   - Inicializado el logger en `src/main.rs`, configurando niveles y formatos de registro para una cobertura completa.

6. **Configuración del Servidor**

   - Configurado el servidor Actix-Web con middleware para registro y manejo de errores.
   - Añadidas rutas y endpoints esenciales, incluyendo un endpoint de verificación de salud para monitoreo.

7. **Configuración del Servidor WebSocket**

   - Implementado un servidor WebSocket utilizando `tokio-tungstenite` para gestionar actualizaciones de stock en tiempo real.
   - Asegurada la concurrencia eficiente usando `tokio` para operaciones asíncronas.

8. **Implementación de Caché Redis**

   - Integrado Redis para funcionalidades de caché y pub/sub.
   - Implementados mecanismos para actualizar la caché eficientemente e invalidar datos obsoletos.

9. **Mejoras en el Manejo de Errores**

   - Mejorado el manejo de errores en toda la aplicación utilizando el crate `thiserror`.
   - Proporcionadas respuestas de error detalladas a los clientes manteniendo las mejores prácticas de seguridad.

10. **Mejoras de Seguridad**

    - Aplicada validación y sanitización de entradas para prevenir vulnerabilidades de seguridad comunes.
    - Uso regular de `cargo clippy` y `cargo audit` para linting y revisiones de seguridad.
    - Implementado limitación de tasa y mecanismos seguros de autenticación.

11. **Expansión de la Suite de Pruebas**

    - Ampliadas las pruebas para incluir pruebas unitarias de módulos individuales, pruebas de integración para endpoints API y pruebas de rendimiento bajo carga.
    - Utilizadas utilidades de prueba de `actix-web` y una instancia de base de datos de prueba para simular escenarios del mundo real.

## Visión General de la Arquitectura

```mermaid
flowchart TD
    subgraph Lado del Cliente
        A[Navegador Web]
    end
    subgraph Lado del Servidor
        B[Servidor Actix-Web & WebSocket]
        C[Manejadores]
        D[Procesador en Tiempo Real]
        E[Pool de Conexión a la Base de Datos]
        F[(Base de Datos PostgreSQL)]
        G[(Caché Redis)]
    end
    A <-->|HTTP/WebSocket| B
    B --> C
    C --> D
    D --> E
    E --> F
    D --> G
```

## Cómo Funciona

### Actualizaciones de Stock en Tiempo Real

- **Flujo de Datos**:

  1. Las actualizaciones de inventario son enviadas al servidor vía llamadas API.
  2. El servidor actualiza la base de datos PostgreSQL y la caché Redis.
  3. Redis publica la actualización a través de su mecanismo de pub/sub.
  4. El servidor WebSocket recibe el mensaje y lo transmite a los clientes conectados.
  5. Los clientes reciben la actualización y refrescan la información de stock en tiempo real.

- **Manejo de Concurrencia**:

  - Utiliza `tokio` para tareas asíncronas, asegurando que múltiples solicitudes sean manejadas eficientemente sin bloquear el hilo principal.

### Integración Web3

- **Modelos Preparados para Blockchain**:

  - Las structs están diseñadas para ser compatibles con estándares Web3, derivando traits `Serialize` y `Deserialize` para una serialización JSON fluida.

- **Interacciones con Smart Contracts**:

  - La aplicación está preparada para futura integración con smart contracts, asegurando que las estructuras de datos se alineen con los requisitos blockchain.

- **Seguridad y Validación**:

  - Las entradas son rigurosamente validadas para prevenir que datos maliciosos afecten transacciones en la blockchain.

### Integración de IA vía APIs

- **Llamadas a APIs Externas**:

  - Utiliza `reqwest` para hacer solicitudes HTTP a servicios de IA.
  - Implementa manejo de errores sólido con reintentos y mensajes de error significativos.

- **Análisis de JSON**:

  - Analiza las respuestas de IA cuidadosamente para prevenir errores en tiempo de ejecución y asegurar la integridad de los datos.
  - Valida y sanea el contenido generado por IA antes de usarlo en la aplicación.

- **Limitación de Tasa y Registro**:

  - Gestiona la frecuencia de solicitudes para adherirse a los límites de tasa de las APIs.
  - Registra las interacciones con APIs para monitoreo y depuración.

## Comenzando

### Requisitos Previos

- **Rust**: Asegúrate de tener Rust instalado en tu sistema.
- **PostgreSQL**: Configura una instancia de base de datos PostgreSQL.
- **Redis**: Instala Redis para caché y procesamiento de datos en tiempo real.
- **Diesel CLI**: Instala Diesel CLI para gestionar las migraciones de la base de datos.

### Instalación

1. **Clonar el Repositorio**:

   ```bash
   git clone https://github.com/JtPerez-Acle/rust_market.git
   cd rust_market
   ```

2. **Configurar Variables de Entorno**:

   Crea un archivo `.env` y añade tus URLs de base de datos y Redis:

   ```env
   DATABASE_URL=postgres://usuario:contraseña@localhost/rust_market
   REDIS_URL=redis://localhost:6379
   ```

3. **Ejecutar Migraciones de Base de Datos**:

   ```bash
   diesel migration run
   ```

4. **Compilar y Ejecutar la Aplicación**:

   ```bash
   cargo run
   ```

### Pruebas

Ejecuta la suite de pruebas usando:

```bash
cargo test
```

Para ejecutar pruebas de rendimiento:

```bash
cargo test --test performance_tests -- --nocapture
```

## Documentación

La documentación detallada está disponible en el directorio `documentation`, incluyendo:

- **Documentación Principal**: `documentation/core.md` proporciona una visión general de la arquitectura y componentes del proyecto.
- **Guía de Pruebas**: `tests.md` explica cómo configurar y ejecutar las pruebas.
- **Progreso del Desarrollo**: `current.md` y `documentation/25102024.md` contienen información sobre el estado actual del desarrollo y los próximos pasos.

## Contribuyendo

¡Las contribuciones son bienvenidas! Por favor, consulta `CONTRIBUTING.md` para obtener pautas sobre cómo contribuir a este proyecto.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT.
