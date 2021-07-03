Feature: log_collection http microservice

	Scenario: retrieve 10 lines from weekly.out via log_collection microservice
		Given log_collection is running
		Given expected result:
			"""
			"""
		When we request 10 lines from weekly.out 
		Then we get the expected result

	Scenario: retrieve May entries from weekly.out log via log_collection 
		Given log_collection is running
		Given expected result:
			"""
			"""
		When we request 10 lines from weekly.out with filter keyword May
		Then we get the expected result

	Scenario: retrieve non-existant file from log_collection microservice
		Given log_collection is running
		When we request 10 lines from non_existant.txt
		Then we get a server error
