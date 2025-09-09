-- Create user_stories table
CREATE TABLE user_stories (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    persona TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create acceptance_criteria table
CREATE TABLE acceptance_criteria (
    id TEXT PRIMARY KEY NOT NULL,
    user_story_id TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_story_id) REFERENCES user_stories(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX idx_acceptance_criteria_user_story_id ON acceptance_criteria(user_story_id);
CREATE INDEX idx_user_stories_created_at ON user_stories(created_at);
CREATE INDEX idx_acceptance_criteria_created_at ON acceptance_criteria(created_at);
