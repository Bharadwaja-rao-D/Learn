/*Implimentation similar to the rust channels*/


template<typename T>
class Sender{
	public:
		void send(T value){}
};

template<typename T>
class Reciever{
	public: 
		T recv(){}
};

template<typename T>
struct Channel{
	Sender<T> Sender;
	Reciever<T> Reciever;
};

template<typename T>
Channel<T> channel(const unsigned size){
	return Channel<T>{};
}
