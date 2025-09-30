# Quiz Management API

A robust REST API built with Rust, Axum, and PostgreSQL for managing quiz questions and topics. Perfect for building quiz applications, learning platforms, or exam preparation systems.

## Features

- **Topic Management**: Organize questions by topics with human-readable slugs
- **Question CRUD**: Full create, read, update, delete operations for questions
- **Bulk Import**: Import multiple questions at once with transaction support
- **Flexible Search**: Search questions by content, explanation, or topic
- **Type Safety**: Built with Rust for compile-time guarantees and zero-cost abstractions
- **JSONB Storage**: Efficient storage and querying of question options and answers
- **Pagination**: Built-in pagination for large question sets
- **CORS Enabled**: Ready for frontend integration

## Tech Stack

- **Rust** - Systems programming language with memory safety
- **Axum** - Modern web framework built on Tokio
- **SQLx** - Async SQL toolkit with compile-time query verification
- **PostgreSQL** - Robust relational database with JSONB support
- **Tokio** - Asynchronous runtime

## Getting Started

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- PostgreSQL 14+ ([Install PostgreSQL](https://www.postgresql.org/download/))
- SQLx CLI: `cargo install sqlx-cli`

### Installation

1. **Clone the repository**
```bash
git clone https://github.com/yourusername/quiz-api.git
cd quiz-api
```

2. **Set up environment variables**
```bash
cp .env.example .env
# Edit .env with your database credentials
```

`.env` file:
```env
DATABASE_URL=postgresql://username:password@localhost/quiz_db
```

3. **Create the database**
```bash
sqlx database create
```

4. **Run migrations**
```bash
sqlx migrate run
```

5. **Build and run**
```bash
cargo run --release
```

The API will be available at `http://localhost:3000`

## API Documentation

### Base URL
```
http://localhost:3000
```

### Health Check
```http
GET /health
```

### Topics

#### Get all topics
```http
GET /topics
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "name": "AWS Storage",
      "slug": "aws-storage",
      "description": "Questions about AWS storage services",
      "created_at": "2025-09-29T10:00:00Z",
      "updated_at": "2025-09-29T10:00:00Z"
    }
  ],
  "message": null
}
```

#### Create topic
```http
POST /topics
Content-Type: application/json

{
  "name": "AWS Storage",
  "slug": "aws-storage",  // Optional - auto-generated if not provided
  "description": "Questions about AWS storage services"
}
```

#### Get topic by ID
```http
GET /topics/{id}
```

#### Get topic by slug
```http
GET /topics/slug/{slug}
```

#### Update topic
```http
PUT /topics/{id}
Content-Type: application/json

{
  "name": "AWS Storage Services",  // Optional
  "description": "Updated description"  // Optional
}
```

#### Delete topic
```http
DELETE /topics/{id}
```

### Questions

#### Get questions (paginated)
```http
GET /questions?page=1&limit=20
```

**Response:**
```json
{
  "success": true,
  "data": [
    {
      "id": "uuid",
      "topic_id": "uuid",
      "question_number": 1,
      "question": "What is Amazon S3?",
      "options": {
        "A": "A compute service",
        "B": "A storage service",
        "C": "A database service",
        "D": "A networking service"
      },
      "correct_answer": ["B"],
      "explanation": "Amazon S3 is an object storage service.",
      "question_type": "single",
      "difficulty": "easy",
      "tags": ["s3", "storage"],
      "created_at": "2025-09-29T10:00:00Z",
      "updated_at": "2025-09-29T10:00:00Z"
    }
  ],
  "message": null
}
```

#### Create question
```http
POST /questions
Content-Type: application/json

{
  "topic_id": "uuid",
  "question_number": 1,
  "question": "What is Amazon S3?",
  "options": [
    "A compute service",
    "A storage service",
    "A database service",
    "A networking service"
  ],
  "correct_answer": ["B"],
  "explanation": "Amazon S3 is an object storage service.",
  "question_type": "single",
  "difficulty": "easy",  // Optional: easy, medium, hard
  "tags": ["s3", "storage"]  // Optional
}
```

#### Bulk create questions
```http
POST /questions/bulk
Content-Type: application/json

{
  "topic_slug": "aws-storage",
  "questions": [
    {
      "question_number": 1,
      "question": "What is Amazon S3?",
      "options": [
        "A compute service",
        "A storage service",
        "A database service",
        "A networking service"
      ],
      "correct_answer": ["B"],
      "explanation": "Amazon S3 is an object storage service.",
      "question_type": "single",
      "difficulty": "easy",
      "tags": ["s3", "storage"]
    },
    {
      "question_number": 2,
      "question": "Which services provide serverless compute? (Select TWO)",
      "options": [
        "AWS Lambda",
        "Amazon EC2",
        "AWS Fargate",
        "Amazon ECS"
      ],
      "correct_answer": ["A", "C"],
      "explanation": "Lambda and Fargate are serverless.",
      "question_type": "multiple",
      "difficulty": "medium",
      "tags": ["serverless", "compute"]
    }
  ]
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "created": 2,
    "failed": 0,
    "errors": []
  },
  "message": null
}
```

#### Get question by ID
```http
GET /questions/{id}
```

#### Update question
```http
PUT /questions/{id}
Content-Type: application/json

{
  "question": "Updated question text",  // All fields optional
  "difficulty": "hard"
}
```

#### Delete question
```http
DELETE /questions/{id}
```

#### Get questions by topic
```http
GET /questions/topic/{topic_id}
```

#### Get questions by type
```http
GET /questions/type/{question_type}
```
Types: `single` or `multiple`

#### Search questions
```http
GET /questions/search/{query}
```
Searches in question text, explanation, and topic name.

## Data Models

### Question Types
- `single` - Single correct answer
- `multiple` - Multiple correct answers

### Difficulty Levels
- `easy`
- `medium`
- `hard`

### Options Format
- **Input**: Array of strings `["Option A", "Option B", "Option C", "Option D"]`
- **Output**: Object with letter keys `{"A": "Option A", "B": "Option B", ...}`
- **Storage**: Efficient JSONB array in PostgreSQL

### Answer Format
- Store as letter labels: `["A"]` or `["A", "C"]`
- Corresponds to array indices: A=0, B=1, C=2, D=3, etc.

## Database Schema

### Topics Table
```sql
CREATE TABLE topics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    slug TEXT UNIQUE NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Questions Table
```sql
CREATE TABLE questions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    topic_id UUID NOT NULL REFERENCES topics(id) ON DELETE CASCADE,
    question_number INTEGER NOT NULL,
    question TEXT NOT NULL,
    options JSONB NOT NULL,
    correct_answer JSONB NOT NULL,
    explanation TEXT NOT NULL,
    question_type question_type NOT NULL,
    difficulty difficulty_level NOT NULL,
    tags TEXT[],
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

## Project Structure

```
quiz-api/
├── src/
│   ├── main.rs           # Application entry point, routes
│   ├── handlers.rs       # Request handlers
│   ├── models.rs         # Data models and types
│   └── database.rs       # Database connection
├── migrations/           # SQL migration files
├── Cargo.toml           # Rust dependencies
└── README.md
```

## Development

### Running in development mode
```bash
cargo run
```

### Running tests
```bash
cargo test
```

### Database migrations

Create a new migration:
```bash
sqlx migrate add migration_name
```

Run migrations:
```bash
sqlx migrate run
```

Revert last migration:
```bash
sqlx migrate revert
```

## Error Handling

All endpoints return consistent error responses:

```json
{
  "success": false,
  "data": null,
  "message": "Error description"
}
```

HTTP Status Codes:
- `200` - Success
- `400` - Bad Request (invalid input)
- `404` - Not Found
- `500` - Internal Server Error

## CORS Configuration

The API has CORS enabled for all origins. For production, modify the CORS settings in `main.rs`:

```rust
CorsLayer::new()
    .allow_origin("https://yourdomain.com".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_headers(Any)
```

## Performance Considerations

- Database queries use connection pooling via SQLx
- JSONB fields are indexed for fast queries
- Pagination limits prevent large data transfers
- Bulk operations use database transactions

## Roadmap

- [ ] Authentication and authorization
- [ ] User management
- [ ] Quiz sessions and scoring
- [ ] Question categories and tags filtering
- [ ] Export/import in various formats (JSON, CSV)
- [ ] Question statistics and analytics
- [ ] Rate limiting
- [ ] Caching layer
- [ ] Full-text search with PostgreSQL FTS
- [ ] API documentation with OpenAPI/Swagger

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues, questions, or contributions, please open an issue on GitHub.

## Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum)
- Database migrations with [SQLx](https://github.com/launchbadge/sqlx)
- Async runtime by [Tokio](https://tokio.rs/)