#!/bin/python
import sys

class Excutor:
    def __init__(self,one_place,max_place_cout=10):
        self.one_place = one_place
        self.max_place_cout = max_place_cout
        self.num=[]
        self.count=0
    
    def generate(self):
        self.__next(self.one_place)
    
    def __next(self,num):
        self.count +=1
        remainder = num%2
        quotient = int(num/2)
        self.num.append(str(quotient))
        if not (remainder==0 and quotient == self.one_place) and self.count < self.max_place_cout:
            self.__next(remainder*10+quotient)

    def get_num(self):
        if self.count < self.max_place_cout:
            return "".join(self.num)
        else:
            return False

def main():
    for i in (3,64):
        for j in (2,9):
            excutor = Excutor(j,i)
            excutor.generate()
            if excutor.get_num():                
                print(excutor.get_num())

if __name__ == '__main__':
    main()