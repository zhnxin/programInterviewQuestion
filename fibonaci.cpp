#include <iostream>
#include<stdlib.h>
using namespace std;

int next(int index){
    if(index<3) return 1;
    else{
        return next(index-1)+next(index-2);
    }
}

int getNum(int index){
    if(index<1) throw "the index is out of range";
    return next(index);
}

int main(){
    int index;
    cout<<"index:";
    cin>>index;
    cout<<endl;
    cout<<getNum(index);
    return 0;
}
