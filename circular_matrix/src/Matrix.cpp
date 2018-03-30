#include "Matrix.h"
#include <iostream>

Matrix::Matrix(int order)
{
    //ctor
    this->m_order = order;
    this->m_matrix = new int*[order];
    for(int i=0;i<order;i++){
        this->m_matrix[i] = new int[order];
    }
    int i=0;int j=0;
    int lastNum = order*order;
    for(int num=1;num<=lastNum;num++){
        generateNext(i,j,num);
    }
}

Matrix::~Matrix()
{
    //dtor
    for(int i=0;i<this->m_order;i++) delete [] this->m_matrix[i];
}

int ** Matrix::getMatrix(){
    return this->m_matrix;
}

int Matrix::getOrder(){
    return this->m_order;
}
int Matrix::get(int i,int j){
    return this->m_matrix[i][j];
}
void Matrix::generateNext(int &i,int &j,int num){
    this->m_matrix[i][j] = num;
    int loop = this->getLoop(i,j);
    if(j>=i && i<this->m_order-loop-1){
        if(j<this->m_order-loop-1) j++;
        else i++;
    }else{
        if(j>loop) j--;
        else if(i> loop+1) {
                i--;
        }
        else {
            j++;
        }
    }
}

int Matrix::getLoop(int i,int j){
    int a[] ={i,j,this->m_order-i-1,this->m_order-j-1};
    return getListMin(a,4);

}
const int Matrix::getListMin(int *a,int length){
    int temp = a[0];
    for(int i=1;i<length;i++)
        if(temp > a[i]) temp = a[i];
    return temp;
}


