# MongoDB + Rust Examples

A collection of Rust projects demonstrating how to use the [MongoDB Rust Driver](https://github.com/mongodb/mongo-rust-driver) across a variety of use cases — from basic CRUD operations to vector search, geospatial queries, and AWS Lambda deployments.

## Projects

### [mongodb-proj](mongodb-proj/) — CRUD Operations

Comprehensive reference for core MongoDB operations, each in its own module:

- Insert single and bulk documents
- Find one / find many with filters
- Update single and bulk documents
- Delete single and bulk documents
- Count documents and list distinct fields
- Geospatial search with 2dsphere indexes

Uses the `sample_restaurants` dataset on MongoDB Atlas.

### [mongodb-playground](mongodb-playground/) — Quick Start

Minimal project for experimenting with MongoDB connections and basic insert/find operations using the `sample_restaurants` dataset.

### [geo-search](geo-search/) — Geospatial Queries

Demonstrates geospatial querying on the `sample_geospatial.shipwrecks` collection using GeoJSON Point queries and the `$near` operator to find records within a specified distance from coordinates.

### [mongo-atlas-vs](mongo-atlas-vs/) — Atlas Search & Vector Search

Shows how to use MongoDB Atlas Search with:

- **Vector search** — `$vectorSearch` aggregation stage on the `sample_mflix.embedded_movies` collection (1536-dimension embeddings, dot product similarity)
- **Compound search** — `$search` with compound operators combining text, regex, and exclusion filters

### [mongo-airbnb](mongo-airbnb/) — Vector Search with Filters

End-to-end vector search pipeline on an Airbnb listings dataset:

- Load and index listing data from JSON
- Generate embeddings via OpenAI (`text-embedding-3-small`)
- Create vector search indexes with filter fields
- Run filtered vector search queries
- Includes a Jupyter notebook ([Lesson_1.ipynb](mongo-airbnb/Lesson_1.ipynb))

### [mongo-embedding](mongo-embedding/) — Embeddings & Semantic Search

Processes a news dataset with vector embeddings for semantic search:

- Load JSONL news data into MongoDB
- Generate embeddings via the Anthropic API
- Create vector search indexes (1536 dimensions, dot product similarity)
- Query with vector search and metadata retrieval

### [polymorphic_data](polymorphic_data/) — Polymorphic Documents

Demonstrates working with heterogeneous document schemas in a single collection. Queries a sports dataset where documents have varying fields (`grand_slam_wins`, `career_earnings`, etc.) and shows how to deserialize them with flexible Rust types.

### [gestor_proyectos](gestor_proyectos/) — CLI Project Manager

Command-line project and task manager built with [Clap](https://github.com/clap-rs/clap). Supports creating projects, adding tasks, viewing tasks, and updating task status against a local MongoDB instance.

### [hackathon-mongodb](hackathon-mongodb/) — AWS Lambda + MongoDB

AWS Lambda function that handles HTTP requests to insert and query restaurant data. Retrieves MongoDB credentials from AWS Secrets Manager and caches them locally for performance.

Build and deploy with [Cargo Lambda](https://www.cargo-lambda.info/):

```bash
cargo lambda build --release
cargo lambda deploy
```

### [mongodb-lambda-proj](mongodb-lambda-proj/) — Lambda Starter

Simpler AWS Lambda + MongoDB template. A minimal starting point for serverless MongoDB functions on AWS.

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- A running MongoDB instance — either local (`mongod`) or [MongoDB Atlas](https://www.mongodb.com/atlas)
- For vector search projects: an Atlas cluster with Atlas Search enabled
- For Lambda projects: [Cargo Lambda](https://www.cargo-lambda.info/) and an AWS account

## Environment Variables

Each project uses one of the following for its connection string:

| Variable | Used by |
|---|---|
| `MONGODB_URI` | geo-search, mongo-airbnb, mongo-atlas-vs, mongo-embedding, polymorphic_data |
| `MONGODB_SRV` | mongodb-playground |
| `MONGODB_PASS` | mongodb-proj |
| `MONGODB_SECRET_NAME` | hackathon-mongodb (AWS Secrets Manager secret name) |

gestor_proyectos connects directly to `mongodb://localhost:27017`.

## Running an Example

```bash
cd mongodb-proj
export MONGODB_PASS="your-atlas-password"
cargo run
```

## Key Dependencies

All projects share a common foundation:

| Crate | Purpose |
|---|---|
| `mongodb` | Official MongoDB Rust driver (v3.1–3.2) |
| `tokio` | Async runtime |
| `serde` / `bson` | Serialization and BSON document handling |
| `futures` | Stream processing for cursors |
| `reqwest` | HTTP client (embedding API calls) |
| `lambda_http` / `lambda_runtime` | AWS Lambda integration |
| `clap` | CLI argument parsing |
