-- Test acceptance criteria fixtures
INSERT INTO acceptance_criteria (id, user_story_id, description, created_at, updated_at) VALUES
('AC-001', 'US-001', 'Given I am on the login page, When I enter valid credentials, Then I should be logged in successfully', '2024-01-01 10:05:00', '2024-01-01 10:05:00'),
('AC-002', 'US-001', 'Given I am on the login page, When I enter invalid credentials, Then I should see an error message', '2024-01-01 10:06:00', '2024-01-01 10:06:00'),
('AC-003', 'US-001', 'Given I am logged in, When I click logout, Then I should be logged out and redirected to the home page', '2024-01-01 10:07:00', '2024-01-01 10:07:00'),
('AC-004', 'US-002', 'Given I am on the registration page, When I fill out all required fields with valid data, Then my account should be created', '2024-01-01 11:05:00', '2024-01-01 11:05:00'),
('AC-005', 'US-002', 'Given I am on the registration page, When I submit with missing required fields, Then I should see validation errors', '2024-01-01 11:06:00', '2024-01-01 11:06:00'),
('AC-006', 'US-003', 'Given I am on the password reset page, When I enter my email, Then I should receive a reset link', '2024-01-01 12:05:00', '2024-01-01 12:05:00'),
('AC-007', 'US-003', 'Given I have a valid reset token, When I set a new password, Then my password should be updated', '2024-01-01 12:06:00', '2024-01-01 12:06:00'),
('AC-008', 'US-004', 'Given I am on my profile page, When I update my information, Then the changes should be saved', '2024-01-01 13:05:00', '2024-01-01 13:05:00'),
('AC-009', 'US-005', 'Given I am on the search page, When I enter search terms, Then I should see relevant results', '2024-01-01 14:05:00', '2024-01-01 14:05:00'),
('AC-010', 'US-005', 'Given I perform a search, When no results are found, Then I should see a "no results" message', '2024-01-01 14:06:00', '2024-01-01 14:06:00');
