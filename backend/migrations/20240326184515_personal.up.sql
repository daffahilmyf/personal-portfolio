-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
  id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
  email VARCHAR(255) NOT NULL,
  first_name VARCHAR(255) NOT NULL,
  last_name VARCHAR(255) NOT NULL,
  user_type VARCHAR(4) NOT NULL,
  slug VARCHAR(255) NOT NULL,
  role VARCHAR(4) NOT NULL DEFAULT '1002',
  created_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT,
  updated_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT
);

CREATE TABLE IF NOT EXISTS companies (
  id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  description TEXT,
  slug VARCHAR(255) NOT NULL,
  created_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT,
  updated_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT
);

CREATE TABLE IF NOT EXISTS achievements (
  id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  slug VARCHAR(255) NOT NULL,
  issuer VARCHAR(255) NOT NULL,
  achievement_type VARCHAR(4) NOT NULL,
  created_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT,
  updated_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT
);

CREATE TABLE IF NOT EXISTS reviews (
  id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
  reviewer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  reviewee_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  description TEXT,
  review_type VARCHAR(4),
  created_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT,
  updated_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT
);

CREATE TABLE IF NOT EXISTS jobs (
  id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
  company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  title VARCHAR(255) NOT NULL,
  description TEXT,
  job_type VARCHAR(4) NOT NULL,
  started_at BIGINT,
  ended_at BIGINT,
  created_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT,
  updated_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT
);

CREATE TABLE IF NOT EXISTS skills (
  id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  skill_type VARCHAR(4) NOT NULL,
  slug VARCHAR(255) NOT NULL,
  created_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT,
  updated_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT

);

CREATE TABLE IF NOT EXISTS user_skills (
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
  PRIMARY KEY (user_id, skill_id),
  created_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT,
  updated_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT
);

CREATE TABLE IF NOT EXISTS user_companies (
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  company_id UUID NOT NULL REFERENCES companies(id) ON DELETE CASCADE,
  PRIMARY KEY (user_id, company_id),
  created_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT,
  updated_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT
);

CREATE TABLE IF NOT EXISTS user_achievements (
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  achievement_id UUID NOT NULL REFERENCES achievements(id) ON DELETE CASCADE,
  PRIMARY KEY (user_id, achievement_id),
  created_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT,
  updated_at BIGINT DEFAULT extract(epoch from CURRENT_TIMESTAMP)::BIGINT
);



-- Inserting data into the users table
INSERT INTO users (id, email, first_name, last_name, user_type, slug, role)
VALUES ('f6af341e-9a69-4238-8c99-ce0e0baadc67', 'daffahilmanafrizal@gmail.com', 'Daffa Hilmy', 'Fadhlurrohman', '1001', 'daffa-hilmy-fadhlurrohman', '1001');

INSERT INTO users (id, email, first_name, last_name, user_type, slug)
VALUES ('d9a1dc84-0d8e-4c77-9e79-9398e687b26a', 'user2@example.com', 'Jane', 'Smith', '1002', 'jane-smith');

-- Inserting data into the companies table
INSERT INTO companies (id, name, description, slug)
VALUES ('893588d7-ccf1-4b5d-b2b1-518acb6e46f1', 'Tokhimo Inc', 'Software Development Company', 'tokhimo-inc'),
       ('733bddc9-1aa1-4a90-9c73-3f021db5628f', 'Telkom Indonesia', 'Data Analytics Company', 'telkom-indonesia');

-- Inserting data into the achievements table
INSERT INTO achievements (id, title, description, issuer, slug, achievement_type)
VALUES ('023847d4-e416-49ae-ae74-0f589607fe2d', 'Certificate of Excellence', 'Awarded for outstanding performance', 'Company A', 'certificate-of-excellence', '2001'),
       ('952c53e2-928f-4298-a203-33ba98dca121', 'Data Analytics Certification', 'Achieved for proficiency in data analysis', 'Company B', 'data-analytics-certification', '2002');

-- Inserting data into the user achievements table
INSERT INTO user_achievements (user_id, achievement_id)
VALUES ('f6af341e-9a69-4238-8c99-ce0e0baadc67', '023847d4-e416-49ae-ae74-0f589607fe2d'),
       ('f6af341e-9a69-4238-8c99-ce0e0baadc67', '952c53e2-928f-4298-a203-33ba98dca121');

-- Inserting data into the user companies table
INSERT INTO user_companies (user_id, company_id)
VALUES ('f6af341e-9a69-4238-8c99-ce0e0baadc67', '893588d7-ccf1-4b5d-b2b1-518acb6e46f1'),
       ('f6af341e-9a69-4238-8c99-ce0e0baadc67', '733bddc9-1aa1-4a90-9c73-3f021db5628f');

-- Inserting data into the jobs table
INSERT INTO jobs (company_id, user_id, title, description, job_type, started_at, ended_at)
VALUES ('893588d7-ccf1-4b5d-b2b1-518acb6e46f1', 'f6af341e-9a69-4238-8c99-ce0e0baadc67', 'Software Engineer', 'Developing software applications', '3001', extract(epoch from CURRENT_TIMESTAMP)::BIGINT, NULL),
       ('733bddc9-1aa1-4a90-9c73-3f021db5628f', 'f6af341e-9a69-4238-8c99-ce0e0baadc67', 'QA', 'Analyzing data for insights', '3002', extract(epoch from CURRENT_TIMESTAMP)::BIGINT, NULL);

INSERT INTO reviews (reviewer_id, reviewee_id, description, review_type) 
VALUES ( 'd9a1dc84-0d8e-4c77-9e79-9398e687b26a', 'f6af341e-9a69-4238-8c99-ce0e0baadc67', 'Lorem Ipsum Dolor er Amet', '2001');