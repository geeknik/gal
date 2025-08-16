# GAL Package Registry System

The GAL Package Registry System provides comprehensive package management capabilities for the GAL programming language, including publishing, discovery, dependency resolution, and security verification.

## Overview

The package system is designed with the following principles:

- **Security First**: All packages are scanned for vulnerabilities and security issues
- **Deterministic Builds**: Reproducible dependency resolution with lock files
- **Enterprise Ready**: Support for private registries and enterprise authentication
- **Chaos Engineering**: Integration with GAL's chaos engineering features
- **Gödelian Features**: Support for self-modifying and meta-programming packages

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   gal-pkg CLI   │────│  Package Cache  │────│    Registry     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │              ┌─────────────────┐              │
         └──────────────│  Dependency     │──────────────┘
                        │   Resolver      │
                        └─────────────────┘
                                 │
                        ┌─────────────────┐
                        │  Security       │
                        │   Scanner       │
                        └─────────────────┘
```

## Core Components

### 1. Package Manifest (`gal.toml`)

The package manifest defines package metadata, dependencies, and configuration:

```toml
[package]
name = "my-package"
version = "1.0.0"
authors = ["Your Name <your@email.com>"]
description = "A GAL package"
license = "MIT"
repository = "https://github.com/user/my-package"
homepage = "https://my-package.example.com"
keywords = ["actor", "concurrency"]
categories = ["actor-model", "utilities"]
edition = "2024"
gal-version = "1.0"

[dependencies]
actor-utils = "^1.2.0"
chaos-lib = { version = "0.5", features = ["scheduling"] }
local-dep = { path = "../local-dep" }
git-dep = { git = "https://github.com/user/repo", tag = "v1.0" }

[dev-dependencies]
test-framework = "^2.0"

[build-dependencies]
codegen-helper = "1.0"

# Chaos engineering profiles
[chaos-profile.production]
fault_injection_rate = 0.001
allowed_faults = ["network_delay", "packet_loss"]
max_fault_duration = "5s"
network_partition_probability = 0.0001
message_drop_rate = 0.001

[chaos-profile.testing]
fault_injection_rate = 0.5
allowed_faults = ["all"]
chaos_seed = 12345
max_fault_duration = "1m"
network_partition_probability = 0.1
message_drop_rate = 0.2

# Build profiles
[profile.dev]
optimization_level = 0
debug = true
chaos_profile = "testing"

[profile.release]
optimization_level = 3
debug = false
lto = "fat"
chaos_profile = "production"

# Features
[features]
default = ["standard"]
standard = ["actor-utils/default"]
experimental = ["chaos-lib/experimental"]
```

### 2. Lock File (`gal.lock`)

The lock file ensures deterministic builds by recording exact dependency versions:

```toml
version = 1
root = "my-package"

[[packages]]
name = "actor-utils"
version = "1.2.3"
checksum = "sha256:abc123..."
features = ["default"]

[[packages]]
name = "chaos-lib"
version = "0.5.2"
checksum = "sha256:def456..."
features = ["scheduling"]
dependencies = ["rand"]

[[packages]]
name = "rand"
version = "0.8.5"
checksum = "sha256:789abc..."
```

### 3. Registry Client

The registry client handles communication with package registries:

```rust
use gal::package::{RegistryClient, RegistryConfig, AuthCredentials};

// Configure registry
let config = RegistryConfig {
    name: "company-registry".to_string(),
    url: "https://packages.company.com".to_string(),
    auth_type: AuthType::Token,
    verify_signatures: true,
    ..Default::default()
};

// Create client
let cache = PackageCache::new(&cache_dir)?;
let mut client = RegistryClient::new(config, cache)?;

// Authenticate
client.authenticate(AuthCredentials::Token(token)).await?;

// Search packages
let results = client.search_packages("actor", 1, 20).await?;

// Publish package
client.publish_package(&package_path, false).await?;
```

### 4. Dependency Resolver

The dependency resolver uses a SAT-based algorithm to find compatible package versions:

```rust
use gal::package::{DependencyResolver, PackageManifest, PackageCache};

