# Tampering HTTP

`HTTP Verb Tampering` it is a way to test the web application's response to different HTTP methods.

This tool is still in its early stages of development, but it helps check the HTTP Verbs available for a given endpoint.

You can import results from tools like Feroxbuster, Goburster or any list of endpoints you have

## Features

- Reads URLs from a specified file.
- Applies tampering operations on each URL.
- Writes the results to an output file or displays them in the console.

## Usage

### Arguments

- `-i, --input <INPUT>`: Specifies the input file containing the URLs to process.
- `-o, --output <OUTPUT>`: (Optional) Specifies the output file to write the results.

### Examples

#### Process URLs and write the results to a file

```bash
tampering_http --input urls.txt --output results.txt
```

#### Process URLs and write the results to a file
```bash
tampering_http --input urls.txt
```

## License

This project is licensed under the MIT License. See the LICENSE file for more details.


## Contributions
Contributions are welcome! Feel free to open issues or pull requests.