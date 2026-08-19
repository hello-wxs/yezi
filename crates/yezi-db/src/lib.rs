// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod schema;

const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!();

pub fn migrate(connection: &mut diesel::SqliteConnection) -> Result<(), Error> {
    use diesel_migrations::MigrationHarness;

    connection
        .run_pending_migrations(MIGRATIONS)
        .map_err(|_| Error::MigrationError)?;

    Ok(())
}

pub fn init_database(db_path: &std::path::Path) -> Result<diesel::SqliteConnection, Error> {
    use diesel::Connection;

    let db_url = format!("sqlite://{}", db_path.display());

    let mut connection = diesel::SqliteConnection::establish(&db_url)?;
    migrate(&mut connection)?;

    Ok(connection)
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to connect to database: {0}")]
    ConnectError(#[from] diesel::ConnectionError),
    #[error("failed to migrate database")]
    MigrationError,
}
