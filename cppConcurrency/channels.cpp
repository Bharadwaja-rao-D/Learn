#include <iostream>
#include <thread>
#include <mutex>
#include <condition_variable>
#include <chrono>
#define MAX_SIZE 10

class Shared {
	std::mutex m;
	std::condition_variable cv;
	int buffer[MAX_SIZE];
	int in = 0;
	int out = 0;
	public: 
	void send(int item){
		//if (last_full+1) % MAX_SIZE == next_free => full

		std::unique_lock<std::mutex> lock(m);
		//cv will acquire lock internally
		cv.wait( lock, [&] { return (in + 1) % MAX_SIZE != out; });
		buffer[in] = item;
		in = (in + 1) % MAX_SIZE;
		lock.unlock();
		cv.notify_one();
	}

	int recv(){
		//if (last_full) % MAX_SIZE == next_free => free

		std::unique_lock<std::mutex> lock(m);
		//cv will acquire lock internally
		cv.wait(lock, [&] { return (out) != in; });
		int item = buffer[out];
		out = (out + 1) % MAX_SIZE;
		lock.unlock();
		cv.notify_one();

		return item;
	}
};

void stimulate_pro(Shared* inner){
	for(int i=0; i<30;  i++){
		inner->send( i);
		std::this_thread::sleep_for(std::chrono::milliseconds(100));
	}
}

void stimulate_cons(Shared* inner){
	for(int i=0; i<30;  i++){
		int item = inner->recv();
		std::cout << item << std::endl;
		std::this_thread::sleep_for(std::chrono::milliseconds(100));
	}
}

int main(){
	Shared* inner = new Shared;
	std::thread pro = std::thread(stimulate_pro, inner);
	std::thread cons = std::thread(stimulate_cons, inner);

	pro.join();
	cons.join();
}
