import os
import signal
import subprocess
import time

from behave import fixture
from behave.fixture import use_fixture
 
def stop_microservice(context):
	os.killpg(context.service.pid, signal.SIGINT)

@fixture
def start_microservice(context, timeout=30, **kwargs):

	print("Starting log_collection microservice")

	context.service = subprocess.Popen("cargo run -- _data", stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True, preexec_fn=os.setsid)
	time.sleep(.5)

	yield context.service

	stop_microservice(context)
	time.sleep(.5)

def before_tag(context, tag):
	if tag == "fixture.start.log_collection":
		return use_fixture(start_microservice, context, timeout=30)