let resolver = DependencyResolver::new();
let manifest = PackageManifest::load(&manifest_path)?;
let cache = PackageCache::new(&cache_dir)?;

// Resolve dependencies
let lock_file = resolver.resolve(&manifest, &cache)?;

// Resolve with specific profile and features
let lock_file = resolver.resolve_with_profile(
    &manifest, 
    &cache, 
    Some("release"), 
    &["experimental"]
)?;
```

### 5. Security Scanner

The security scanner performs comprehensive security analysis:

```rust
use gal::package::SecurityScan;

// Perform security scan
let scan_result = SecurityScan::scan_package(&package_path).await?;

// Check results
if scan_result.has_critical_issues() {
    eprintln!("Critical security issues found:");
    for issue in &scan_result.critical_issues {
        eprintln!("- {}: {}", issue.title, issue.description);
    }
}

let security_score = scan_result.get_security_score();
println!("Security score: {}/10", security_score);
```

## CLI Usage

### Package Initialization

```bash
# Initialize new package
gal-pkg init my-package

# Initialize in specific directory
gal-pkg init my-package --path ./packages/my-package
```

### Dependency Management

```bash
# Add dependency
gal-pkg add actor-utils@^1.2.0

# Add development dependency
gal-pkg add test-framework@^2.0 --dev

# Add with features
gal-pkg add chaos-lib@0.5 --features scheduling,experimental

# Remove dependency
gal-pkg remove actor-utils

# Install dependencies
gal-pkg install

# Install with specific profile
gal-pkg install --profile release

# Update dependencies
gal-pkg update

# Update specific package
gal-pkg update actor-utils
```

### Package Discovery

```bash
# Search packages
gal-pkg search "actor system"

# Get package information
gal-pkg info actor-utils
gal-pkg info actor-utils@1.2.3

# Show dependency tree
gal-pkg tree

# Show dependency tree with specific depth
gal-pkg tree --depth 3
```

### Publishing

```bash
# Publish package
gal-pkg publish

# Publish to specific registry
gal-pkg publish --registry company-registry

# Dry run (validate without publishing)
gal-pkg publish --dry-run

# Sign package
gal-pkg publish --sign ./my-key.pem

# Yank published version
gal-pkg yank my-package@1.0.0 --reason "Security vulnerability"
```

### Security

```bash
# Security scan
gal-pkg scan

# Scan with JSON output
gal-pkg scan --output json

# Fail on specific security level
gal-pkg scan --fail-on high

# Audit dependencies
gal-pkg audit

# Audit with automatic fixes
gal-pkg audit --fix
```

### Registry Management

```bash
# Add registry
gal-pkg registry add company https://packages.company.com

# List registries
gal-pkg registry list

# Remove registry
gal-pkg registry remove company

# Login to registry
gal-pkg login --registry company --token abc123

# Logout from registry
gal-pkg logout --registry company
```

### Utilities

```bash
# Check package for issues
gal-pkg check

# Clean cache
gal-pkg clean --cache

# Clean everything
gal-pkg clean --all

# Configuration management
gal-pkg config set default-registry company
gal-pkg config get default-registry
gal-pkg config list
```

## Registry Configuration

### Multiple Registries

Configure multiple registries in `~/.config/gal/registries.toml`:

```toml
default_registry = "default"

[registries.default]
name = "default"
url = "https://registry.gal-lang.org"
auth_type = "Token"
verify_signatures = true
allow_insecure = false
timeout_seconds = 30
max_retries = 3

[registries.company]
name = "company"
url = "https://packages.company.com"
auth_type = "Token"
verify_signatures = true
public_keys = ["..."]
trusted_publishers = ["company-team"]
```

### Authentication Types

1. **Token Authentication**
   ```bash
   gal-pkg login --token your-api-token
   ```

2. **Basic Authentication**
   ```bash
   gal-pkg login --username user --password pass
   ```

3. **OAuth2** (planned)
   ```bash
   gal-pkg login --oauth2
   ```

## Security Features

### Package Signing

Packages can be cryptographically signed to ensure authenticity:

```bash
# Generate signing key
gal-pkg keygen --output ./my-key.pem

# Sign package during publish
gal-pkg publish --sign ./my-key.pem

