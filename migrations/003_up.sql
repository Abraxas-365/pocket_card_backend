
-- Migration: Add template string to user_settings

-- Step 1: Add the new column
ALTER TABLE user_settings
ADD COLUMN template_string TEXT;

-- Step 2: Update existing rows with a default value (optional)
UPDATE user_settings
SET template_string = 'default';

-- Step 3: Add any constraints if needed (optional)
ALTER TABLE user_settings
ALTER COLUMN template_string SET NOT NULL;

-- Step 4: Update the create_user_settings function to include the new column
CREATE OR REPLACE FUNCTION create_user_settings()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO user_settings (user_id, template_string)
    VALUES (NEW.id, 'default');
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Note: The trigger doesn't need to be recreated as it's calling the function

-- Step 5: Update timestamp
UPDATE user_settings
SET updated_at = CURRENT_TIMESTAMP;
