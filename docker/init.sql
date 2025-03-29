CREATE TABLE urls (
    id INT AUTO_INCREMENT PRIMARY KEY,
    original_url VARCHAR(255) NOT NULL,
    short_code VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

INSERT INTO urls (original_url, short_code) VALUES
('https://example.com', 'exmpl'),
('https://rust-lang.org', 'rust');