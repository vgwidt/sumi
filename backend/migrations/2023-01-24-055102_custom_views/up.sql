ALTER TABLE user_preferences ADD COLUMN custom_views jsonb NOT NULL DEFAULT '[]'::jsonb;

-- for each user in user_preferences, create an empty custom view
UPDATE user_preferences SET custom_views = '[]'::jsonb;