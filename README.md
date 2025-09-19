# 🔐 Hashemi - Professional Login System

A modern, professional login system built with Rust using **Onion Architecture** principles. Features a robust backend with Axum, elegant frontend demonstrations, and comprehensive security.

## ✨ Features

- 🦀 **Rust Backend** - High-performance, memory-safe server
- ⚡ **Axum Framework** - Modern async web framework
- 🏗️ **Onion Architecture** - Clean, maintainable code structure
- 🔐 **JWT Authentication** - Secure token-based auth
- 🔒 **bcrypt Password Hashing** - Industry-standard security
- 🌐 **CORS Support** - Cross-origin request handling
- 📱 **Responsive UI** - Beautiful, professional interface
- 🧪 **Comprehensive Tests** - Unit and integration testing

## 🏗️ Architecture

The project follows **Onion Architecture** with clear separation of concerns:

```
📦 Hashemi/
├── 🔧 backend/                 # Axum backend server
│   ├── src/
│   │   ├── 🎯 domain/         # Core business entities
│   │   │   ├── user.rs        # User entity
│   │   │   └── repository.rs  # Repository contracts
│   │   ├── 💼 application/    # Business logic layer
│   │   │   └── user_service.rs # Authentication service
│   │   ├── 🏗️ infrastructure/ # External concerns
│   │   │   └── postgres_user_repo.rs # Database implementation
│   │   ├── 🌐 presentation/   # HTTP/API layer
│   │   │   └── auth_handlers.rs # Login endpoints
│   │   └── main.rs           # Application entry point
├── 🎨 frontend/               # Leptos frontend (planned)
├── 📄 demo.html              # Interactive demo page
├── 🗄️ migrations/            # Database migrations
└── 📚 README.md              # This file
```

## 🚀 Quick Start

### 1. Prerequisites
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install PostgreSQL (optional - demo works without it)
brew install postgresql  # macOS
```

### 2. Clone and Build
```bash
git clone <your-repo-url>
cd Hashemi
cargo build
```

### 3. Run Demo Server
```bash
# Start the backend server
cargo run --bin backend

# In another terminal, serve the demo page
python3 -m http.server 8080

# Open http://localhost:8080/demo.html in your browser
```

### 4. Test the API
```bash
# Successful login
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"test123"}'

# Failed login
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"wrong"}'

# Health check
curl http://localhost:3000/health
```

## 🔧 Configuration

### Environment Variables (.env)
```env
DATABASE_URL=postgresql://username:password@localhost/hashemi_db
JWT_SECRET=your-super-secret-jwt-key-here-make-it-very-long-and-secure
JWT_EXP=3600
```

### Demo Credentials
- **Email:** `test@example.com`
- **Password:** `test123`

## 📊 API Endpoints

### POST /login
Authenticate user and return JWT token.

**Request:**
```json
{
  "email": "test@example.com",
  "password": "test123"
}
```

**Success Response (200):**
```json
{
  "token": "demo-jwt-token-12345",
  "message": "Login successful"
}
```

**Error Response (401):**
```json
{
  "error": "Invalid credentials"
}
```

### GET /health
Health check endpoint.

**Response (200):**
```
OK
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run backend tests only
cargo test -p backend

# Run with output
cargo test -- --nocapture
```

## 🛠️ Development

### Backend Development
```bash
# Run backend in development mode
cargo watch -x "run --bin backend"

# Check for issues
cargo clippy

# Format code
cargo fmt
```

### Database Setup (Optional)
```bash
# Install sqlx-cli
cargo install sqlx-cli

# Run migrations
sqlx migrate run

# Create new migration
sqlx migrate add create_users_table
```

## 📦 Dependencies

### Backend Core
- **axum** `0.6` - Web framework
- **tokio** `1.0` - Async runtime
- **serde** `1.0` - Serialization
- **anyhow** `1.0` - Error handling

### Authentication & Security
- **jsonwebtoken** `8.3` - JWT handling
- **bcrypt** `0.15` - Password hashing

### Database (Optional)
- **sqlx** `0.6` - Database toolkit
- **uuid** `1.0` - UUID generation
- **chrono** `0.4` - Date/time handling

### HTTP & Middleware
- **tower-http** `0.4` - HTTP middleware
- **dotenvy** `0.15` - Environment variables

## 🎯 Demo Features

The interactive demo includes:
- ✅ **Professional UI** - Modern, responsive design
- 🔄 **Real-time Testing** - Live API interaction
- 📱 **Mobile Friendly** - Works on all devices
- 🎨 **Persian/English Support** - Bilingual interface
- 🚀 **Tech Stack Display** - Shows all technologies used

## 🌟 Production Readiness

- 🔒 **Security Best Practices** - JWT + bcrypt
- 📊 **Error Handling** - Comprehensive error responses
- 🧪 **Test Coverage** - Unit and integration tests
- 📚 **Documentation** - Clear API documentation
- 🏗️ **Scalable Architecture** - Clean separation of concerns

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and test them
4. Commit: `git commit -am 'Add feature'`
5. Push: `git push origin feature-name`
6. Create a Pull Request

## 📝 License

This project is licensed under the MIT License.

## 🎉 Acknowledgments

Built with ❤️ using:
- 🦀 [Rust](https://www.rust-lang.org/)
- ⚡ [Axum](https://github.com/tokio-rs/axum)
- 🎯 [Leptos](https://leptos.dev/)
- 🐘 [PostgreSQL](https://www.postgresql.org/)

---

**🚀 Happy Coding!** - Built by developers, for developers.
