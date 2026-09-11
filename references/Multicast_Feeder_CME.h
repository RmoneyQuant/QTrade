/*
 * OrderRequestHandler.h
 *
 *  Created on: 28-May-2016
 *      Author: navneet-ubantu
 */

#ifndef MULTICAST_FEEDER_CME_H_
#define MULTICAST_FEEDER_CME_H_

#include <iostream>
#include <fstream>
#include <thread>
#include <sstream>
#include <stdint.h>
#include <sys/timerfd.h>
#include <sys/time.h>
#include <sys/resource.h>
#include <syscall.h> // for SYS_gettid
#include <unistd.h> //for syscall
#include <sched.h> //for cpu_set
#include <random>
#include <functional>
#include <errno.h>
#include <tr1/unordered_map>

using namespace std;

namespace CME{

	class OrderRequestHandler {

	private:
		FILE *feedLog;
		FILE *feedCapture;

		static void *Stream_Processing_Thread_Func(void *arg);
		pthread_t 				PrimaryStreamPThreadID;
	public:
		int32_t StartPrimaryFeed();
		int32_t StopPrimaryFeed(){return 0;}

		uint32_t 	Primary_Multicast_Core;
		int8_t	 	DataLine;
		OrderRequestHandler();

		void receiveMarketData(void *md);

	};
}
#endif /* CME_MULTICAST_FEEDER_CME_H_ */
