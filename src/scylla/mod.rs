use scylla::frame::response::result::Row;
use scylla::macros::FromRow;
use scylla::Session;
use scylla::SessionBuilder;

struct Scylla {
    pub session: Session,
    pub keyspace: String,
}

impl Scylla {
    pub async fn new(contact_point: String, keyspace: String) -> Scylla {
        let session: Session = SessionBuilder::new()
            .known_node(contact_point.as_str())
            .build()
            .await
            .unwrap();

        Scylla {
            session: session,
            keyspace: keyspace,
        }
    }
    pub async fn create_keyspace(&self) {
        let query = format!(
            "CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{ 'class' : 'SimpleStrategy', 'replication_factor' : 1 }}",
            self.keyspace
        );
        self.session.query(query.as_str(), &[]).await.unwrap();
    }

    pub async fn create_table(&self, table_name: String, schema: String) {
        let query = format!(
            "CREATE TABLE IF NOT EXISTS {}.{} ({}); ",
            self.keyspace, table_name, schema
        );
        self.session.query(query.as_str(), &[]).await.unwrap_err();
    }

    pub async fn check_if_table_exists(&self, table_name: String) -> bool {
        let query = format!(
            "SELECT * FROM system_schema.tables WHERE keyspace_name = '{}' AND table_name = '{}'",
            self.keyspace, table_name
        );
        let result = self.session.query(query.as_str(), &[]).await.unwrap().rows;
        if result.expect("Error").is_empty() {
            return false;
        }
        return true;
    }

    pub async fn insert<T: scylla::frame::value::Value>(
        &self,
        table_name: String,
        fields: String,
        schema_placeholder: String,
        values: Vec<T>,
    ) {
        let query = format!(
            "INSERT INTO {}.{} ({}) VALUES ({})",
            self.keyspace, table_name, fields, schema_placeholder
        );
        self.session
            .query(query.as_str(), &values)
            .await
            .unwrap_err();
    }

    pub async fn update<T: scylla::frame::value::Value>(
        &self,
        table_name: String,
        fields: String,
        schema_placeholder: String,
        values: Vec<T>,
    ) {
        let query = format!(
            "UPDATE {}.{} SET ({}) = ({})",
            self.keyspace, table_name, fields, schema_placeholder
        );
        self.session
            .query(query.as_str(), &values)
            .await
            .unwrap_err();
    }

    pub async fn delete<T: scylla::frame::value::Value>(
        &self,
        table_name: String,
        fields: String,
        values: Vec<T>,
    ) {
        let query = format!(
            "DELETE FROM {}.{} WHERE ({})",
            self.keyspace, table_name, fields
        );
        self.session
            .query(query.as_str(), &values)
            .await
            .unwrap_err();
    }

    pub async fn get_all(&self, table_name: String) -> Vec<Row> {
        let query = format!("SELECT * FROM {}.{}", self.keyspace, table_name);
        let result = self.session.query(query.as_str(), &[]).await.unwrap().rows;
        match result {
            Some(result) => return result,
            None => return Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use tokio::time::{sleep, Duration};

    fn setup_scylla() -> Command {
        let mut command = Command::new("docker");
        command.args(&[
            "run",
            "--name",
            "scylla_test",
            "-p",
            "9042:9042",
            "-d",
            "scylladb/scylla",
        ]);
        command
    }

    #[tokio::test]
    async fn test_create_keyspace() {
        // Start ScyllaDB in Docker
        let mut command = setup_scylla();
        command.output().expect("Failed to execute command");

        // Give the database a little bit of time to start up
        sleep(Duration::from_secs(30)).await;

        let contact_point = "127.0.0.1:9042".to_string();
        let keyspace = "test_keyspace".to_string();

        let scylla = Scylla::new(contact_point, keyspace.clone()).await;
        scylla.create_keyspace().await;

        // Check that the keyspace was created
        let query = format!(
            "SELECT * FROM system_schema.keyspaces WHERE keyspace_name = '{}'",
            keyspace
        );
        let result = scylla
            .session
            .query(query.as_str(), &[])
            .await
            .unwrap()
            .rows;
        assert!(
            result.expect("Error").is_empty() == false,
            "Keyspace was not created"
        );

        // Stop and remove the ScyllaDB instance
        let mut command = Command::new("docker");
        command.args(&["rm", "-f", "scylla_test"]);
        command.output().expect("Failed to execute command");
    }
}
