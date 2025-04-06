CREATE TABLE urls (
    id INT AUTO_INCREMENT PRIMARY KEY,
    original_url VARCHAR(255) NOT NULL,
    short_code VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    access_count SMALLINT NOT NULL DEFAULT 0
);

INSERT INTO urls (original_url, short_code) VALUES
('https://example.com', 'exmpl'),
('https://rust-lang.org', 'rust');

-- Opcional: crear usuario específico para la aplicación
CREATE USER IF NOT EXISTS 'app_user'@'%' IDENTIFIED BY 'app_password';
GRANT ALL PRIVILEGES ON url_shortening_service.* TO 'app_user'@'%';
FLUSH PRIVILEGES;