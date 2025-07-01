# Database Migrator CLI

- Check the status of all migrations
  ```sh
  cargo run -- status
  ```
- Apply all pending migrations
  ```sh
  cargo run
  # OR
  cargo run -- up
  ```
- Drop all tables from the database, then reapply all migrations
  ```sh
  cargo run -- fresh
  ```
- Rollback all applied migrations, then reapply all migrations
  ```sh
  cargo run -- refresh
  ```
- Rollback all applied migrations
  ```sh
  cargo run -- reset
  ```

> For more `cargo run -- help`
