# 🎬 Movie API

A RESTful API built with Rust and Actix-web for managing movie information. This service allows you to store and retrieve movie details using MongoDB as the backend database.

## ✨ Features

- Modern Rust-based web API
- MongoDB integration for persistent storage
- RESTful architecture
- Movie information management
- Structured error handling

## 🛠️ Tech Stack

- **Rust** - Systems programming language known for performance and safety
- **Actix-web** - Fast, pragmatic web framework for Rust
- **MongoDB** - NoSQL database for flexible data storage
- **Serde** - Powerful serialization/deserialization framework
- **Chrono** - Date and time handling

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable version)
- MongoDB
- Git

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd first-api
```

2. Create a `.env` file in the root directory:
```env
MONGO_URI=your_mongodb_connection_string
```

3. Build and run the project:
```bash
cargo build
cargo run
```

The server will start at `http://localhost:5001`

## 📡 API Endpoints

### Base URL
```
http://localhost:5001
```

### Endpoints

#### Health Check
```http
GET /
Response: "Hello World"
```

#### Add Movie
```http
POST /add_movie
Content-Type: application/json

{
    "title": "string",
    "plot": "string",
    "rating": number,
    "genre": "string",
    "director": "string"
}
```

## 📁 Project Structure

```
src/
├── main.rs           # Application entry point and server setup
├── models/           # Data structures and types
│   ├── mod.rs
│   └── movie.rs     # Movie model definition
├── routes/          # API route handlers
│   ├── mod.rs
│   └── movies_route.rs
└── services/        # Business logic and database operations
    ├── mod.rs
    └── db.rs        # Database connection and operations
```

## 🧱 Components

### Movie Model
The API handles movie data with the following structure:
```rust
pub struct Movie {
    title: String,
    plot: String,
    rating: i32,
    genre: String,
    director: String
}
```

### Database Operations
- Insert new movies
- Retrieve all movies
- MongoDB integration with error handling

## 🔒 Environment Variables

- `MONGO_URI`: MongoDB connection string (required)

## 🛡️ Error Handling

The API implements comprehensive error handling for:
- Database operations
- Request validation
- Server errors

## 🤝 Contributing

Feel free to contribute to this project:
1. Fork the repository
2. Create a new branch
3. Make your changes
4. Submit a pull request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.
