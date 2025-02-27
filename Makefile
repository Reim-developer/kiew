# All sample command, and error handling test

# Cargo command. Remove "@" from CARGO to show full command when debugging
CARGO = @cargo run --

.PHONY: all httpbin example json xml

####################### GET COMMAND TEST
# All
default: httpbin # Default

# With customize your headers
httpbin:
	$(CARGO) get -w "https://httpbin.org/headers" -H "User-Agent: KiewCLI" -H "Accept: application/json" -H "Custom-Header: This is a test message!"

# Sample.
example:
	$(CARGO) get -w "https://example.com" -H "User-Agent: KiewCLI"

# Plain text test
text:
	$(CARGO) get -w "https://plaintextoffenders.com"

# JSON output test
json:
	$(CARGO) get -w "https://httpbin.org/headers"

# XML output test
xml:
	$(CARGO) get -w "https://www.xml-sitemaps.com/sitemap.xml"

# Error handling test
err:

	$(CARGO) get -w "https://nonexistweb.nonexistweb" -H "JustSimple: Error"
	
# Test in case debug mode is enable
debug:
	$(CARGO) get -w "https://httpbin.org/headers" -H "User-Agent: KiewCLI" -H "Accept: application/json" -H "Custom-Header: This is a test message!" --debug

###################################################### 
