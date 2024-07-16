-- User settings table
CREATE TABLE user_settings (
  id SERIAL PRIMARY KEY,
  user_id INTEGER UNIQUE NOT NULL REFERENCES users(id),
  exchange_contacts BOOLEAN DEFAULT true,
  save_contact BOOLEAN DEFAULT true,
  call_me BOOLEAN DEFAULT true,
  email_me BOOLEAN DEFAULT true,
  social_media BOOLEAN DEFAULT false,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Trigger for user_settings table to update timestamp
CREATE TRIGGER update_user_settings_timestamp
BEFORE UPDATE ON user_settings
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

-- Function to create user settings
CREATE OR REPLACE FUNCTION create_user_settings()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO user_settings (user_id)
    VALUES (NEW.id);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to automatically create user settings
CREATE TRIGGER create_user_settings_trigger
AFTER INSERT ON users
FOR EACH ROW
EXECUTE FUNCTION create_user_settings();
