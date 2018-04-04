#include <iostream>

using namespace std;

const void doExchange(int *list,int index){
    int temp = list[index+1];
    list[index+1] = list[index];
    list[index] = temp;
}

void exchange(int *list, int length, int index=0, bool flag=true){
    bool isExchange = list[index]>list[index+1];
    if(isExchange){
        doExchange(list,index);
        flag = false;
    }
    if(index + 1 < length){
        exchange(list,length,index+1,flag);
    }else if(flag) return;
    else{
        exchange(list,length);
    }
}

void printSorted(int *list,int length){
    for(int i=0;i<length;i++) cout<<list[i]<<" ";
}

int main(){
    int a[10] = {2,4,32,45,3,1,5,9,6,7};
    exchange(a,10);
    printSorted(a,10);
    return 0;
}
