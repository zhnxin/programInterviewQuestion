#include "Matrix.h"
#include <iostream>
#include <iomanip>
#include <cmath>

using namespace std;

void printMatrix(Matrix matrix){
    int order = matrix.getOrder();
    int strWidth = int(log10(double(order*order)))+1;

    for(int i=0;i< order;i++){
        for(int j=0;j<order;j++){
            cout<<setw(strWidth)<<matrix.get(i,j)<<" ";
        }
        cout<<endl;
    }

}

int main()
{
    int order;
    cin>>order;
    Matrix matrix(order);
    printMatrix(matrix);
    return 0;
}
