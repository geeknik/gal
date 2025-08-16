# The ? Operator in GAL

The question mark operator (`?`) provides ergonomic error propagation in GAL, similar to Rust's implementation but adapted for the actor model. It works with both `Result<T, E>` and `Option<T>` types to enable clean, readable error handling.

## Basic Usage

### With Result Types

```gal
fn read_config(path: String) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)?;  // Propagates IoError as ConfigError
    let config = parse_config(content)?;     // Propagates ParseError as ConfigError
    Ok(config)
}
```

The `?` operator on `Result<T, E>`:
- If the result is `Ok(value)`, unwraps and returns `value`
- If the result is `Err(error)`, converts the error (if needed) and returns early

### With Option Types

```gal
fn get_user_email(user_id: String) -> Option<String> {
    let user = find_user(user_id)?;      // Returns None if user not found
    let profile = user.get_profile()?;   // Returns None if no profile
    Some(profile.email)
}
```

The `?` operator on `Option<T>`:
- If the option is `Some(value)`, unwraps and returns `value`
- If the option is `None`, returns `None` early

## Error Conversion with From Trait

GAL automatically converts error types using the `From` trait when using `?`:

```gal
enum AppError {
    Io(IoError),
    Parse(ParseError),
    Database(DbError)
}

impl From<IoError> for AppError {
    fn from(err: IoError) -> AppError {
        AppError::Io(err)
    }
}

impl From<ParseError> for AppError {
    fn from(err: ParseError) -> AppError {
        AppError::Parse(err)
    }
}

fn process_file(path: String) -> Result<Data, AppError> {
    let content = fs::read_to_string(path)?;  // IoError -> AppError
    let data = parse_data(content)?;          // ParseError -> AppError
    Ok(data)
}
```

## Actor Model Integration

The `?` operator works seamlessly with GAL's actor model:

```gal
actor DatabaseActor {
    state connection: Option<Connection>
    
    on Connect(url: String) -> Result<(), DatabaseError> {
        let conn = establish_connection(url)?;
        self.connection = Some(conn);
        Ok(())
    }
    
    on Query(sql: String) -> Result<QueryResult, DatabaseError> {
        let conn = self.connection.as_ref()?;  // Convert None to DatabaseError
        let result = conn.execute(sql)?;
        Ok(result)
    }
}
```

### Message Handler Return Types

Actor message handlers can return `Result` or `Option` types, and the `?` operator respects these return types:

```gal
actor FileProcessor {
    on ProcessFile(path: String) -> Result<ProcessedData, ProcessingError> {
        let content = self.read_file(path)?;    // Early return on file read error
        let validated = self.validate(content)?; // Early return on validation error
        let processed = self.transform(validated)?;
        Ok(processed)
    }
}
```

## Method Chaining

The `?` operator works naturally with method chaining:

```gal
fn complex_pipeline(input: String) -> Result<Output, PipelineError> {
    let result = input
        .trim()
        .parse::<i32>()?
        .validate()?
        .transform()?
        .finalize()?;
    Ok(result)
}
```

## Explicit Error Conversion

For cases where automatic conversion isn't available, you can specify explicit conversions:

```gal
fn custom_conversion(data: String) -> Result<ProcessedData, CustomError> {
    let parsed = data.parse::<i32>()? as CustomError;  // Explicit conversion
    Ok(ProcessedData::new(parsed))
}
```

## Type Safety

The `?` operator includes comprehensive type checking:

### Valid Usage
- Expression must be of type `Result<T, E>` or `Option<T>`
- Function must return compatible `Result<_, E2>` or `Option<_>` type
- Error types must be convertible via `From` trait

### Compile-Time Errors

```gal
fn invalid_usage() -> String {
    let value: i32 = 42;
    value?  // ERROR: Cannot use ? operator on type i32
}

fn wrong_return_type() -> String {
    let result: Result<i32, Error> = Ok(42);
    result?  // ERROR: Function returns String, not Result
}

fn incompatible_errors() -> Result<String, ErrorA> {
    let result: Result<String, ErrorB> = get_result();
    result?  // ERROR: ErrorB cannot be converted to ErrorA
}
```

## Built-in Trait Implementations

GAL provides several built-in `From` implementations for common conversions:

```gal
// Standard library errors
impl From<IoError> for String
impl From<ParseIntError> for String
impl From<Utf8Error> for String

// Custom error hierarchies
trait Error {
    fn description(&self) -> String;
}

impl<T: Error> From<T> for Box<dyn Error>
```

## Advanced Usage

### Custom Try Trait Implementation

For advanced use cases, you can implement the `Try` trait for custom types:

```gal
trait Try {
    type Output;
    type Residual;
    
    fn from_output(output: Self::Output) -> Self;
    fn branch(self) -> ControlFlow<Self::Residual, Self::Output>;
}

enum ControlFlow<B, C> {
    Continue(C),
    Break(B),
}
```

### Zero-Cost Abstractions

The `?` operator compiles to efficient code equivalent to explicit match statements:

```gal
// This code with ?:
let value = operation()?;

// Is equivalent to:
let value = match operation() {
    Ok(val) => val,
    Err(e) => return Err(e.into()),
};
```

## Best Practices

1. **Use ? for Error Propagation**: Prefer `?` over explicit `match` for simple error propagation
2. **Implement From Traits**: Create `From` implementations for common error conversions
3. **Design for ?**: When designing APIs, consider how `?` will be used
4. **Document Error Types**: Clearly document what errors your functions can return
5. **Chain Operations**: Use `?` to create clean operation chains

## Examples

### File Processing Pipeline

```gal
actor FileProcessor {
    on ProcessFiles(paths: Vec<String>) -> Result<Vec<ProcessedFile>, ProcessingError> {
        let mut results = Vec::new();
        
        for path in paths {
            let content = fs::read_to_string(&path)?;
            let parsed = parse_file_content(content)?;
            let validated = validate_content(parsed)?;
            let processed = transform_content(validated)?;
            
            results.push(ProcessedFile {
                path,
                content: processed,
            });
        }
        
        Ok(results)
    }
}
```

### HTTP Client with Error Handling

```gal
actor HttpClient {
    state base_url: String
    
    on Get(endpoint: String) -> Result<Response, HttpError> {
        let url = self.build_url(endpoint)?;
        let request = Request::new(url)?;
        let response = self.send_request(request)?;
        self.validate_response(response)?;
        Ok(response)
    }
}
```

The `?` operator makes error handling in GAL both safe and ergonomic, enabling clean code while maintaining the strong type safety guarantees of the language.