# Configure automatic signing
gal-pkg config set signing.key ./my-key.pem
gal-pkg config set signing.auto true
```

### Vulnerability Scanning

The security scanner checks for:

- **Known Vulnerabilities**: CVE database integration
- **Malicious Code**: Pattern detection for suspicious code
- **Dependency Risks**: Outdated or vulnerable dependencies
- **Supply Chain**: Maintainer verification and repository security
- **Code Quality**: Security-related code issues
- **License Compliance**: License compatibility checking

### Security Policies

Configure security policies in the manifest:

```toml
[security]
require_signatures = true
max_risk_level = "medium"
vulnerability_scan = true
dependency_audit = true
allowed_licenses = ["MIT", "Apache-2.0", "BSD-3-Clause"]
```

## Enterprise Features

### Private Registries

Deploy private registries for internal packages:

```yaml
# docker-compose.yml
version: '3.8'
services:
  gal-registry:
    image: gal-lang/registry:latest
    environment:
      - REGISTRY_AUTH_TYPE=ldap
      - REGISTRY_LDAP_URL=ldap://company-ldap:389
      - REGISTRY_REQUIRE_SIGNATURES=true
    volumes:
      - registry-data:/var/lib/gal-registry
    ports:
      - "8080:8080"
```

### CI/CD Integration

Integrate with CI/CD pipelines:

```yaml
# .github/workflows/build.yml
name: Build and Test
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Setup GAL
        uses: gal-lang/setup-gal@v1
        with:
          version: '1.0'
      
      - name: Install dependencies
        run: gal-pkg install
      
      - name: Security scan
        run: gal-pkg scan --fail-on high
      
      - name: Audit dependencies
        run: gal-pkg audit
      
      - name: Build
        run: galc build
      
      - name: Test
        run: galc test
      
      - name: Publish (on release)
        if: github.event_name == 'release'
        run: gal-pkg publish
        env:
          GAL_REGISTRY_TOKEN: ${{ secrets.GAL_REGISTRY_TOKEN }}
```

### Access Control

Configure fine-grained access control:

```toml
# Registry access control
[access]
read_public = true
write_requires_auth = true
admin_users = ["admin@company.com"]

[access.packages."company/*"]
read_groups = ["employees"]
write_groups = ["developers"]
admin_groups = ["package-maintainers"]
```

## Chaos Engineering Integration

The package system integrates with GAL's chaos engineering features:

### Chaos Profiles

Define different chaos behavior for different environments:

```toml
[chaos-profile.production]
fault_injection_rate = 0.001
allowed_faults = ["network_delay", "packet_loss"]
max_fault_duration = "5s"

[chaos-profile.staging]
fault_injection_rate = 0.1
allowed_faults = ["all"]
max_fault_duration = "30s"

[chaos-profile.testing]
fault_injection_rate = 0.5
allowed_faults = ["all"]
chaos_seed = 12345
max_fault_duration = "1m"
```

### Fault Injection

Schedule specific faults for testing:

```toml
[[chaos-profile.testing.fault_schedule]]
fault_type = "message_drop"
trigger = "on_send"
probability = 0.1
duration = "100ms"

[[chaos-profile.testing.fault_schedule]]
fault_type = "network_partition"
trigger = "cron:0 */5 * * * *"  # Every 5 minutes
probability = 1.0
duration = "30s"
```

## Performance and Scaling

### Caching Strategy

The package cache uses a multi-level hierarchy:

1. **Memory Cache**: Recently accessed packages
2. **Local Disk Cache**: Downloaded packages and metadata
3. **Shared Cache**: Network-accessible cache for teams
4. **Registry Cache**: CDN-backed package distribution

### Parallel Operations

Package operations are optimized for performance:

- **Parallel Downloads**: Multiple packages downloaded concurrently
- **Incremental Updates**: Only changed dependencies are updated
- **Lazy Loading**: Dependencies loaded on-demand
- **Compression**: Packages compressed for faster transfer

### Metrics and Monitoring

Monitor package system performance:

```rust
use gal::package::PackageMetrics;

