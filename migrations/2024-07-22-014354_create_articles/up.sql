--- Create the articles table
CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  thumbnail_image VARCHAR(255),
  static_file VARCHAR(255),
  published BOOLEAN NOT NULL DEFAULT FALSE,
  createdAt TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create the categories table
CREATE TABLE categories (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL
);

-- Create a table to associate articles with categories
CREATE TABLE article_categories (
  article_id INT REFERENCES articles(id),
  category_id INT REFERENCES categories(id),
  PRIMARY KEY (article_id, category_id)
);
