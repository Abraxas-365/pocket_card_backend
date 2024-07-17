-- Users table
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  google_id VARCHAR(255) UNIQUE NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- User profiles table (updated to include former user_settings fields)
CREATE TABLE user_profiles (
  id VARCHAR(36) PRIMARY KEY,  -- UUID as string (36 characters including hyphens)
  user_id INTEGER NOT NULL REFERENCES users(id),
  email VARCHAR(255),
  name VARCHAR(255),
  bio TEXT,
  phone_number VARCHAR(20),
  location VARCHAR(255),
  profile_picture_url TEXT,
  theme VARCHAR(50),
  template TEXT NOT NULL DEFAULT 'default',
  custom_url VARCHAR(255) UNIQUE,
  job_title VARCHAR(255),
  facebook_url TEXT,
  twitter_url TEXT,
  instagram_url TEXT,
  linkedin_url TEXT,
  exchange_contacts BOOLEAN DEFAULT true,
  save_contact BOOLEAN DEFAULT true,
  call_me BOOLEAN DEFAULT true,
  email_me BOOLEAN DEFAULT true,
  social_media BOOLEAN DEFAULT false,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Function to update timestamp
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = CURRENT_TIMESTAMP;
   RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for users table
CREATE TRIGGER update_users_timestamp
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

-- Trigger for user_profiles table
CREATE TRIGGER update_user_profiles_timestamp
BEFORE UPDATE ON user_profiles
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

-- Function to create user profile
CREATE OR REPLACE FUNCTION create_user_profile()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO user_profiles (id, user_id, email, template)
    VALUES (gen_random_uuid()::text, NEW.id, NEW.email, 'default');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to automatically create user profile
CREATE TRIGGER create_user_profile_trigger
AFTER INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION create_user_profile();
