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

## Ejemplos

### Ejecución de un SP sin parámetros

Accediendo a la siguiente URL:

##TODO

- SP, con parámetros, pasar un JSON para ejecutar los parámetros
- Autenticación, middleware, algún sistema de autenticación de usuarios y contraseñas activos
- GitHub Action
  - Para los test
  - Para publicar los exe

## Contribución

Las contribuciones son bienvenidas. Si tienes sugerencias, abre un issue o un pull request.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT. Consulta el archivo [LICENSE](LICENSE) para obtener más información.