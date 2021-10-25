# Simple calculator written in Rust

## Installation:

```
cargo install --path . --features bin
```

## Usage:

```
1+1
>> 2
```

You can also use variables:

```
a = 1+1
b = a + 1
b
>> 3
```

## Library api

```
let calculator = Calculator::new();
match calculator.exec("1+1") {
    Ok(Some(result)) => println!("The answer was {}", result),
    Err(e) => eprintln!("An error occured: {}", e),
    _ => {}, // an assignment (eg a = 1+1) will not yield any result
}
```