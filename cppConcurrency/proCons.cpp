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

void producer(Shared* shared, int item){
	//if (last_full+1) % MAX_SIZE == next_free => full

	std::unique_lock<std::mutex> lock(shared->lock);
	//cv will acquire lock internally
	shared->cv.wait(
			lock, [&] { return (shared->in + 1) % MAX_SIZE != shared->out; });
	shared->buffer[shared->in] = item;
	shared->in = (shared->in + 1) % MAX_SIZE;
	lock.unlock();
	shared->cv.notify_one();
}

void consumer(Shared* shared){
	//if (last_full) % MAX_SIZE == next_free => free

	std::unique_lock<std::mutex> lock(shared->lock);
	//cv will acquire lock internally
	shared->cv.wait(lock, [&] { return (shared->out) != shared->in; });
	int item = shared->buffer[shared->out];
	shared->out = (shared->out + 1) % MAX_SIZE;
	lock.unlock();
	shared->cv.notify_one();

	std::cout << item << std::endl;
}

void stimulate_pro(Shared* inner){
	for(int i=0; i<30;  i++){
		producer(inner, i);
		std::this_thread::sleep_for(std::chrono::milliseconds(100));
	}
}

void stimulate_cons(Shared* inner){
	for(int i=0; i<30;  i++){
		consumer(inner);
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
