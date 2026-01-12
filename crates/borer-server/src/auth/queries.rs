pub const INSERT_CLIENT: &str = r#"
    INSERT INTO clients (id, name) VALUES ($1, $2)
"#;

pub const INSERT_TOKEN: &str = r#"
    INSERT INTO tokens (token, client_id) VALUES ($1, $2)
"#;

pub const GET_TOKEN: &str = r#"
    SELECT token, client_id, revoked, created_at, last_used_at
    FROM tokens
    WHERE token = $1
"#;

pub const GET_VALID_TOKEN: &str = r#"
    SELECT token, client_id, revoked, created_at, last_used_at
    FROM tokens
    WHERE token = $1 AND revoked = false
"#;

pub const UPDATE_LAST_USED: &str = r#"
    UPDATE tokens SET last_used_at = now() WHERE token = $1
"#;
