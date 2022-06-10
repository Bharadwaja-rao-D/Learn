#include <iostream>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <chrono>
#define MAX_SIZE 10

typedef struct Shared{
	std::mutex lock;
	std::condition_variable cv;
	int buffer[MAX_SIZE];
	int in = 0;
	int out = 0;
}Shared;

class Channel{
	Shared inner;
	public: 
	void producer(int item){
		//if (last_full+1) % MAX_SIZE == next_free => full

		std::unique_lock<std::mutex> lock(inner.lock);
		//cv will acquire lock internally
		inner.cv.wait(
				lock, [&] { return (inner.in + 1) % MAX_SIZE != inner.out; });
		inner.buffer[inner.in] = item;
		inner.in = (inner.in + 1) % MAX_SIZE;
		lock.unlock();
		inner.cv.notify_one();
	}

};

