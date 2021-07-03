@given(u'log_collection is running')
def cargo_run(context):
	pass

@given(u'expected result')
def capture_expected_result(context):
	context.expected_result = context.text

@when(u'we request {num_lines} lines from {log_file} with filter keyword {keyword}')
def request_filtered_log(context, num_lines, log_file, keyword):
	pass

@when(u'we request {num_lines} lines from {log_file}')
def request_log(context, num_lines, log_file):
	pass

@then(u'we get the expected result')
def assert_result(context):
	result = ""
	assert context.expected_result == result 

@then(u'we get a server error')
def assert_error(context):
	pass

