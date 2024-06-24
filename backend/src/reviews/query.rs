pub const GET_USER_REVIEW_BY_SLUG: &str = r#"
    SELECT
        r.id,
        r.reviewer_id,
        r.reviewee_id,
        reviewer.first_name AS reviewer_first_name,
        reviewer.last_name AS reviewer_last_name,
        reviewee.first_name AS reviewee_first_name,
        reviewee.last_name AS reviewee_last_name,
        r.description,
        r.review_type, 
        r.created_at,
        r.updated_at
    FROM
        reviews r
    JOIN
        users reviewer ON r.reviewer_id = reviewer.id
    JOIN
        users reviewee ON r.reviewee_id = reviewee.id
    WHERE
        reviewee.slug = $1
"#;