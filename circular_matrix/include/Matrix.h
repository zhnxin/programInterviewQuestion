#ifndef MATRIX_H
#define MATRIX_H


class Matrix
{
    public:
        Matrix(int);
        virtual ~Matrix();
        int ** getMatrix();
        int getOrder();
        int get(int,int);

    protected:

    private:
        int **m_matrix;
        int m_order;

        void generateNext(int &,int &,int);

        int getLoop(int,int);
        const int getListMin(int *,int);
};


#endif // MATRIX_H
