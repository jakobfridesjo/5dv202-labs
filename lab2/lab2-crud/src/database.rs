/**
 * Description: To be used for communicating with postgresql
 * 
 * Author: Jakob Fridesjö
 * Date: 2022-02-11
 */

use postgres::{error::Error, Client, NoTls};

mod model;

/**
 * Create postgresql connection
 */
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

/**
 * Get all snacks from database
 */
fn db_read(client: &mut Client) -> Result<Vec<Snack>, Error> {
    let mut snacks = Vec::new();
    for row in client.query(
        "SELECT name,amount,price FROM snacks",
        &[]
    )? {
        snacks.push(Snack {
            name: row.get(0),
            amount: row.get(1),
            price: row.get(2),
        });
    }

    Ok(snacks)
}

/**
 * Update snack in database
 */
fn db_update(client: &mut Client) -> Result<Vec<Snack>, Error> {
    client.execute(
        "UPDATE snacks
         SET col = val0
         WHERE name = val1",
        &[]
    )?;

    Ok(())
}

/**
 * Insert snack into database
 */
fn db_insert(client: &mut Client, snack: Snack) -> Result<(), Error> {

    client.execute(
        "INSERT INTO snacks (name,amount,price) 
        VALUES ($1 ,$2, $3)",
        &[snack.name, snack.amount, snack.price]
    )?;

    Ok(())
}

/**
 * Remove snack from database
 */
fn db_remove(client: &mut Client, snack: Snack) -> Result<(), Error> {

    client.execute(
        "DELETE FROM snacks WHERE name= 
        VALUES $1",
        &[snack.name]
    )?;

    Ok(())
}