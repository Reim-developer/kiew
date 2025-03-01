# All sample command, and error handling test

# Cargo command. Remove "@" from CARGO to show full command when debugging
CARGO = @cargo run --
HTTPBIN_GET = "https://httpbin.org/headers"
HTTPBIN_POST = "https://httpbin.org/post"

.PHONY: get_default get_example get_text get_json get_xml get_err get_debug test post_default post_error post_long

####################### GET COMMAND TEST
# All
get_default: get_httpbin # Default

# With customize your headers
get_httpbin:
	$(CARGO) get -w $(HTTPBIN_GET) -H "User-Agent: KiewCLI" -H "Accept: application/json" -H "Custom-Header: This is a test message!"

# Sample.
get_example:
	$(CARGO) get -w "https://example.com" -H "User-Agent: KiewCLI"

# Plain text test
get_text:
	$(CARGO) get -w "https://plaintextoffenders.com"

# JSON output test
get_json:
	$(CARGO) get -w "https://httpbin.org/headers"

# XML output test
get_xml:
	$(CARGO) get -w "https://www.xml-sitemaps.com/sitemap.xml"

# Error handling test
get_err:

	$(CARGO) get -w "https://nonexistweb.nonexistweb" -H "JustSimple: Error"
	
# Test in case debug mode is enable
get_debug:
	$(CARGO) get -w "https://httpbin.org/headers" -H "User-Agent: KiewCLI" -H "Accept: application/json" -H "Custom-Header: This is a test message!" --debug

###################################################### 

# FOR TEST:
test:
	cargo test

####################

####################### POST REQUEST

# Default POST request
post_default:
	$(CARGO) post -w $(HTTPBIN_POST) -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"key": "value"}'

# POST request error
post_error:
	$(CARGO) post -w $(HTTPBIN_POST)  -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"key": "value}'

# POST request but body is long
post_long:
	cargo run -- post -w $(HTTPBIN_POST) -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"user":{"id":12345,"name":"Nguyen Van A","email":"nguyenvana@example.com","age":30},"order":{"order_id":"ABC123XYZ","items":[{"product":"Laptop Dell XPS 13","price":1500.99,"quantity":1},{"product":"Mouse Logitech MX Master","price":99.50,"quantity":2}],"total":1699.99,"status":"pending"},"timestamp":"2025-03-01T12:34:56Z","metadata":{"source":"web","campaign":"spring_sale_2025"}}'

# POST request with long body and enable debug mode
post_debug:
	cargo run -- post -w $(HTTPBIN_POST) -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"user":{"id":12345,"name":"Nguyen Van A","email":"nguyenvana@example.com","age":30},"order":{"order_id":"ABC123XYZ","items":[{"product":"Laptop Dell XPS 13","price":1500.99,"quantity":1},{"product":"Mouse Logitech MX Master","price":99.50,"quantity":2}],"total":1699.99,"status":"pending"},"timestamp":"2025-03-01T12:34:56Z","metadata":{"source":"web","campaign":"spring_sale_2025"}}' --debug
