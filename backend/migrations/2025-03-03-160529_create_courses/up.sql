CREATE TABLE courses (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    code VARCHAR NOT NULL,
    credits INTEGER NOT NULL,
    department VARCHAR NOT NULL,
    description TEXT
);

INSERT INTO courses (title, code, credits, department, description)
VALUES 
    ('Introduction to Computer Science', 'CS101', 3, 'Computer Science', 'Fundamental concepts of programming'),
    ('Data Structures', 'CS201', 4, 'Computer Science', 'Advanced data structures and algorithms'),
    ('Calculus I', 'MATH101', 3, 'Mathematics', 'Introduction to differential calculus'),
    ('Organic Chemistry', 'CHEM301', 4, 'Chemistry', 'Study of carbon compounds'),
    ('Digital Logic Design', 'EE201', 3, 'Electrical Engineering', 'Boolean algebra and logic circuits');