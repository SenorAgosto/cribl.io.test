default_host="localhost"
default_port="8888"

def log_uri(log_name, num_lines, host=default_host, port=default_port):
	""" return URI for getting log data
	"""
	return f"http://{host}:{port}/log/{log_name}/{num_lines}"

def filtered_log_uri(log_name, keyword, num_lines, host=default_host, port=default_port):
	""" return URI for getting filtered_log data
	"""
	return f"http://{host}:{port}/filtered_log/{log_name}/{keyword}/{num_lines}"

