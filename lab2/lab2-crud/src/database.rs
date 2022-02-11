// Postgres
use postgres::{error::Error, Client, NoTls};

fn db_create() -> Result<Client, Error> {
    let username = "postgres";
    let password = "postgres";
    let host = "postgres";
    let port = "5432";
    let database = "rustdb";

    let conn_str = &format!(
        "postgres://{}{}{}@{}{}{}{}{}",
        username,
        if password.is_empty() { "" } else { ":" },
        password,
        host,
        if port.is_empty() { "" } else { ":" },
        port,
        if database.is_empty() { "" } else { "/" },
        database
    );

    let mut client = Client::connect(conn_str, NoTls)?;

    // Drop all tables
    let _ = client.execute(
        "DROP TABLE IF EXISTS Roles;
               DROP TABLE IF EXISTS Actor;
               DROP TABLE IF EXISTS Media;", 
               &[]
    );

    // Create all tables
    client.execute(
        "-- Create table for media
        CREATE TABLE Media (
            media_id INT GENERATED ALWAYS AS IDENTITY,
            media_name VARCHAR(255) NOT NULL,
            genre VARCHAR(255) NOT NULL,
            year DATE NOT NULL,
            score FLOAT,
            PRIMARY KEY(media_id)
        );
        
        -- Create table for participants in media
        CREATE TABLE Actor (
            actor_id INT GENERATED ALWAYS AS IDENTITY,
            first_name VARCHAR(255) NOT NULL,
            last_name VARCHAR(255) NOT NULL,
            date_birth DATE NOT NULL,
            PRIMARY KEY(actor_id)
        );
        
        -- Create table for participants role in media
        CREATE TABLE Roles (
            roles_id INT GENERATED ALWAYS AS IDENTITY,
            actor_id INT,
            media_id INT,
            roles VARCHAR(255) NOT NULL,
            PRIMARY KEY(roles_id),
            CONSTRAINT fk_media
                FOREIGN KEY(media_id)
                    REFERENCES Media(media_id),
            CONSTRAINT fk_actor
                FOREIGN KEY(actor_id)
                    REFERENCES Actor(actor_id)
        );",
     &[]
    )?;

    Ok(client)
}