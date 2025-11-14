# Modernization Checklist

## Static Serving Logic (`src/lib.rs`)
- [ ] Benchmark current throughput/latency to set performance baselines.
- [ ] Replace manual extension matching with `mime_guess` or `mime` crate for accuracy and maintenance.
- [ ] Ensure middleware skips re-setting `Content-Type` if already present (avoid double headers).
- [ ] Add configurable default MIME fallback instead of hardcoded `application/octet-stream`.
- [ ] Support dynamic content negotiation (e.g., gzip, brotli, zstd) via `tower_http::CompressionLayer` when enabled.
- [ ] Provide optional cache-control header management (ETag, Last-Modified, immutable hints).
- [ ] Add security headers (CSP, X-Content-Type-Options, X-Frame-Options) via optional middleware.
- [ ] Validate path normalization to prevent directory traversal (enable `ServeDir::precompressed_assets` if suitable).
- [ ] Offer toggle for range request handling to optimize large asset delivery.
- [ ] Parameterize directory index handling (custom index file names, disable feature).
- [ ] Provide logging/tracing hooks via `tower_http::TraceLayer` with sensible defaults.
- [ ] Document middleware order expectations and extension points in README/docs.rs.

## Error Handling (`static_router`)
- [ ] Expand `handle_error` to differentiate client vs server IO errors for clearer diagnostics.
- [ ] Emit structured logs/metrics for IO errors (with `tracing` spans).
- [ ] Allow custom error responders via generic type parameter or builder pattern without breaking API.

## Cargo & Release Hygiene (`Cargo.toml`)
- [ ] Audit dependency versions for security/perf updates; evaluate optional features (compression, trace, headers).
- [ ] Expose feature flags for optional middleware (compression, cache-control, security headers) to keep API stable but extensible.
- [ ] Add MSRV policy and CI badge in README/docs metadata.
- [ ] Ensure docs.rs metadata enables all features for documentation examples.
- [ ] Consider enabling `tracing`/`tokio-console` support behind feature flags for production diagnostics.

## Testing & CI
- [ ] Add integration tests covering various file types, existing headers, and missing files.
- [ ] Include load/regression tests for high-concurrency scenarios.
- [ ] Provide GitHub Actions workflow for lint, test, fmt, wasm32 builds.

## Documentation
- [ ] Publish usage cookbook: static site, SPA, mixed dynamic/static, S3-backed.
- [ ] Document security best practices (MIME sniffing, cache policies, directory listing controls).
- [ ] Add migration guide for future breaking releases while keeping current API stable.
Consider these refinements:

Use mime_guess to replace the manual match table. It reduces maintenance and improves MIME coverage instantly.
Skip header insertion when Content-Type already exists to avoid overriding types set by upstream middleware.
Provide a configurable fallback MIME type (e.g., via state or environment) instead of hard-coded "application/octet-stream".
Normalize and sanitize paths before extension extraction to prevent odd URIs from misleading the lookup.
Cache lookup results for common extensions in a small LRU or static map to minimize string allocations on hot paths.
Add tracing/log hooks (behind feature flags) to record unknown extensions or unusual overrides for diagnostics.
Support conditional compression negotiation (tying into CompressionLayer) so the middleware can cooperate with future content-encoding logic.