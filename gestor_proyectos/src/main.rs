use clap::{Parser, Subcommand};
use mongodb::{Client, Collection, Database};
use mongodb::bson::doc; // Necesario para ping

// Constantes para nombres de base de datos y colecciones
const DB_NAME: &str = "gestor_proyectos_db";
const PROJECTS_COLLECTION: &str = "proyectos";
const TASKS_COLLECTION: &str = "tareas";

/// Define la estructura de los argumentos de la línea de comandos
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

/// Define los subcomandos disponibles
#[derive(Subcommand, Debug)]
enum Commands {
    /// Crea un nuevo proyecto
    CrearProyecto {
        #[clap(short, long)]
        nombre: String,
        #[clap(short, long)]
        descripcion: Option<String>,
    },
    /// Agrega una nueva tarea a un proyecto
    AgregarTarea {
        #[clap(long)] // Usaremos el ID del proyecto (como string)
        proyecto_id: String,
        #[clap(short, long)]
        descripcion: String,
        // Podríamos añadir un estado inicial opcional aquí más tarde
    },
    /// Muestra las tareas de un proyecto específico
    VerTareas {
        #[clap(long)]
        proyecto_id: String,
    },
    /// Actualiza el estado de una tarea
    ActualizarEstadoTarea {
        #[clap(long)] // Usaremos el ID de la tarea (como string)
        tarea_id: String,
        #[clap(short, long)] // Ej: "pendiente", "en progreso", "completado"
        estado: String,
    },
}

// El punto de entrada principal de nuestra aplicación tiene que ser asíncrono
// debido a que el driver de MongoDB es asíncrono.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parsear los argumentos de la línea de comandos
    let cli = Cli::parse();

    // --- Conexión a MongoDB ---
    // Cambia esta URI si tu MongoDB no está en localhost:27017
    let mongo_uri = "mongodb://localhost:27017";
    let client = Client::with_uri_str(mongo_uri).await?;

    // Ping para confirmar la conexión (opcional pero recomendado)
    client
        .database("admin") // "admin" es una DB común para este comando
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("¡Conexión a MongoDB exitosa!");

    // Obtener un handle para la base de datos
    let db = client.database(DB_NAME);

    // (Opcional) Obtener handles para las colecciones si los necesitas aquí directamente
    // let _proyectos_collection: Collection<Document> = db.collection(PROJECTS_COLLECTION);
    // let _tareas_collection: Collection<Document> = db.collection(TASKS_COLLECTION);


    // --- Lógica de Comandos ---
    // Aquí manejaremos los diferentes subcomandos
    match &cli.command {
        Commands::CrearProyecto { nombre, descripcion } => {
            println!("Comando 'CrearProyecto':");
            println!("  Nombre: {}", nombre);
            if let Some(desc) = descripcion {
                println!("  Descripción: {}", desc);
            }
            // Aquí llamaremos a la función para crear un proyecto en la BD
            // Por ejemplo: gestionar_proyectos::crear_proyecto(&db, nombre, descripcion.as_deref()).await?;
        }
        Commands::AgregarTarea { proyecto_id, descripcion } => {
            println!("Comando 'AgregarTarea':");
            println!("  ID Proyecto: {}", proyecto_id);
            println!("  Descripción Tarea: {}", descripcion);
            // Lógica para agregar tarea
        }
        Commands::VerTareas { proyecto_id } => {
            println!("Comando 'VerTareas':");
            println!("  ID Proyecto: {}", proyecto_id);
            // Lógica para ver tareas
        }
        Commands::ActualizarEstadoTarea { tarea_id, estado } => {
            println!("Comando 'ActualizarEstadoTarea':");
            println!("  ID Tarea: {}", tarea_id);
            println!("  Nuevo Estado: {}", estado);
            // Lógica para actualizar estado de tarea
        }
    }

    Ok(())
}