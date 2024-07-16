
-- Migration: Rename template_string to template in user_settings

-- Step 1: Rename the column
ALTER TABLE user_settings
RENAME COLUMN template_string TO template;

-- Step 2: Update the create_user_settings function to use the new column name
CREATE OR REPLACE FUNCTION create_user_settings()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO user_settings (user_id, template)
    VALUES (NEW.id, 'default');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Step 3: Update timestamp for all rows
UPDATE user_settings
SET updated_at = CURRENT_TIMESTAMP;