let metrics = PackageMetrics::global();
println!("Packages cached: {}", metrics.cached_packages());
println!("Registry requests: {}", metrics.registry_requests());
println!("Cache hit rate: {:.2}%", metrics.cache_hit_rate() * 100.0);
```

## Best Practices

### Package Development

1. **Semantic Versioning**: Use semantic versioning for releases
2. **Small Dependencies**: Minimize dependency count and size
3. **Feature Flags**: Use features for optional functionality
4. **Documentation**: Include comprehensive documentation
5. **Testing**: Include thorough test coverage
6. **Security**: Regular security audits and updates

### Dependency Management

1. **Version Constraints**: Use appropriate version constraints
2. **Regular Updates**: Keep dependencies up to date
3. **Security Monitoring**: Monitor for security advisories
4. **Minimal Dependencies**: Only include necessary dependencies
5. **Lock Files**: Commit lock files for reproducible builds

### Registry Operations

1. **Staging**: Test packages in staging environment first
2. **Gradual Rollout**: Use feature flags for gradual rollout
3. **Monitoring**: Monitor package usage and performance
4. **Backup**: Regular backup of package data
5. **Access Control**: Implement proper access controls

## API Reference

### Core Types

```rust
// Package manifest
pub struct PackageManifest {
    pub package: PackageInfo,
    pub dependencies: HashMap<String, DependencySpec>,
    pub dev_dependencies: HashMap<String, DependencySpec>,
    pub build_dependencies: HashMap<String, DependencySpec>,
    pub chaos_profiles: HashMap<String, ChaosProfile>,
    pub features: HashMap<String, Vec<String>>,
    // ...
}

// Registry client
pub struct RegistryClient {
    // ...
}

// Dependency resolver
pub struct DependencyResolver {
    // ...
}

// Security scanner
pub struct SecurityScan {
    pub package_name: String,
    pub package_version: String,
    pub overall_risk: RiskLevel,
    pub vulnerability_scan: VulnerabilityScan,
    pub dependency_audit: DependencyAudit,
    pub code_analysis: CodeAnalysis,
    // ...
}
```

### Error Types

```rust
pub enum GalError {
    PackageError(String),
    RegistryError(String),
    AuthError(String),
    SecurityError(String),
    ConfigError(String),
    // ...
}
```

## Troubleshooting

### Common Issues

1. **Authentication Failures**
   ```bash
   # Check stored credentials
   gal-pkg config get auth.token
   
   # Re-authenticate
   gal-pkg login --registry default
   ```

2. **Dependency Conflicts**
   ```bash
   # Show detailed dependency tree
   gal-pkg tree --duplicates
   
   # Update lock file
   rm gal.lock && gal-pkg install
   ```

3. **Security Scan Failures**
   ```bash
   # Get detailed security report
   gal-pkg scan --output json | jq .
   
   # Update vulnerable dependencies
   gal-pkg audit --fix
   ```

4. **Registry Connection Issues**
   ```bash
   # Test registry connectivity
   gal-pkg registry list
   
   # Use alternative registry
   gal-pkg install --registry backup-registry
   ```

### Debug Mode

Enable debug logging for troubleshooting:

```bash
export RUST_LOG=gal::package=debug
gal-pkg install
```

## Migration Guide

### From Other Package Managers

#### From Cargo

```bash
# Convert Cargo.toml to gal.toml
gal-pkg migrate --from cargo

# Import dependencies
gal-pkg add --from-cargo Cargo.toml
```

#### From npm

```bash
# Convert package.json to gal.toml
gal-pkg migrate --from npm

# Import compatible dependencies
gal-pkg add --from-npm package.json
```

### Version Upgrades

When upgrading the package system:

1. **Backup**: Backup package cache and configuration
2. **Update**: Update gal-pkg binary
3. **Migrate**: Run migration commands if needed
4. **Verify**: Verify package functionality

## Contributing

The GAL package system is open source and welcomes contributions:

1. **Repository**: https://github.com/geeknik/gal
2. **Issues**: Report bugs and feature requests
3. **Pull Requests**: Submit improvements and fixes
4. **Documentation**: Help improve documentation
5. **Testing**: Test with your packages and report issues

## License

The GAL Package Registry System is licensed under the MIT License.
