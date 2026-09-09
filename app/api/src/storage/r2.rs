//! S3-API object storage upload/delete. Not yet built. Talks to MinIO
//! locally (BB26090903, self-hosted, S3_ENDPOINT/S3_ACCESS_KEY/
//! S3_SECRET_KEY/S3_BUCKET in .env) and can point at Cloudflare R2 later
//! by swapping the endpoint alone, since both speak the same S3 API via
//! aws-sdk-s3 -- no code change, per the row's own design point.
