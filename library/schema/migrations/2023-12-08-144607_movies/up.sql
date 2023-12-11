CREATE TABLE movies (
    id INT PRIMARY KEY AUTO_INCREMENT,
    title VARCHAR(255) NOT NULL,
    year INT NOT NULL,
    genre VARCHAR(255) NOT NULL,
    rating VARCHAR(255) NOT NULL,
    cast TEXT NOT NULL,
    image VARCHAR(255) NOT NULL
);