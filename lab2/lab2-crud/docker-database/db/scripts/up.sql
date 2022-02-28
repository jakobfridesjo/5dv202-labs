-----------------------------------------------------------
--                                                       --
-- Description: Creates and links all tables             --
--                                                       --
-- Author: Jakob Fridesjo                                --
-- Date: 2022-02-10                                      --
-- Version: 1.0.0                                        --
--                                                       --
-----------------------------------------------------------

DROP TABLE IF EXISTS Roles;
DROP TABLE IF EXISTS Actor;
DROP TABLE IF EXISTS Media;

-- Create table for media
CREATE TABLE Media (
    media_id INT GENERATED ALWAYS AS IDENTITY,
    media_name VARCHAR(255) NOT NULL,
    media_genre VARCHAR(255) NOT NULL,
    media_year INT NOT NULL,
    media_score INT NOT NULL,
    PRIMARY KEY(media_id)
);

-- Create table for participants in media
CREATE TABLE Actor (
    actor_id INT GENERATED ALWAYS AS IDENTITY,
    actor_first_name VARCHAR(255) NOT NULL,
    actor_last_name VARCHAR(255) NOT NULL,
    actor_year INT NOT NULL,
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
);