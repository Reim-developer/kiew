# All sample command, and error handling test

# Cargo command. Remove "@" from CARGO to show full command when debugging
CARGO = @cargo run --
HTTPBIN_GET = "https://httpbin.org/headers"
HTTPBIN_POST = "https://httpbin.org/post"
HTTPBIN_PUT =  "https://httpbin.org/put"
HTTPBIN_DEL = "https://httpbin.org/delete"

.PHONY: get_default get_example get_text get_json get_xml get_err get_debug test post_default post_error post_long

# Formatter project
fmt:
	@cargo fmt

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

# Test in case user wants get details 
# request GET information instead of response body
get_info:
	$(CARGO) get -I -w "https://httpbin.org/headers"

# With --debug option:
get_info_debug:
	$(CARGO) get -I -w "https://httpbin.org/headers" --debug

# With customize headers:
get_info_headers:
	$(CARGO) get -I -w "https://httpbin.org/headers" -H "User-Agent: KiewCLI" -H "Accept: application/json"

# With read setting from TOML setting file
get_with_setting:
	$(CARGO) get -S MyCustomSetting.json

# Test error handling
get_with_err:
	$(CARGO) get -S error --web now

###################################################### 

# FOR TEST:
test:
	@cargo test

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
	$(CARGO) post -w $(HTTPBIN_POST) -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"user":{"id":12345,"name":"Nguyen Van A","email":"nguyenvana@example.com","age":30},"order":{"order_id":"ABC123XYZ","items":[{"product":"Laptop Dell XPS 13","price":1500.99,"quantity":1},{"product":"Mouse Logitech MX Master","price":99.50,"quantity":2}],"total":1699.99,"status":"pending"},"timestamp":"2025-03-01T12:34:56Z","metadata":{"source":"web","campaign":"spring_sale_2025"}}'

# POST request with long body and enable debug mode
post_debug:
	$(CARGO) post -w $(HTTPBIN_POST) -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"user":{"id":12345,"name":"Nguyen Van A","email":"nguyenvana@example.com","age":30},"order":{"order_id":"ABC123XYZ","items":[{"product":"Laptop Dell XPS 13","price":1500.99,"quantity":1},{"product":"Mouse Logitech MX Master","price":99.50,"quantity":2}],"total":1699.99,"status":"pending"},"timestamp":"2025-03-01T12:34:56Z","metadata":{"source":"web","campaign":"spring_sale_2025"}}' --debug

#################################

####################### PUT REQUEST
# Default PUT request
put_default:
	$(CARGO) put -w $(HTTPBIN_PUT) -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"key": "value"}'

# PUT request error
put_error:
	$(CARGO) put -w $(HTTPBIN_PUT)  -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"key": "value}'

# PUT request but body is long
put_long:
	$(CARGO) put -w $(HTTPBIN_PUT) -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"user":{"id":12345,"name":"Nguyen Van A","email":"nguyenvana@example.com","age":30},"order":{"order_id":"ABC123XYZ","items":[{"product":"Laptop Dell XPS 13","price":1500.99,"quantity":1},{"product":"Mouse Logitech MX Master","price":99.50,"quantity":2}],"total":1699.99,"status":"pending"},"timestamp":"2025-03-01T12:34:56Z","metadata":{"source":"web","campaign":"spring_sale_2025"}}'

# PUT request with long body and enable debug mode
put_debug:
	$(CARGO) put -w $(HTTPBIN_PUT) -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"user":{"id":12345,"name":"Nguyen Van A","email":"nguyenvana@example.com","age":30},"order":{"order_id":"ABC123XYZ","items":[{"product":"Laptop Dell XPS 13","price":1500.99,"quantity":1},{"product":"Mouse Logitech MX Master","price":99.50,"quantity":2}],"total":1699.99,"status":"pending"},"timestamp":"2025-03-01T12:34:56Z","metadata":{"source":"web","campaign":"spring_sale_2025"}}' --debug

####################################

####################### DELETE REQUEST
# Default PUT request
delete_default:
	$(CARGO) delete -w $(HTTPBIN_DEL) -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"key": "value"}'

# PUT request error
delete_error:
	$(CARGO) delete -w $(HTTPBIN_DEL)  -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"key": "value}'

# PUT request but body is long
delete_long:
	$(CARGO) delete -w $(HTTPBIN_DEL) -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"user":{"id":12345,"name":"Nguyen Van A","email":"nguyenvana@example.com","age":30},"order":{"order_id":"ABC123XYZ","items":[{"product":"Laptop Dell XPS 13","price":1500.99,"quantity":1},{"product":"Mouse Logitech MX Master","price":99.50,"quantity":2}],"total":1699.99,"status":"pending"},"timestamp":"2025-03-01T12:34:56Z","metadata":{"source":"web","campaign":"spring_sale_2025"}}'

# PUT request with long body and enable debug mode
delete_debug:
	$(CARGO) delete -w $(HTTPBIN_DEL) -H "User-Agent: KiewCLI" -H "Accept: application/json" -P '{"user":{"id":12345,"name":"Nguyen Van A","email":"nguyenvana@example.com","age":30},"order":{"order_id":"ABC123XYZ","items":[{"product":"Laptop Dell XPS 13","price":1500.99,"quantity":1},{"product":"Mouse Logitech MX Master","price":99.50,"quantity":2}],"total":1699.99,"status":"pending"},"timestamp":"2025-03-01T12:34:56Z","metadata":{"source":"web","campaign":"spring_sale_2025"}}' --debug

# Setting default
setting_default:
	$(CARGO) setting

# Setting with customize setting file name
setting_custom:
	$(CARGO) setting -F "MyCustomSetting"
