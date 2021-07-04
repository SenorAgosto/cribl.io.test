import log_collection as log
import requests

def before_scenario(context, scenario):
	print("performing setup")


@given(u'log_collection is running')
def cargo_run(context):
	pass

@given(u'expected result')
def capture_expected_result(context):
	context.expected_result = context.text

@when(u'we request {num_lines} lines from {log_file} with filter keyword {keyword}')
def request_filtered_log(context, num_lines, log_file, keyword):
	url = log.filtered_log_uri(log_file, keyword, num_lines)
	response = requests.get(url)

	context.status_code = response.status_code
	context.result = response.text.strip()

@when(u'we request {num_lines} lines from {log_file}')
def request_log(context, num_lines, log_file):
	url = log.log_uri(log_file, num_lines)
	response = requests.get(url)

	context.status_code = response.status_code
	context.result = response.text.strip()

@then(u'we get the expected result')
def assert_result(context):
	assert context.status_code == 200
	assert context.expected_result == context.result

@then(u'we get a server error')
def assert_error(context):
	assert context.status_code == 500

