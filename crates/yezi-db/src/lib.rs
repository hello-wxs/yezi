// Copyright (C) 2026 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!();

pub fn migrate(
    connection: &mut diesel::SqliteConnection,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use diesel_migrations::MigrationHarness;

    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn init_database(
    db_path: &std::path::Path,
) -> Result<diesel::SqliteConnection, Box<dyn std::error::Error + Send + Sync + 'static>> {
    use diesel::Connection;

    let db_url = format!("sqlite://{}", db_path.display());

    let mut connection = diesel::SqliteConnection::establish(&db_url)?;
    migrate(&mut connection)?;

    Ok(connection)
}
