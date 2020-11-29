CREATE TABLE sharks (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE FUNCTION set_update_time() RETURNS OPAQUE AS '
    BEGIN
    new.updated_at := ''now'';
    return new;
    END;
' LANGUAGE 'plpgsql';

CREATE TRIGGER update_tri BEFORE UPDATE ON sharks FOR EACH ROW
EXECUTE PROCEDURE set_update_time();
