use super::queries;
use crate::auth::models::Token;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthService {
    pool: PgPool,
    master_token: String,
}

impl AuthService {
    pub fn new(pool: PgPool, master_token: String) -> Self {
        Self { pool, master_token }
    }

    pub fn validate_master(&self, token: &str) -> bool {
        token == self.master_token
    }

    pub async fn create_token(&self, client_name: Option<String>) -> anyhow::Result<Token> {
        let client_id = Uuid::new_v4();
        let token = generate_token();

        sqlx::query(queries::INSERT_CLIENT)
            .bind(client_id)
            .bind(&client_name)
            .execute(&self.pool)
            .await?;

        sqlx::query(queries::INSERT_TOKEN)
            .bind(&token)
            .bind(client_id)
            .execute(&self.pool)
            .await?;

        let row = sqlx::query_as::<_, Token>(queries::GET_TOKEN)
            .bind(&token)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    pub async fn validate_token(&self, token: &str) -> anyhow::Result<Option<Token>> {
        let result = sqlx::query_as::<_, Token>(queries::GET_VALID_TOKEN)
            .bind(token)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(ref t) = result {
            sqlx::query(queries::UPDATE_LAST_USED)
                .bind(&t.token)
                .execute(&self.pool)
                .await?;
        }

        Ok(result)
    }
}

fn generate_token() -> String {
    let bytes: [u8; 32] = rand::random();
    format!("borer_{}", hex::encode(bytes))
}
