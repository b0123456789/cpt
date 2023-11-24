
#include "myclass.h"
#include "cxxgen.h"
#include<string.h>

MyClass:: MyClass(Dinosaur * rustPtr):rustPtr(rustPtr){}

int MyClass::myFunction(int x, int y) { 
    return x + y;
}

void MyClass_roar(MyClass *myClass){
    myClass->rustPtr->roar();
}

void MyClass_Point(MyClass *myClass){
    rust::Box<Value> val = new_value(false,1,1.4,"strs");
    print_value(std::move(val));

    rust::Box<Value> val0 = new_value(false,2,3.4,"strs");
    myClass->rustPtr->eat(std::move(val0));
}

MyClass *MyClass_new(Dinosaur * rustPtr) {
     MyClass * cls=  new MyClass(rustPtr);
     std::cout << "MyClass_new:" << cls <<std::endl;
 
     return cls;
}

void MyClass_delete(MyClass *myClass  ) {
    std::cout << "MyClass_delete:" << myClass << std::endl;
    delete myClass;
}

int MyClass_myFunction(MyClass *myClass, int x, int y) {
    return myClass->myFunction(x, y);
}