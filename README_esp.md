# rust_market

Una visión general del proyecto `rust_market` al 25 de octubre de 2024.

## Introducción

`rust_market` es una aplicación web basada en Rust de **alto rendimiento** diseñada para proporcionar una plataforma de mercado **en tiempo real**. La idea principal del proyecto es **procesar información súper rápido**, permitiendo a los usuarios ver el stock, los precios actuales y la disponibilidad **al instante**. El proyecto utiliza frameworks y bibliotecas robustas para garantizar **procesamiento ultra rápido**, seguridad y escalabilidad.

## Características

- **Procesamiento de Datos en Tiempo Real**: Ofrece actualizaciones instantáneas sobre niveles de stock, precios y disponibilidad de productos.
- **Alto Rendimiento**: Optimizado para la velocidad para manejar cambios rápidos de datos y solicitudes de usuarios de manera eficiente.
- **Servidor Actix-Web**: Utiliza el framework Actix-Web para construir servicios web rápidos y confiables.
- **PostgreSQL con Diesel ORM**: Implementa PostgreSQL como base de datos, gestionada a través de Diesel ORM para interacciones de base de datos eficientes y seguras.
- **Integración Blockchain**: Prepara modelos y estructuras compatibles con estándares Web3, asegurando una interacción sin problemas con tecnologías de blockchain.
- **Integración de IA vía APIs**: Planea integrar funcionalidades de IA a través de APIs, con manejo de errores sólido y mecanismos de registro utilizando `reqwest`.

## Hitos del Proyecto

- **Inicialización del Proyecto**: Configurado con `cargo init` y con dependencias esenciales como `actix-web`, `diesel`, `dotenv`, entre otras.
- **Configuración del Esquema de Base de Datos**: Se crearon migraciones para configurar el esquema de la base de datos, incluyendo tablas para `users`, `products`, `orders` y `order_items`.
- **Definición de Modelos**: Se definieron structs de Rust en `src/models.rs` correspondientes a las tablas de la base de datos, derivando traits como `Queryable`, `Insertable`, `Serialize` y `Deserialize` para compatibilidad con Web3.
- **Pool de Conexión a la Base de Datos**: Se estableció un pool de conexiones en `src/db.rs` usando `r2d2` y `diesel`, con manejo de errores robusto y gestión de variables de entorno.
- **Configuración del Servidor**: Se configuró el servidor Actix-Web con middleware para registro de logs y se incluyó un endpoint de verificación de salud.
- **Suite de Pruebas**: Se implementaron pruebas integrales para garantizar la confiabilidad y corrección de los componentes de la aplicación.

## Visión General de la Arquitectura

```mermaid
flowchart TD
    A[Cliente] --> B[Servidor Actix-Web]
    B --> C[Manejadores]
    C --> D[Procesador en Tiempo Real]
    D --> E[Pool de Conexión a la Base de Datos]
    E --> F[Base de Datos PostgreSQL]
```

## Comenzando

### Requisitos Previos

- **Rust**: Asegúrate de tener Rust instalado en tu sistema.
- **PostgreSQL**: Configura una instancia de base de datos PostgreSQL.
- **Diesel CLI**: Instala Diesel CLI para gestionar las migraciones de la base de datos.

### Instalación

1. **Clonar el Repositorio**:

   ```bash
   git clone https://github.com/JtPerez-Acle/rust_market.git
   cd rust_market
   ```

2. **Configurar Variables de Entorno**:

   Crea un archivo `.env` y añade tu URL de base de datos:

   ```env
   DATABASE_URL=postgres://usuario:contraseña@localhost/rust_market
   ```

3. **Ejecutar Migraciones de Base de Datos**:

   ```bash
   diesel migration run
   ```

4. **Compilar y Ejecutar la Aplicación**:

   ```bash
   cargo run
   ```

## Pruebas

Ejecuta la suite de pruebas usando:

```bash
cargo test
```

## Contribuyendo

¡Las contribuciones son bienvenidas! Por favor, abre un issue o envía un pull request para cualquier mejora o corrección de errores.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT.
