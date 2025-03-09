INSERT INTO habits (id, title, description, created, color, streak, category, status)
VALUES
('H001', 'Morning Exercise', 'Start your day with a 30-minute workout', '2022-01-01', '#007bff', 7, 'Fitness', 'ongoing'),
('H002', 'Daily Reading', 'Read for at least 20 minutes before bed', '2022-02-01', '#3498db', 5, 'Learning', 'ongoing'),
('H003', 'Gratitude Journaling', 'Write down three things you are grateful for each day', '2022-03-01', '#f1c40f', 10, 'Self-Care', 'ongoing');

INSERT INTO commits (id, message, image, created, status, completions, habit_id)
VALUES
('C001', 'Started my morning exercise routine with a 30-minute jog', 'https://example.com/exercise.jpg', '2022-01-02', 'completed', 100, 'H001'),
('C002', 'Read for 20 minutes before bed and felt more relaxed', 'https://example.com/reading.jpg', '2022-02-03', 'completed', 50, 'H002'),
('C003', 'Wrote down three things I am grateful for today: a good cup of coffee, a beautiful sunset, and a supportive friend', 'https://example.com/gratitude.jpg', '2022-03-04', 'completed', 200, 'H003');
