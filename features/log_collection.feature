Feature: log_collection http microservice

	Scenario: retrieve 10 lines from weekly.out via log_collection microservice
		Given log_collection is running
		Given expected result:
			"""
            Fri Jul  2 15:15:23 CDT 2021
            Sat Jun 19 07:16:59 CDT 2021
            Thu Jun 10 18:00:12 CDT 2021
            Fri Jun  4 21:28:03 CDT 2021
            Fri May 28 16:25:55 CDT 2021
            Mon May 24 13:53:25 CDT 2021
            Thu May 20 18:34:27 CDT 2021
            Thu May 13 19:47:58 CDT 2021
            Tue May  4 17:51:54 CDT 2021
            Tue Apr 27 17:56:34 CDT 2021
			"""
		When we request 10 lines from weekly.out 
		Then we get the expected result

	Scenario: retrieve May entries from weekly.out log via log_collection 
		Given log_collection is running
		Given expected result:
			"""
            Fri May 28 16:25:55 CDT 2021
            Mon May 24 13:53:25 CDT 2021
            Thu May 20 18:34:27 CDT 2021
            Thu May 13 19:47:58 CDT 2021
            Tue May  4 17:51:54 CDT 2021
			"""
		When we request 10 lines from weekly.out with filter keyword May
		Then we get the expected result

	Scenario: retrieve non-existant file from log_collection microservice
		Given log_collection is running
		When we request 10 lines from non_existant.txt
		Then we get a server error

	Scenario: retrive non-existant file from log_collection microservice with filtering
		Given log_collection is running
		When we request 10 lines from non_existant.txt with filter keyword May
		Then we get a server error

