// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#pragma once
#include <iostream>

struct Dinosaur;
class MyClass {
public:
  MyClass(Dinosaur * rustPtr);
  Dinosaur * rustPtr;
  int myFunction(int x, int y);
};

MyClass *MyClass_new(Dinosaur * rustPtr );
void MyClass_delete(MyClass *myClass  ) ;
int MyClass_myFunction(MyClass *myClass, int x, int y) ;
void MyClass_roar(MyClass *myClass);
void MyClass_Point(MyClass *myClass);
