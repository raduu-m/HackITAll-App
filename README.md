# Student Finance App 

A secure and app for managing student finances, built with Rust and MongoDB. This service provides user authentication, balance management, and transaction handling with real-time system metrics monitoring.

## Features

- User Account Management
  - User registration and authentication
  - Secure password encryption using Caesar cipher (Note: for educational purposes only)
  - Balance tracking and management

- Transaction System
  - Real-time money transfers between users
  - Transaction history tracking
  - Transaction reversion capability
  - Automatic balance updates

- System Monitoring
  - CPU usage metrics
  - Memory usage tracking
  - Prometheus integration for metrics collection

## Technical Stack

- **Backend**: Rust with Actix-web framework
- **Database**: MongoDB
- **Monitoring**: Prometheus metrics
- **System Stats**: systemstat for resource monitoring
-**Frontend**: Vue.js
## API Endpoints

### User Management
- `GET /` - Health check endpoint
- `POST /user/login/{email}/{password}` - User login
- `POST /user` - Create new user
- `PUT /user/{id}` - Update user details
- `DELETE /user/{id}` - Delete user
- `GET /user/{id}` - Get user details

### Transaction Management
- `GET /user/transactions/{id}` - Get user's transaction history
- `POST /transaction` - Create new transaction
- `PUT /transactionx/` - Revert transaction

### Monitoring
- `GET /metrics` - Prometheus metrics endpoint

## Getting Started

### Prerequisites
- Rust (latest stable version)
- MongoDB
- Cargo (Rust package manager)
-Vue.js

### Installation

1. Clone the repository
```bash
git clone [repository-url]
```

2. Install dependencies
```bash
cargo build
```

3. Configure MongoDB connection
   - Ensure MongoDB is running locally
   - Update connection settings if necessary

4. Run the application
```bash
cargo run
```

The server will start on `localhost:8080`

## Security Notes

- The current password encryption uses a basic Caesar cipher for educational purposes
- For production use, implement a proper cryptographic hashing algorithm
- Ensure proper authentication and authorization mechanisms are in place
- Review and update security measures before deploying to production

## Monitoring

The application includes built-in monitoring capabilities:
- CPU usage tracking
- Memory usage monitoring
- Prometheus metrics available at `/metrics`

