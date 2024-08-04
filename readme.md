# Rust SQL Server API

Este proyecto es una API escrita en Rust que ejecuta procedimientos almacenados (SP) en SQL Server. La API utiliza el nombre de la URL para determinar qué SP ejecutar.

## Características

- **Ejecución de SP**: Ejecuta procedimientos almacenados en SQL Server según el nombre de la URL.
- **Resultado del SP**: Devuelve el resultado del SP ejecutado en formato JSON.

## Próximos pasos

- **Soporte para parámetros**: Actualmente, solo se pueden ejecutar SPs que no requieren parámetros. Se planea agregar la funcionalidad para pasar parámetros a los SPs.

## Requisitos

- [Rust](https://www.rust-lang.org/)
- [SQL Server](https://www.microsoft.com/en-us/sql-server)
- Librerías de Rust para SQL Server (por ejemplo, `tiberius`)

## Instalación

1. Clona el repositorio:
    ```sh
    git clone https://github.com/karmoybt/sp_executor.git
    cd rust-sql-server-api
    ```

2. Instala las dependencias:
    ```sh
    cargo build
    ```

## Uso

1. Inicia el servidor:
    ```sh
    cargo run
    ```

2. Ejecuta un procedimiento almacenado accediendo a la URL correspondiente:
    ```
    /SP/nombre_del_sp
    ```

   Donde `nombre_del_sp` es el nombre del procedimiento almacenado que deseas ejecutar.

## Arbol de carpetas

    rust-sql-server-api/
    ├── src/
    │ ├── main.rs # Punto de entrada principal
    │ ├── config/
    │ │ ├── mod.rs # Módulo de configuración
    │ │ └── database.rs # Inicialización y configuración de la base de datos
    │ ├── routes/
    │ │ ├── mod.rs # Módulo de rutas
    │ │ ├── database.rs # Rutas relacionadas con la base de datos
    │ │ └── stored_procedure.rs # Rutas relacionadas con procedimientos almacenados
    │ └── models/
    │ ├── mod.rs # Módulo de modelos
    │ └── response.rs # Estructuras de respuesta

### Descripción de Archivos y Directorios

- **src/**
  - Contiene el código fuente principal del proyecto.

- **main.rs**
  - Archivo de entrada principal de la aplicación.

- **config/**
  - Directorio para la configuración de la aplicación.
  
  - **mod.rs**
    - Módulo principal de configuración.

  - **database.rs**
    - Inicialización y configuración de la base de datos.

- **routes/**
  - Directorio para las rutas de la API.

  - **mod.rs**
    - Módulo principal de rutas.

  - **database.rs**
    - Rutas relacionadas con la base de datos.

  - **stored_procedure.rs**
    - Rutas relacionadas con la ejecución de procedimientos almacenados.

- **models/**
  - Directorio para los modelos de datos utilizados en la aplicación.

  - **mod.rs**
    - Módulo principal de modelos.

  - **response.rs**
    - Estructuras de respuesta utilizadas en la API.


## TODO

- ~~SP, con parámetros, pasar un JSON para ejecutar los parámetros
- SP, Añadir "devolver "multilineas" "
- Autenticación, middleware, algún sistema de autenticación de usuarios y contraseñas activos
- GitHub Action
  - Para los test
  - Para publicar los exe
- "Sistema" para mapear el sp con el front
- Gestionar la bbdd , crear tablas sin/con relaciones, sp (basicos)
- Añadir mas BBDD (ClickHouse, Postgress) 

## Contribución

Las contribuciones son bienvenidas. Si tienes sugerencias, abre un issue o un pull request.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT. Consulta el archivo [LICENSE](LICENSE) para obtener más información.
