#![allow(dead_code)]

pub const GET_USERS: &str = r#"
    SELECT * 
    FROM users
"#;

pub const GET_USER_BY_SLUG: &str = r#"
    SELECT * 
    FROM users 
    WHERE slug = $1
"#;

pub const CREATE_USER: &str = r#"
    INSERT INTO users (email, first_name, last_name, user_type, slug) 
    VALUES ($1, $2, $3, $4, $5) 
    RETURNING *
"#;